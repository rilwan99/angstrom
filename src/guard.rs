use std::{
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use common::PollExt;
use consensus::{ConsensusCore, ConsensusHandle, ConsensusManager, ConsensusMessage};
use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, PubsubClient};
use futures::{Future, FutureExt};
use futures_util::StreamExt;
use guard_network::NetworkConfig;
use sim::Simulator;
use tokio::sync::mpsc::Sender;
use tracing::debug;
use url::Url;

use crate::GeneralConfig;

// TODO: these values should be moved somewhere else bc there ugly
static SIMULATION_RELAY: &str = "https://relay.flashbots.net";
static BUILDER_URLS: &[&str] = &[
    "https://builder0x69.io",
    "https://rpc.beaverbuild.org",
    "https://relay.flashbots.net",
    "https://rsync-builder.xyz",
    "https://rpc.titanbuilder.xyz",
    "https://api.blocknative.com/v1/auction",
    "https://mev.api.blxrbdn.com",
    "https://eth-builder.com",
    "https://builder.gmbit.co/rpc",
    "https://buildai.net",
    "https://rpc.payload.de",
    "https://rpc.lightspeedbuilder.info",
    "https://rpc.nfactorial.xyz"
];

/// holds all the guard handles and deals with failures
pub struct Guard<M, S>
where
    M: Middleware + Unpin + 'static,
    S: Simulator + Unpin + 'static,
    <M as Middleware>::Provider: PubsubClient
{
    /// guard network connection
    consensus: ConsensusHandle,
    /// holder for if we need
    _p:        PhantomData<(M, S)>
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Guard<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(
        middleware: &'static M,
        network_config: NetworkConfig,
        general_config: GeneralConfig<S>
    ) -> anyhow::Result<Self> {
        let (consensus, current_height) = ConsensusManager::new().await;

        Ok(Self { consensus, _p: Default::default() })
    }
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Future for Guard<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
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
            middleware: &'static M,
            network_config: NetworkConfig,
            leader_config: ActionConfig<S>,
            server_config: SubmissionServerConfig
        ) -> anyhow::Result<Self>
        where
            M: Middleware + Unpin + 'static,
            S: Simulator + Unpin + 'static,
            <M as Middleware>::Provider: PubsubClient
        {
            let (tx, rx) = channel(10);
            let guard =
                Guard::new(middleware, network_config, leader_config, server_config).await?;
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
