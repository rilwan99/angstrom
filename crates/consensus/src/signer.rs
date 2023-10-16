use ethers_signers::{LocalWallet, Signer as ESigner, WalletError};
use guard_types::{
    consensus::{BundleVote, LeaderProposal, SignedLeaderProposal, Time},
    on_chain::Signature
};
use reth_primitives::{keccak256, H256};
use reth_rlp::Encodable;
use revm_primitives::{bytes::BytesMut, Address};

/// The Signer deals with verifying external signatures as well as
/// signing our payloads
#[repr(transparent)]
pub struct Signer(
    //TODO: this will prob change in future. placeholder for now
    LocalWallet
);

impl Signer {
    pub fn sign_leader_proposal(
        &self,
        proposal: &LeaderProposal
    ) -> Result<SignedLeaderProposal, WalletError> {
        let hash = proposal.bundle.hash();

        self.0
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
        self.0.sign_hash(hash.into()).map(|signature| BundleVote {
            hash,
            bundle_hash,
            round,
            height: block_height,
            timestamp: Time::now(),
            signature: Signature(signature)
        })
    }
}
