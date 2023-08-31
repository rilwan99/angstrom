use ethers_signers::{LocalWallet, Signer};
use reth_primitives::keccak256;
use revm_primitives::Address;
use shared::{Batch, BatchSignature};
use sim::Simulator;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BundleSigningError {
    #[error("Failed to simulate bundle: {0:#?}")]
    SimulationError(String),
    #[error("Failed to sign bundle: {0:#?}")]
    SigningError(String),
    #[error("The sign request was outside of the sign period")]
    NotDelegatedSigningTime
}

/// deals with verifying the bundle
pub struct BundleSigner<S: Simulator> {
    sim: S,
    // edsca key. in future will bls key
    key: LocalWallet
}

impl<S: Simulator> BundleSigner<S> {
    pub fn new(sim: S, key: LocalWallet) -> Self {
        Self { sim, key }
    }

    pub fn get_key(&self) -> Address {
        self.key.address().into()
    }

    /// TODO: this needs to be rewrote to sim the batch
    pub fn verify_batch_for_inclusion(
        &self,
        batch: &Batch
    ) -> Result<BatchSignature, BundleSigningError> {
        let bundle_hash = keccak256(
            &serde_json::to_vec(&batch)
                .map_err(|e| BundleSigningError::SigningError(e.to_string()))?
        );

        let signed_msg = tokio::runtime::Handle::current()
            .block_on(self.key.sign_message(bundle_hash))
            .map_err(|e| BundleSigningError::SigningError(e.to_string()))?;

        Ok(BatchSignature { sig: signed_msg, hash: bundle_hash })
    }
}
