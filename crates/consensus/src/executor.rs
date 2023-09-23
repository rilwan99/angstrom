use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_signers::{LocalWallet, Signer, WalletError};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use guard_types::{
    consensus::{BundleVote, LeaderProposal, SignedLeaderProposal},
    on_chain::{CallerInfo, Signature}
};
use reth_primitives::{keccak256, H256};
use revm_primitives::{Address, B160};
use sim::{errors::SimError, Simulator};
use thiserror::Error;

pub enum ExecutorMessage {}

#[derive(Debug, Error)]
pub enum BundleError {
    #[error("Failed to simulate bundle: {0:#?}")]
    SimulationError(#[from] SimError),
    #[error("Bundle reverted")]
    BundleRevert,
    #[error("Failed to sign bundle: {0:#?}")]
    SigningError(String),
    #[error("The sign request was outside of the sign period")]
    NotDelegatedSigningTime
}

type PendingSims = Pin<Box<dyn Future<Output = Result<ExecutorMessage, BundleError>> + Send>>;
/// verifies all signed data requests from the guard network
pub struct Executor<S: Simulator + 'static> {
    sim:          S,
    /// edsca key. in future will bls key
    key:          LocalWallet,
    /// pending sims
    pending_sims: FuturesUnordered<PendingSims>,
    call_info:    CallerInfo
}
impl<S: Simulator + 'static> Executor<S> {
    pub fn new(sim: S, key: LocalWallet) -> Self {
        Self {
            sim,
            key,
            pending_sims: FuturesUnordered::default(),
            call_info: CallerInfo {
                address:   B160::default(),
                nonce:     69,
                overrides: HashMap::new()
            }
        }
    }

    pub fn sign_leader_proposal(
        &self,
        proposal: LeaderProposal
    ) -> Result<SignedLeaderProposal, WalletError> {
        let hash = proposal.bundle.hash();

        self.key
            .sign_hash(hash)
            .map(|signature| SignedLeaderProposal(signature))
    }

    pub fn sign_bundle_vote(
        &self,
        bundle_hash: H256,
        block_height: u64,
        round: u64
    ) -> Result<BundleVote, WalletError> {
        let hash = keccak256((bundle_hash, block_height, round));
        self.key.sign_hash(hash).map(|signature| BundleVote {
            hash,
            bundle_hash,
            round,
            height: block_height,
            timestamp: Time::now(),
            signature: Signature(signature)
        })
    }

    pub fn get_key(&self) -> Address {
        self.key.address().into()
    }

    pub fn verify_bundle_for_inclusion(
        &self,
        bundle: Arc<LeaderProposal>
    ) -> Result<(), BundleError> {
        let hash: H256 = bundle.bundle.raw.clone().into();

        let handle = self.sim.clone();
        // rip
        let cloned_bundle = (*bundle).clone();
        let call_info = self.call_info.clone();
        let key = self.key.clone();

        self.pending_sims.push(Box::pin(async move {
            let sig = key
                .sign_message(hash)
                .await
                .map_err(|e| BundleError::SigningError(e.to_string()))?;

            let sign_messaged = Signature(sig);

            // if handle
            //     .simulate_sign_request(call_info, cloned_bundle)
            //     .await
            //     .map(|res| res.is_success())?
            // {
            //     Ok(BundleSignature { sig: sign_messaged, hash })
            // } else {
            Err(BundleError::BundleRevert)
            // }
        }));

        Ok(())
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Result<ExecutorMessage, BundleError>> {
        if let Poll::Ready(Some(res)) = self.pending_sims.poll_next_unpin(cx) {
            return Poll::Ready(res)
        }
        Poll::Pending
    }
}
