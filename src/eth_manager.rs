use std::{
    sync::Arc,
    task::{Context, Poll}
};

use common::{PollExt, ThreadStream};
use ethers_core::types::{Block, H256};
use ethers_flashbots::PendingBundleError;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use futures_util::StreamExt;
use guard_types::{primitive::Angstrom::Bundle, rpc::SignedLimitOrder};

use crate::relay_sender::RelaySender;

/// Holds all of our network state
pub struct EthNetworkManager<M: Middleware + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// for the leader to submit to relays
    relay_sender: RelaySender<M>,
    /// general new block stream. Will be updated when our local optimized
    /// mem-pool is built
    block_stream: SubscriptionStream<'static, M::Provider, Block<H256>>
}

impl<M: Middleware + 'static> EthNetworkManager<M>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(middleware: &'static M, relay_sender: RelaySender<M>) -> anyhow::Result<Self> {
        let block_stream = middleware.subscribe_blocks().await?;

        Ok(Self { relay_sender, block_stream })
    }

    /// sends the bundle to all specified relays
    pub fn send_to_relay(&mut self, bundle: ()) {
        self.relay_sender.submit_bundle(bundle);
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
