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
use reth_provider::{CanonStateNotification, CanonStateNotifications, StateProviderFactory};
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    handle::{EthCommand, EthHandle},
    relay_sender::RelaySender
};

pub enum EthNetworkEvent {}

/// Holds all of our eth network state.
/// Will do the following
/// 1) Deal with submitting bundles
/// 2) Deal with fetching block state differences + fmt (need for validation and
///    orderpool)
pub struct EthNetworkManager<M: Middleware + 'static, DB> {
    /// our command receiver
    commander:       ReceiverStream<EthCommand>,
    /// people listening to events
    event_listeners: Vec<Sender<EthNetworkEvent>>,

    /// for the leader to submit to relays
    relay_sender:      RelaySender<M>,
    /// Notifications for Canonical Block updates
    canonical_updates: CanonStateNotifications,
    /// used to fetch data from db
    db:                DB
}

impl<M, DB> EthNetworkManager<M, DB>
where
    M: Middleware + 'static,
    DB: StateProviderFactory + Send + Sync + Unpin + 'static
{
    pub fn new(
        canonical_updates: CanonStateNotifications,
        relay_sender: RelaySender<M>,
        db: DB
    ) -> anyhow::Result<EthHandle> {
        let (tx, rx) = channel(10);
        let stream = ReceiverStream::new(rx);

        let this = Self {
            relay_sender,
            canonical_updates,
            commander: stream,
            event_listeners: Vec::new(),
            db
        };

        let handle = EthHandle::new(tx);
        tokio::spawn(this);

        Ok(handle)
    }

    /// sends the bundle to all specified relays
    pub fn send_to_relay(&mut self, bundle: SubmissionBundle) {
        self.relay_sender.submit_bundle(bundle);
    }

    pub fn poll_relay_submission(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<Result<(), PendingBundleError>> {
        self.relay_sender.poll(cx)
    }

    fn on_canon_update(&mut self, canonical_updates: CanonStateNotification) {
        match canonical_updates {
            CanonStateNotification::Reorg { old, new } => {}
            CanonStateNotification::Commit { new } => {}
        }
    }
}

impl<M, DB> Future for EthNetworkManager<M, DB>
where
    M: Middleware + 'static,
    DB: StateProviderFactory + Send + Sync + Unpin + 'static
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
