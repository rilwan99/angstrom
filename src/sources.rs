use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use action::RelaySender;
use ethers_core::types::{Block, H256};
use ethers_flashbots::PendingBundleError;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use futures::Stream;
use futures_util::StreamExt;
use guard_network::{Swarm, SwarmEvent};
use guard_types::on_chain::{SimmedBundle, UserOrder};

use crate::submission_server::{Submission, SubmissionServer};

pub enum SourceMessages {
    Swarm(SwarmEvent),
    SubmissionServer(Submission),
    NewEthereumBlock(Block<H256>),
    RelaySubmission(Result<(), PendingBundleError>)
}

/// Holds all of our message sources
pub struct Sources<M: Middleware + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// guard network connection
    guard_net:         Swarm,
    /// deals with new submissions through a rpc to the network
    submission_server: SubmissionServer,
    /// for the leader to submit to relays
    relay_sender:      RelaySender<M>,
    /// general new block stream. Will be updated when our local optimized
    /// mem-pool is built
    block_stream:      SubscriptionStream<'static, M::Provider, Block<H256>>
}

impl<M: Middleware + 'static> Sources<M>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(
        guard_net: Swarm,
        submission_server: SubmissionServer,
        relay_sender: RelaySender<M>
    ) -> Self {
        let block_stream = middleware.subscribe_blocks().await?;
        Self { relay_sender, guard_net, submission_server, block_stream }
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
        self.submission_server.on_new_user_txes(txes);
    }

    /// used to share new bundles with externally subscribed users
    pub fn on_new_best_bundle(&mut self, bundle: Arc<SimmedBundle>) {
        self.submission_server.on_new_best_bundle(bundle)
    }
}

impl<M: Middleware + 'static> Stream for Sources<M>
where
    <M as Middleware>::Provider: PubsubClient
{
    type Item = SourceMessages;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(swarm_event)) = self.guard_net.poll_next_unpin(cx) {
            return Poll::Ready(Some(SourceMessages::Swarm(swarm_event)))
        }

        while let Poll::Ready(Some(sub_server)) = self.submission_server.poll_next_unpin(cx) {
            return Poll::Ready(Some(SourceMessages::SubmissionServer(sub_server)))
        }

        while let Poll::Ready(Some(eth_block)) = self.block_stream.poll_next_unpin(cx) {
            return Poll::Ready(Some(SourceMessages::NewEthereumBlock(eth_block)))
        }

        if let Poll::Ready(relay_result) = self.relay_sender.poll(cx) {
            return Poll::Ready(Some(SourceMessages::RelaySubmission(relay_result)))
        }

        Poll::Pending
    }
}
