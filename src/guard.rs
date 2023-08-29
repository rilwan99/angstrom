use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use ethers_providers::Middleware;
use futures::{Future, FutureExt};
use futures_util::StreamExt;
use guard_network::{NetworkConfig, Swarm};
use jsonrpsee::server::ServerHandle;
use leader::leader_manager::{Leader, LeaderConfig};
use sim::Simulator;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::submission_server::{
    Submission, SubmissionServer, SubmissionServerConfig, SubmissionServerInner, SubscriptionKind,
    SubscriptionResult
};

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    /// guard network connection
    network: Swarm,
    /// deals with leader related requests and actions including bundle building
    leader:  Leader<M, S>,
    /// deals with new submissions through a rpc to the network
    server:  SubmissionServer,

    #[cfg(feature = "subscription")]
    /// make sure we keep subscribers upto date
    server_subscriptions: HashMap<SubscriptionKind, Vec<Sender<SubscriptionResult>>>,
    /// handle
    _simulator_thread:    JoinHandle<()>
}

impl<M: Middleware + Unpin, S: Simulator> Guard<M, S> {
    pub async fn new(
        network_config: NetworkConfig,
        leader_config: LeaderConfig<'_, M, S>,
        server_config: SubmissionServerConfig,
        sim_thread: JoinHandle<()>
    ) -> anyhow::Result<Self> {
        let sub_server = SubmissionServerInner::new(server_config).await?;
        let swarm = Swarm::new(network_config).await?;
        let leader = Leader::new(leader_config)?;

        Ok(Self {
            leader,
            #[cfg(feature = "subscription")]
            server_subscriptions: HashMap::default(),
            server: sub_server,
            network: swarm,
            _simulator_thread: sim_thread
        })
    }

    #[cfg(feature = "subscription")]
    fn handle_submissions(&mut self, msg: Submission) {
        let data = match msg {
            Submission::Subscription(kind, sender) => {
                self.server_subscriptions
                    .entry(kind)
                    .or_default()
                    .push(sender);
                return
            }
            Submission::Submission(typed_data) => typed_data
        };

        self.leader.new_transaction(data.clone());
        //TODO: propagate to network

        if let Some(senders) = self
            .server_subscriptions
            .get_mut(&SubscriptionKind::CowTransactions)
        {
            senders.retain(|sender| {
                sender
                    .try_send(SubscriptionResult::CowTransaction(data.clone()))
                    .is_ok()
            });
        }
    }

    #[cfg(not(feature = "subscription"))]
    fn handle_submissions(&mut self, msgs: Submission) {
        let Submission::Submission(data) = msg;
        self.leader.new_transaction(data.clone());
    }

    fn handle_network_msg(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(msg) = self.network.poll_next_unpin(cx) {
            let Some(swarm_event) = msg else { return Poll::Ready(()) };
        }

        Poll::Ready(())
    }
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Future for Guard<M, S> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(new_msgs) = self.server.poll_next_unpin(cx) {
            let Some(new_msgs) = new_msgs else { return Poll::Ready(()) };

            self.handle_submissions(new_msgs);
        }
        if let Poll::Ready(Some(msgs)) = self.network.poll_next_unpin(cx) {
            cx.waker().wake_by_ref();
            println!("{msgs:?}");
        }

        if let Poll::Ready(msgs) = self.leader.poll(cx) {}

        return Poll::Pending
    }
}
