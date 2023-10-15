use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use action::action_core::{ActionConfig, ActionCore, ActionMessage};
use consensus::core::ConsensusCore;
use ethers_providers::{Middleware, PubsubClient};
use futures::{Future, FutureExt};
use futures_util::StreamExt;
use guard_network::{GaurdStakingEvent, NetworkConfig, PeerMessages, Swarm, SwarmEvent};
use guard_types::on_chain::{RawLvrSettlement, RawUserSettlement};
use sim::Simulator;
use tokio::sync::mpsc::{Sender, UnboundedSender};
use tracing::{debug, warn};

use crate::submission_server::{
    Submission, SubmissionServer, SubmissionServerConfig, SubmissionServerInner, SubscriptionKind,
    SubscriptionResult
};

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M, S>
where
    M: Middleware + Unpin + 'static,
    S: Simulator + Unpin + 'static,
    <M as Middleware>::Provider: PubsubClient
{
    /// guard network connection
    network:              Swarm,
    consensus:            ConsensusCore<S>,
    /// deals with all action related requests and actions including bundle
    /// building
    action:               ActionCore<M, S>,
    /// deals with new submissions through a rpc to the network
    server:               SubmissionServer,
    /// TODO: remove this terrorism joe added
    valid_stakers_tx:     UnboundedSender<GaurdStakingEvent>,
    /// make sure we keep subscribers upto date
    server_subscriptions: HashMap<SubscriptionKind, Vec<Sender<SubscriptionResult>>>
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Guard<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(
        network_config: NetworkConfig,
        action_config: ActionConfig<M, S>,
        server_config: SubmissionServerConfig
    ) -> anyhow::Result<Self> {
        let (valid_stakers_tx, valid_stakers_rx) = tokio::sync::mpsc::unbounded_channel();
        let sub_server = SubmissionServerInner::new(server_config).await?;
        let swarm = Swarm::new(network_config, valid_stakers_rx).await?;
        let action = ActionCore::new(action_config).await?;
        let consensus = ConsensusCore::new().await;

        Ok(Self {
            action,
            server_subscriptions: HashMap::default(),
            consensus,
            server: sub_server,
            valid_stakers_tx,
            network: swarm
        })
    }

    fn handle_submissions(&mut self, msgs: Vec<Submission>) {
        debug!(amount = msgs.len(), "handling new submissions");

        let (user_subs, arb_subs): (Vec<Option<RawUserSettlement>>, Vec<Option<RawLvrSettlement>>) =
            msgs.into_iter()
                .filter_map(|msg| match msg {
                    Submission::Subscription(kind, sender) => {
                        self.server_subscriptions
                            .entry(kind)
                            .or_default()
                            .push(sender);
                        None
                    }

                    Submission::UserTx(data) => Some((Some(data), None)),
                    Submission::ArbTx(data) => Some((None, Some(data)))
                })
                .unzip();

        self.action
            .get_cow()
            .new_user_transactions(user_subs.into_iter().filter_map(|f| f).collect());

        self.action
            .get_cow()
            .new_searcher_transactions(arb_subs.into_iter().filter_map(|f| f).collect());
    }

    fn handle_network_events(&mut self, network_events: Vec<SwarmEvent>) {
        network_events.into_iter().for_each(|event| match event {
            SwarmEvent::ValidMessage { peer_id, request } => {
                debug!(?peer_id, ?request, "got data from peer");
                match request {
                    PeerMessages::PropagateBundle(bundle) => {
                        self.action.get_cow().new_bundle((*bundle).clone().into())
                    }
                    PeerMessages::PropagateSearcherTransactions(new_txes) => {
                        self.action.get_cow().new_searcher_transactions(
                            (*new_txes).clone().into_iter().map(Into::into).collect()
                        );
                    }
                    PeerMessages::PropagateUserTransactions(new_txes) => {
                        self.action.get_cow().new_user_transactions(
                            (*new_txes).clone().into_iter().map(Into::into).collect()
                        );
                    }
                    PeerMessages::NewBlock(b) => {}
                    PeerMessages::BundleVote(vote) => {}
                    PeerMessages::Bundle23Vote(vote) => {}
                    PeerMessages::LeaderProposal(prop) => {}
                    PeerMessages::SignedLeaderProposal(signed_prop) => {}
                    PeerMessages::NewState(state) => {}
                }
            }
            res @ _ => {
                debug!(?res, "got swarm event");
            }
        });
    }

    fn handle_action_messages(&mut self, action_events: Vec<ActionMessage>) {
        debug!(?action_events, "got actions");
        action_events.into_iter().for_each(|event| match event {
            ActionMessage::NewBestBundle(bundle) => {
                self.network
                    .propagate_msg(PeerMessages::PropagateBundle(bundle.clone()));
                // submit to subscribers
                if let Some(senders) = self
                    .server_subscriptions
                    .get_mut(&SubscriptionKind::BestBundles)
                {
                    senders.retain(|sender| {
                        sender
                            .try_send(SubscriptionResult::Bundle(bundle.clone()))
                            .is_ok()
                    });
                }
            }
            ActionMessage::NewValidUserTransactions(transactions) => {
                // prop to other peers
                self.network
                    .propagate_msg(PeerMessages::PropagateUserTransactions(transactions.clone()));
                // submit to subscribers
                if let Some(senders) = self
                    .server_subscriptions
                    .get_mut(&SubscriptionKind::CowTransactions)
                {
                    senders.retain(|sender| {
                        sender
                            .try_send(SubscriptionResult::CowTransaction(Arc::new(
                                (*transactions)
                                    .clone()
                                    .into_iter()
                                    .map(|tx| tx.raw.order)
                                    .collect()
                            )))
                            .is_ok()
                    });
                }
            }
            ActionMessage::NewValidSearcherTransactions(transactions) => self
                .network
                .propagate_msg(PeerMessages::PropagateSearcherTransactions(transactions))
        });
    }
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Future for Guard<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut work = 4096;
        loop {
            let mut sub_server_msg = Vec::with_capacity(3);
            while let Poll::Ready(new_msg) = self.server.poll_next_unpin(cx) {
                let Some(new_msg) = new_msg else { return Poll::Ready(()) };

                sub_server_msg.push(new_msg)
            }
            self.handle_submissions(sub_server_msg);

            let mut swarm_msgs = Vec::with_capacity(3);
            while let Poll::Ready(new_msg) = self.network.poll_next_unpin(cx) {
                let Some(new_msg) = new_msg else { return Poll::Ready(()) };
                swarm_msgs.push(new_msg);
            }
            self.handle_network_events(swarm_msgs);

            if let Poll::Ready(msg) = self.action.poll(cx) {
                self.handle_action_messages(msg);
            }

            work -= 1;
            if work == 0 {
                cx.waker().wake_by_ref();
                return Poll::Pending
            }
        }
    }
}

#[cfg(feature = "test_harness")]
pub mod test_harness {
    use std::net::SocketAddr;

    use ethers_core::types::{H256, U64};
    use reth_primitives::PeerId;
    use tokio::{
        sync::{
            mpsc::{channel, Receiver},
            oneshot::{channel as one_channel, Sender as OneSender}
        },
        task::JoinHandle
    };

    use crate::guard::*;

    pub enum GuardCheatCodes {
        /// returns the address the peer is at
        PeerAddr(OneSender<SocketAddr>),
        /// makes the given peer id the leader
        MakeLeader(PeerId, U64),
        /// makes this node propagate there best bundle
        PropagateBundle,
        /// makes this node propagate all user txes it has
        PropagateUserTransactions,
        /// make the node propagate all searcher txes it has
        PropagateSearcherTransactions,
        /// checks to see if the guard contains a bundle we sent
        CheckForBundle(H256, OneSender<bool>),
        /// checks to see if the guard has a user transaction
        CheckForUserTx(H256, OneSender<bool>),
        /// check to see if the guard has a searcher tx
        CheckForSearcherTx(H256, OneSender<bool>)
    }

    pub struct GuardHandle {
        sender:        Sender<GuardCheatCodes>,
        _guard_handle: JoinHandle<()>
    }

    impl GuardHandle {
        pub async fn new<M, S>(
            network_config: NetworkConfig,
            leader_config: ActionConfig<M, S>,
            server_config: SubmissionServerConfig
        ) -> anyhow::Result<Self>
        where
            M: Middleware + Unpin + 'static,
            S: Simulator + Unpin + 'static,
            <M as Middleware>::Provider: PubsubClient
        {
            let (tx, rx) = channel(10);
            let guard = Guard::new(network_config, leader_config, server_config).await?;
            let guard_wrapper = GuardWrapper { guard, receiver: rx };
            let handle = tokio::spawn(guard_wrapper);

            Ok(Self { sender: tx, _guard_handle: handle })
        }
    }

    impl GuardHandle {
        pub async fn get_addr(&self) -> SocketAddr {
            let (tx, rx) = one_channel();
            let _ = self.sender.send(GuardCheatCodes::PeerAddr(tx)).await;

            rx.await.unwrap()
        }

        pub async fn make_leader(&self, id: PeerId, block: U64) {
            let _ = self
                .sender
                .send(GuardCheatCodes::MakeLeader(id, block))
                .await;
        }

        pub async fn check_for_bundle(&self, hash: H256) -> bool {
            let (tx, rx) = one_channel();
            let _ = self
                .sender
                .send(GuardCheatCodes::CheckForBundle(hash, tx))
                .await;

            rx.await.unwrap()
        }

        pub async fn check_for_user_tx(&self, hash: H256) -> bool {
            let (tx, rx) = one_channel();
            let _ = self
                .sender
                .send(GuardCheatCodes::CheckForUserTx(hash, tx))
                .await;

            rx.await.unwrap()
        }

        pub async fn check_for_searcher_tx(&self, hash: H256) -> bool {
            let (tx, rx) = one_channel();
            let _ = self
                .sender
                .send(GuardCheatCodes::CheckForSearcherTx(hash, tx))
                .await;

            rx.await.unwrap()
        }

        pub async fn propagate_bundle(&self) {
            let _ = self.sender.send(GuardCheatCodes::PropagateBundle).await;
        }

        pub async fn propagate_user_transaction(&self) {
            let _ = self
                .sender
                .send(GuardCheatCodes::PropagateUserTransactions)
                .await;
        }

        pub async fn propagate_searcher_transaction(&self) {
            let _ = self
                .sender
                .send(GuardCheatCodes::PropagateSearcherTransactions)
                .await;
        }
    }

    pub struct GuardWrapper<M, S>
    where
        M: Middleware + Unpin + 'static,
        S: Simulator + Unpin + 'static,
        <M as Middleware>::Provider: PubsubClient
    {
        receiver: Receiver<GuardCheatCodes>,
        guard:    Guard<M, S>
    }

    impl<M, S> Future for GuardWrapper<M, S>
    where
        M: Middleware + Unpin + 'static,
        S: Simulator + Unpin + 'static,
        <M as Middleware>::Provider: PubsubClient
    {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Poll::Ready(msg) = self.receiver.poll_recv(cx) {
                let Some(msg) = msg else { return Poll::Ready(()) };

                match msg {
                    GuardCheatCodes::PeerAddr(sender) => {
                        let _ = sender.send(self.guard.network.local_addr());
                    }
                    GuardCheatCodes::MakeLeader(peer_id, block) => {
                        todo!()
                    }
                    GuardCheatCodes::CheckForBundle(hash, sender) => {
                        let _ = sender.send(self.guard.action.get_cow().check_for_bundle(hash));
                    }
                    GuardCheatCodes::CheckForUserTx(hash, sender) => {
                        let _ = sender.send(self.guard.action.get_cow().check_for_user_tx(hash));
                    }
                    GuardCheatCodes::CheckForSearcherTx(hash, sender) => {
                        let _ =
                            sender.send(self.guard.action.get_cow().check_for_searcher_tx(hash));
                    }
                    GuardCheatCodes::PropagateBundle => {
                        self.guard.action.get_cow().propagate_bundle();
                    }
                    GuardCheatCodes::PropagateUserTransactions => {
                        self.guard.action.get_cow().propagate_user_transactions();
                    }
                    GuardCheatCodes::PropagateSearcherTransactions => {
                        self.guard
                            .action
                            .get_cow()
                            .propagate_searcher_transactions();
                    }
                }
            }
            self.guard.poll_unpin(cx)
        }
    }
}
