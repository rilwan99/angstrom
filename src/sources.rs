use std::{
    pin::Pin,
    task::{Context, Poll}
};

use action::RelaySender;
use ethers_providers::Middleware;
use futures::Stream;
use guard_network::Swarm;
use guard_types::on_chain::{SimmedBundle, UserOrder};

use crate::submission_server::SubmissionServer;

/// Holds all of our message sources
pub struct Sources<M: Middleware + 'static> {
    /// guard network connection
    guard_net:         Swarm,
    /// deals with new submissions through a rpc to the network
    submission_server: SubmissionServer,
    relay_sender:      RelaySender<M>
}

impl<M: Middleware + 'static> Sources<M> {
    pub fn new(
        guard_net: Swarm,
        submission_server: SubmissionServer,
        relay_sender: RelaySender<M>
    ) -> Self {
        Self { relay_sender, guard_net, submission_server }
    }

    /// grabs the guard network handle
    pub fn guard_net_mut(&mut self) -> &mut Swarm {
        &mut self.guard_net
    }

    /// sends the bundle to all specified relays
    pub fn send_to_relay(&mut self, bundle: SimmedBundle) {
        self.relay_sender.submit_bundle(bundle);
    }

    /// used to share new txes with externally subscribed users
    pub fn on_new_user_txes(&mut self, txes: Arc<Vec<UserOrder>>) {
        todo!()
    }

    /// used to share new bundles with externally subscribed users
    pub fn on_new_best_bundle(&mut self, bundle: Arc<SimmedBundle>) {
        todo!()
    }
}

impl<M: Middleware + 'static> Stream for Sources<M> {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
