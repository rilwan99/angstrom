use std::{
    sync::Arc,
    task::{Context, Poll}
};

use common::{PollExt, ThreadStream};
use ethers_core::types::{Block, H256};
use ethers_flashbots::PendingBundleError;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use futures_util::StreamExt;
use guard_network::{PeerMessages, Swarm, SwarmEvent};
use guard_types::on_chain::{SubmissionBundle, SubmittedOrder, VanillaBundle};

use crate::{
    relay_sender::RelaySender,
    submission_server::{Submission, SubmissionServer}
};

/// Holds all of our network state
pub struct NetworkManager<M: Middleware + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// guard network connection. This is a thread-stream as it is a pretty
    /// bulky poll
    guard_net:         ThreadStream<Swarm, SwarmEvent>,
    /// deals with new submissions through a rpc to the network
    submission_server: SubmissionServer,
    /// for the leader to submit to relays
    relay_sender:      RelaySender<M>,
    /// general new block stream. Will be updated when our local optimized
    /// mem-pool is built
    block_stream:      SubscriptionStream<'static, M::Provider, Block<H256>>
}

impl<M: Middleware + 'static> NetworkManager<M>
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

        Ok(Self {
            relay_sender,
            guard_net: ThreadStream::new(guard_net),
            submission_server,
            block_stream
        })
    }

    pub fn new_guard_msg(&mut self, msg: PeerMessages) {
        self.guard_net.send_msg(|guard| guard.propagate_msg(msg));
    }

    /// sends the bundle to all specified relays
    pub fn send_to_relay(&mut self, bundle: SubmissionBundle) {
        self.relay_sender.submit_bundle(bundle);
    }

    /// used to share new txes with externally subscribed users
    pub fn on_new_user_txes(&mut self, tx: Arc<SubmittedOrder>) {
        self.submission_server.on_new_user_tx(tx);
    }

    /// used to share new bundles with externally subscribed users
    pub fn on_new_best_bundle(&mut self, bundle: Arc<VanillaBundle>) {
        self.submission_server.on_new_best_bundle(bundle)
    }

    // poll fns

    pub fn poll_swarm(&mut self, cx: &mut Context<'_>) -> Poll<SwarmEvent> {
        self.guard_net.poll_next_unpin(cx).filter_map(|f| f)
    }

    pub fn poll_submission_server(&mut self, cx: &mut Context<'_>) -> Poll<Submission> {
        self.submission_server.poll_next_unpin(cx).filter_map(|f| f)
    }

    pub fn poll_block_stream(&mut self, cx: &mut Context<'_>) -> Poll<Block<H256>> {
        self.block_stream.poll_next_unpin(cx).filter_map(|f| f)
    }

    pub fn poll_relay_submission(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), PendingBundleError>> {
        self.relay_sender.poll(cx)
    }
}