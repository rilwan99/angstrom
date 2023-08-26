use futures::{Future, FutureExt};
use guard_network::Swarm;
use leader::leader_role::Leader;
use tokio::sync::JoinHandle;

use crate::submission_server::SubmissionServer;

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware, S: Simulations> {
    /// guard network connection
    network:     Swarm,
    /// deals with leader related requests and actions
    leader:      Leader<M>,
    /// deals with new submissions through a rpc to the network
    submissions: SubmissionServer,

    //TODO: Most likely no point of simming because thats gaurd duty?
    // we also can't enforce
    /// handle
    _simulator_thread: JoinHandle<()>
}

impl<M: Middleware, S: Simulations> Guard<M, S> {

    fn handle_submissions(&mut self, cx: &mut Context<'_>) {
        todo!()
    }
}

impl<M: Middleware, S: Simulations> Future for Guard<M, S> {
    type Output = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(submission) = self.submissions.poll_next_unpin(cx) {}
    }
}
