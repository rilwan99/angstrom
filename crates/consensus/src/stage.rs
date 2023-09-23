use guard_types::consensus::Time;

/// the representation of these represents time in seconds
/// for the current cutoff period
pub enum ConsensusStage {
    DataPropagation     = 0,
    BundleSigningCutoff = 8,
    Bundle23PropCutoff  = 9,
    LeaderProposeCutoff = 10
}

impl ConsensusStage {
    pub fn get_current_period(time: Time) -> Self {
        todo!()
    }
}

pub struct Stage {
    pub height: u64,
    pub round:  u64,
    pub time:   Time,
    pub stage:  ConsensusStage
}

impl Stage {
    pub fn is_past_proposal_cutoff(&self) -> bool {
        todo!()
    }

    pub fn is_past_vote_cutoff(&self) -> bool {
        todo!()
    }
}
