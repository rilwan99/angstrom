use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use ethers_providers::Middleware;
use futures::{Future, FutureExt};
use futures_util::StreamExt;
use guard_network::Swarm;
use leader::leader_manager::Leader;
use sim::Simulator;
use tokio::task::JoinHandle;

// use crate::submission_server::SubmissionServer;

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    /// guard network connection
    network:              Swarm,
    /// deals with leader related requests and actions
    leader:               Leader<M, S>,
    /// deals with new submissions through a rpc to the network
    server:               SubmissionServer,
    server_subscriptions: HashMap<SubscriptionKind, Vec<Sender<SubscriptionResult>>>,

    // we also can't enforce
    /// handle
    _simulator_thread: JoinHandle<()>
}

impl<M: Middleware + Unpin, S: Simulator> Guard<M, S> {
    fn handle_submissions(&mut self, cx: &mut Context<'_>, msgs: Vec<Submission>) {
        let submissions = msgs
            .into_iter()
            .filter_map(|submission| match submission {
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

        self.leader.new_transaction(submissions.clone());
        //TODO: propagate to network

        if let Some(senders) = self
            .server_subscriptions
            .get_mut(&SubscriptionKind::CowTransactions)
        {
            senders.retain(|sender| {
                sender
                    .try_send(SubscriptionResult::CowTransaction(submissions))
                    .is_ok()
            });
        }
    }

    fn handle_network_msg(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(msg) = self.network.poll_next_unpin(cx) {
            let Some(swarm_event) = msg else { return Poll::Ready(()) };
        }

        Poll::Ready(())
    }
}

impl<M: Middleware + Unpin, S: Simulator> Future for Guard<M, S> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(new_msgs) = self.server.poll_next_unpin(cx) {
            let Some(new_msgs) = new_msgs else { return Poll::Ready(()) };

            if self.handle_submissions(cx, new_msgs).is_ready() {
                return Poll::Ready(())
            };
        }

        if let Poll::Ready(msgs) = self.leader.poll(cx) {}

        todo!()
    }
}

