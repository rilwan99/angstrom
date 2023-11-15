use std::{
    sync::Arc,
    task::{Context, Poll}
};

use common::PollExt;
use ethers_core::types::{Block, H256};
use ethers_flashbots::PendingBundleError;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use futures::Future;
use futures_util::StreamExt;
use guard_types::submission::SubmissionBundle;
use reth_provider::r#trait::chain::CanonStateNotifications;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    handle::{EthCommand, EthHandle},
    relay_sender::RelaySender
};

pub enum EthNetworkEvent {}

/// Holds all of our eth network state.
/// The relay sender part will mostly be dealing with Chainbound's echo

pub struct EthNetworkManager<M> {
    /// our command receiver
    commander:       ReceiverStream<EthCommand>,
    /// people listening to events
    event_listeners: Vec<Sender<EthNetworkEvent>>,

    /// for the leader to submit to relays
    relay_sender:      RelaySender<M>,
    /// Notifications for Canonical Block updates
    canonical_updates: CanonStateNotifications
}

impl<M> EthNetworkManager<M> {
    pub fn new(
        canonical_updates: CanonStateNotifications,
        relay_sender: RelaySender<M>
    ) -> anyhow::Result<EthHandle> {
        let (tx, rx) = channel(10);
        let stream = ReceiverStream::new(rx);

        let this = Self {
            relay_sender,
            canonical_updates,
            commander: stream,
            event_listeners: Vec::new()
        };

        let handle = EthHandle::new(tx);
        tokio::spawn(this);

        Ok(handle)
    }

    /// sends the bundle to all specified relays
    pub fn send_to_relay(&mut self, bundle: SubmissionBundle) {
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

impl<M> Future for EthNetworkManager<M> {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
