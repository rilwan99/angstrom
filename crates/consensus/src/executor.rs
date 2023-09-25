use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_signers::{LocalWallet, Signer, WalletError};
use futures::{stream::FuturesUnordered, Future, StreamExt};
use guard_types::{
    consensus::{BundleVote, LeaderProposal, SignedLeaderProposal, Time},
    on_chain::{CallerInfo, Signature}
};
use reth_primitives::{keccak256, H256};
use reth_rlp::Encodable;
use revm_primitives::{bytes::BytesMut, Address, B160};
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
        proposal: &LeaderProposal
    ) -> Result<SignedLeaderProposal, WalletError> {
        let hash = proposal.bundle.hash();

        self.key
            .sign_hash(hash.into())
            .map(|signature| SignedLeaderProposal(Signature(signature)))
    }

    pub fn sign_bundle_vote(
        &self,
        bundle_hash: H256,
        block_height: u64,
        round: u64
    ) -> Result<BundleVote, WalletError> {
        let mut buf = BytesMut::new();
        bundle_hash.encode(&mut buf);
        block_height.encode(&mut buf);
        round.encode(&mut buf);

        let hash = keccak256(&buf.freeze()[..]);
        self.key.sign_hash(hash.into()).map(|signature| BundleVote {
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

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Result<ExecutorMessage, BundleError>> {
        if let Poll::Ready(Some(res)) = self.pending_sims.poll_next_unpin(cx) {
            return Poll::Ready(res)
        }
        Poll::Pending
    }
}
