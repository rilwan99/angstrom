use ethers_signers::{LocalWallet, Signer as ESigner, WalletError};
use guard_types::{
    consensus::{LeaderProposal, PrePreposeBundle, ProposalCommit},
    on_chain::{LowerBound, Signature, VanillaBundle}
};
use reth_primitives::{keccak256, H256, H512};
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
    pub fn sign_proposal(
        &self,
        ethereum_block: u64,
        vanilla_bundle: VanillaBundle,
        lower_bound: LowerBound
    ) -> Result<LeaderProposal, WalletError> {
        todo!()
    }

    pub fn sign_commit(
        &self,
        ethereum_block: u64,
        proposal: LeaderProposal
    ) -> Result<ProposalCommit, WalletError> {
        todo!()
    }

    pub fn sign_pre_propose(
        &self,
        ethereum_block: u64,
        bundle: VanillaBundle
    ) -> Result<PrePreposeBundle, WalletError> {
        todo!()
    }

    pub fn is_us(&self, addr: &H512) -> bool {
        todo!("change key to proper")
    }
}
