use std::task::Context;

use guard_types::consensus::{ConsensusStage, Time};

pub struct Stage {
    height: u64,
    round:  u64,

    time:  Time,
    stage: ConsensusStage
}

impl Stage {
    pub fn new_height(&mut self) {
        self.round = 0;
        self.height += 1;
    }

    pub fn new_round(&mut self) {
        self.round += 1;
    }

    pub fn round(&self) -> u64 {
        self.round
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn is_past_proposal_cutoff(&self) -> bool {
        todo!()
    }

    pub fn is_past_proposal_vote_cutoff(&self) -> bool {
        todo!()
    }

    pub fn is_past_bundle_signing_cutoff(&self) -> bool {
        todo!()
    }

    pub fn is_past_bundle23_prop_cutoff(&self) -> bool {
        todo!()
    }

    pub fn update_current_stage(&mut self) -> bool {
        todo!()
    }
}
