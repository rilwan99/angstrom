use std::task::{Context, Poll};

use sim::Simulator;

pub enum BundleSignResults {
    Approve(),
    Failed()
}
/// deals with verifying the bundle
pub struct BundleSigner<S: Simulator> {
    sim: S
}

impl<S: Simulator> BundleSigner<S> {
    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<BundleSignResults> {
        todo!()
    }
}
