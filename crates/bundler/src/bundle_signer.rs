use sim::Simulator;

pub enum BundleSignResults {
    Approve(),
    Failed()
}
/// deals with verifying the bundle
pub struct BundleSigner<S: Simulator> {
    sim: S
}

