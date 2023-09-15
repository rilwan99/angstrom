use guard_types::consensus::Vote;
use thiserror::Error;

use crate::clock::Time;

#[derive(Debug, Error)]
pub enum EvidenceError {
    #[error("invalid evidence")]
    InvalidEvidence
}

pub struct EvidenceCollector {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Evidence {
    DuplicateVoteEvidence(DuplicateVoteEvidence)
}

/// Duplicate vote evidence
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateVoteEvidence {
    pub vote_a:             Vote,
    pub vote_b:             Vote,
    pub total_voting_power: u64,
    pub validator_power:    u64,
    // TODO:
    pub timestamp:          Time
}

impl DuplicateVoteEvidence {
    /// constructor
    pub fn new(vote_a: Vote, vote_b: Vote) -> Result<Self, EvidenceError> {
        if vote_a.height != vote_b.height {
            return Err(EvidenceError::InvalidEvidence)
        }

        // Todo: make more assumptions about what is considered a valid evidence for
        // duplicate vote
        Ok(Self {
            vote_a,
            vote_b,
            total_voting_power: Default::default(),
            validator_power: Default::default(),
            timestamp: Time::unix_epoch()
        })
    }

    /// Get votes
    pub fn votes(&self) -> (&Vote, &Vote) {
        (&self.vote_a, &self.vote_b)
    }
}
