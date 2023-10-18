use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use action::RelaySender;
use common::{return_if, PollExt};
use ethers_core::types::{Block, H256};
use ethers_flashbots::PendingBundleError;
use ethers_providers::{Middleware, PubsubClient, RpcError, SubscriptionStream};
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
        middleware: &'static M,
        guard_net: Swarm,
        submission_server: SubmissionServer,
        relay_sender: RelaySender<M>
    ) -> anyhow::Result<Self> {
        let block_stream = middleware.subscribe_blocks().await?;

        Ok(Self { relay_sender, guard_net, submission_server, block_stream })
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
    pub fn on_new_user_txes(&mut self, tx: Arc<UserOrder>) {
        self.submission_server.on_new_user_tx(tx);
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
        return_if!(
        self
            .guard_net
            .poll_next_unpin(cx)
            .filter_map(|poll| poll)
            .map(|event| Some(SourceMessages::Swarm(event))) => { is_ready() }
        );

        return_if!(
        self
            .submission_server
            .poll_next_unpin(cx)
            .filter_map(|poll| poll)
            .map(|event| Some(SourceMessages::SubmissionServer(event))) => { is_ready() }
        );

        return_if!(
        self
            .block_stream
            .poll_next_unpin(cx)
            .filter_map(|poll| poll)
            .map(|event| Some(SourceMessages::NewEthereumBlock(event))) =>{ is_ready() }
        );

        return_if!(
        self
            .relay_sender
            .poll(cx)
            .map(|event| Some(SourceMessages::RelaySubmission(event))) => { is_ready() }
        );

        Poll::Pending
    }
}
