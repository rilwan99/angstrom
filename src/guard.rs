use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_providers::Middleware;
use futures::{Future, FutureExt};
use futures_util::StreamExt;
use guard_network::{GaurdStakingEvent, NetworkConfig, PeerMessages, Swarm, SwarmEvent};
use jsonrpsee::server::ServerHandle;
use leader::leader_manager::{Leader, LeaderConfig, LeaderMessage};
use sim::Simulator;
use tokio::{
    sync::mpsc::{Sender, UnboundedSender},
    task::JoinHandle
};
use tracing::{debug, trace, warn};

use crate::submission_server::{
    Submission, SubmissionServer, SubmissionServerConfig, SubmissionServerInner, SubscriptionKind,
    SubscriptionResult
};

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    /// guard network connection
    network:              Swarm,
    /// deals with leader related requests and actions including bundle building
    leader:               Leader<M, S>,
    /// deals with new submissions through a rpc to the network
    server:               SubmissionServer,
    /// channel for sending updates to the set of stakers
    valid_stakers_tx:     UnboundedSender<GaurdStakingEvent>,
    /// make sure we keep subscribers upto date
    server_subscriptions: HashMap<SubscriptionKind, Vec<Sender<SubscriptionResult>>>
}

impl<M: Middleware + Unpin, S: Simulator> Guard<M, S> {
    pub async fn new(
        network_config: NetworkConfig,
        leader_config: LeaderConfig<M, S>,
        server_config: SubmissionServerConfig
    ) -> anyhow::Result<Self> {
        let (valid_stakers_tx, valid_stakers_rx) = tokio::sync::mpsc::unbounded_channel();
        let sub_server = SubmissionServerInner::new(server_config).await?;
        let swarm = Swarm::new(network_config, valid_stakers_rx).await?;
        let leader = Leader::new(leader_config)?;

        Ok(Self {
            leader,
            server_subscriptions: HashMap::default(),
            server: sub_server,
            valid_stakers_tx,
            network: swarm
        })
    }

    fn handle_submissions(&mut self, msgs: Vec<Submission>) {
        debug!(amount = msgs.len(), "handling new submissions");

        let submissions = msgs
            .into_iter()
            .filter_map(|msg| match msg {
                Submission::Subscription(kind, sender) => {
                    self.server_subscriptions
                        .entry(kind)
                        .or_default()
                        .push(sender);
                    None
                }
                Submission::Submission(typed_data) => Some(typed_data)
            })
            .collect::<Vec<_>>();

        self.leader.new_transactions(submissions.clone());
        let submissions = Arc::new(submissions);
        // we sim transactions locally before
        if let Some(senders) = self
            .server_subscriptions
            .get_mut(&SubscriptionKind::CowTransactions)
        {
            senders.retain(|sender| {
                sender
                    .try_send(SubscriptionResult::CowTransaction(submissions.clone()))
                    .is_ok()
            });
        }
    }

    fn handle_network_events(&mut self, network_events: Vec<SwarmEvent>) {
        network_events.into_iter().for_each(|event| match event {
            SwarmEvent::ValidMessage { peer_id, request } => {
                debug!(?peer_id, ?request, "got data from peer");
                match request {
                    PeerMessages::PeerRequests(req) => match req {
                        guard_network::PeerRequests::GetTeeModule(_, sender) => {
                            warn!("got a tee module request");
                        }
                    },
                    PeerMessages::PropagateTransactions(new_txes) => {}
                    PeerMessages::PropagateBundleSignature(new_sig) => {
                        if self.leader.is_leader() {}
                    }
                    PeerMessages::PropagateSignatureRequest(btach) => {}
                }
            }
            res @ _ => {
                debug!(?res, "got swarm event");
            }
        });
    }

    fn handle_leader_events(&mut self, leader_events: Vec<LeaderMessage>) {
        debug!(?leader_events, "got actions from the leader");
    }
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Future for Guard<M, S> {
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

            while let Poll::Ready(msgs) = self.leader.poll(cx) {}

            work -= 1;
            if work == 0 {
                cx.waker().wake_by_ref();
                return Poll::Pending
            }
        }
    }
}
