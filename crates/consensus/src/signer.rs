use alloy_rlp::Encodable;
use ethers_signers::{LocalWallet, Signer as ESigner, WalletError};
use guard_types::{
    consensus::{Commit, PreProposal, Proposal},
    primitive::{
        Angstrom::{Bundle, LowerBound},
        Signature
    }
};
use reth_primitives::{keccak256, BytesMut, H256, H512};
use revm_primitives::Address;

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
        vanilla_bundle: Bundle,
        lower_bound: LowerBound
    ) -> Result<Proposal, WalletError> {
        todo!()
    }

    pub fn sign_commit(
        &self,
        ethereum_block: u64,
        proposal: Proposal
    ) -> Result<Commit, WalletError> {
        todo!()
    }

    pub fn sign_pre_propose(
        &self,
        ethereum_block: u64,
        bundle: Bundle
    ) -> Result<PreProposal, WalletError> {
        todo!()
    }

    pub fn is_us(&self, addr: &H512) -> bool {
        todo!("change key to proper")
    }
}
