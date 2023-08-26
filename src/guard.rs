use guard_network::state::NetworkState;
use leader::leader_role::Leader;
use tokio::sync::JoinHandle;

use crate::submission_server::SubmissionServer;

/// This is the control unit of the guard that delegates
/// all of our signing and messages.
pub struct Guard<M: Middleware, S: Simulations> {
    /// guard network connection
    network:     NetworkState,
    /// deals with leader related requests and actions
    leader:      Leader<M>,
    /// deals with new submissions through a rpc to the network
    submissions: SubmissionServer,

    //TODO: Most likely no point of simming because thats gaurd duty?
    // we also can't enforce
    /// handle
    _simulator_thread: JoinHandle<()>
}
