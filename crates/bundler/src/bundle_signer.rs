use shared::Bundle;
use sim::Simulator;
use this_error::Error;

#[derive(Error)]
pub enum BundleSigningError{
    #[error("Failed to simulate bundle: {0:#?}")]
    SimulationError
    #[error("failed to sign bundle: {0:#?}")]
    SigningError
}


/// deals with verifying the bundle
pub struct BundleSigner<S: Simulator> {
    sim: S,
    // edsca key. in future will bls key
    key: Wallet<SigningKey>
}

impl<S: Simulator> BundleSigner<S> {
    async fn sim_and_sign_bundle(&self) -> Result<Bundle, BundleSigningError> {
        todo!()
    }
}
