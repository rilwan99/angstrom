use sim::Simulator;

use crate::{evidence::EvidenceCollector, verifier::DataVerifier};
/// The ConsensusCore module handles all calculations and prov
pub struct ConsensusCore<S: Simulator + 'static> {
    evidence_collector: EvidenceCollector,
    data_verifier:      DataVerifier<S>
}

impl<S: Simulator + 'static> ConsensusCore<S> {
    pub async fn new() -> Self {
        todo!()
    }
}
