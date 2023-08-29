use shared::Bundle;
use sim::Simulator;
use thiserror::Error;
use ethers_signers::LocalWallet;

#[derive(Debug, Error)]
pub enum BundleSigningError {
    #[error("Failed to simulate bundle")]
    SimulationError,
    #[error("failed to sign bundle")]
    SigningError,
}


/// deals with verifying the bundle
pub struct BundleSigner<S: Simulator> {
    sim: S,
    // edsca key. in future will bls key
    key: LocalWallet,
}

impl<S: Simulator> BundleSigner<S> {
    async fn sim_and_sign_bundle(&self) -> Result<Bundle, BundleSigningError> {
        todo!()
    }
}
