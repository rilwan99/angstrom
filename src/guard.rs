use std::{
    pin::Pin,
    task::{Context, Poll}
};

use ethers_providers::Middleware;
use futures::{Future, FutureExt};
use guard_network::Swarm;
use leader::leader_manager::Leader;
use sim::Simulator;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::ReceiverStream;

use crate::submission_server::Submissions;

// use crate::submission_server::SubmissionServer;

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware + 'static, S: Simulator + 'static> {
    /// guard network connection
    network:           Swarm,
    /// deals with leader related requests and actions
    leader:            Leader<M, S>,
    /// deals with new submissions through a rpc to the network
    submissions: ReceiverStream<Submissions>,

    // we also can't enforce
    /// handle
    _simulator_thread: JoinHandle<()>,
    _rpc_thread:       JoinHandle<()>
}

impl<M: Middleware, S: Simulator> Guard<M, S> {
    fn handle_submissions(&mut self, cx: &mut Context<'_>) {
        todo!()
    }
}

impl<M: Middleware, S: Simulator> Future for Guard<M, S> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // if let Poll::Ready(submission) = self.submissions.poll_next_unpin(cx) {}
        todo!()
    }
}