use sim::Simulator;

use crate::{evidence::EvidenceCollector, verifier::DataVerifier};

/// The ConsensusCore module handles everything related to consensus.
/// This includes but not limited to.
///
/// 1) Collecting Evidence against misbehavior
/// 2) Verifying Block and bundle data
/// 3) Leader Selection
/// 4) Verifying Historical State
/// 5) Signing Votes, Commitments & Proposals
pub struct ConsensusCore<S: Simulator + 'static> {
    evidence_collector: EvidenceCollector,
    data_verifier:      DataVerifier<S>
}

impl<S: Simulator + 'static> ConsensusCore<S> {
    pub async fn new() -> Self {
        todo!()
    }
}
