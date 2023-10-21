use std::collections::HashSet;

use guard_types::on_chain::Evidence;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvidenceCollectorError {
    #[error("Evidence collector already has this evidence")]
    AlreadyContainsEvidence,
    #[error("Could not find requested evidence")]
    NoEvidenceFound
}

pub struct EvidenceCollector {
    pending_evidence: HashSet<Evidence>
}

impl EvidenceCollector {
    pub fn add_evidence(&mut self, evidence: Evidence) -> Result<(), EvidenceCollectorError> {
        if !self.pending_evidence.insert(evidence) {
            return Err(EvidenceCollectorError::AlreadyContainsEvidence)
        }

        Ok(())
    }

    pub fn check_evidence(&mut self, evidence: &Evidence) -> Result<bool, EvidenceCollectorError> {
        Ok(self.pending_evidence.contains(evidence)).and_then(|has| {
            if !has {
                Err(EvidenceCollectorError::NoEvidenceFound)
            } else {
                Ok(has)
            }
        })
    }
}
