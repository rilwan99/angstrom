use sim::Simulator;

pub struct BundleBuilder<S: Simulator + Unpin + 'static> {
    sim: S
}

impl<S: Simulator + Unpin + 'static> BundleBuilder<S> {
    pub fn new(sim: S) -> Self {
        Self { sim }
    }
}
