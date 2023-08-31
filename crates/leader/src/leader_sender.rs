use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_flashbots::{
    BroadcasterMiddleware, BundleRequest, FlashbotsMiddlewareError, PendingBundle,
    PendingBundleError
};
use ethers_middleware::SignerMiddleware;
use ethers_providers::Middleware;
use ethers_signers::LocalWallet;
use futures::{Future, FutureExt};
use reth_primitives::PeerId;
use shared::{Batch, BatchSignature};
use tracing::{debug, info};

type StakedWallet = LocalWallet;
type BundleKey = LocalWallet;

pub type SubmissionFut = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;

pub struct LeaderSender<M: Middleware + 'static> {
    signer: Arc<SignerMiddleware<BroadcasterMiddleware<&'static M, BundleKey>, StakedWallet>>,
    signatures:    Vec<BatchSignature>,
    valid_stakers: Vec<PeerId>,
    batch:         Option<Batch>,
    future:        Option<SubmissionFut>
}

impl<M: Middleware + 'static> LeaderSender<M> {
    pub fn new(
        signer: Arc<SignerMiddleware<BroadcasterMiddleware<&'static M, BundleKey>, StakedWallet>>
    ) -> Self {
        Self {
            signer,
            signatures: Vec::new(),
            valid_stakers: Vec::new(),
            batch: None,
            future: None
        }
    }

    pub fn has_submitted(&self) -> bool {
        self.future.is_some()
    }

    pub fn has_selected_bundle(&self) -> bool {
        self.batch.is_some()
    }

    pub fn set_selected_batch(&mut self, batch: Batch) {
        self.batch = Some(batch);
    }

    pub fn on_new_block(&mut self) {
        self.signatures.clear();
        self.batch = None;
    }

    /// keeps collecting new signatures for specified bundle request.
    /// once the amount of signatures are met, we go ahead and send out
    /// the bundle
    pub fn new_signature(&mut self, sig: BatchSignature) {
        if self.signatures.contains(&sig) {
            debug!(?sig, "got a duplicate signature");
        }

        if let Ok(key) = sig.recover_key() {
            if self.valid_stakers.contains(&key) {
                info!(?key, "got new signature for bundle");
                self.signatures.push(sig);
            }
        }
        //TODO: check if we reached threshold and then submit
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        if let Some(submit_fut) = self.future.as_mut() {
            return submit_fut.poll_unpin(cx)
        }

        Poll::Pending
    }
}
