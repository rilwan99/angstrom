use alloy_primitives::B512;
use ethers_signers::{LocalWallet, WalletError};
use guard_types::{
    consensus::{Commit, PreProposal, Proposal},
    primitive::Angstrom::{Bundle, LowerBound}
};
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

    pub fn is_us(&self, addr: &B512) -> bool {
        todo!("change key to proper")
    }
}
