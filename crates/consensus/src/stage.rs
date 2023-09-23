use guard_types::consensus::Time;

/// the representation of these represents time in seconds
/// for the current cutoff period
pub enum ConsensusStage {
    DataPropagation     = 0,
    BundleSigningCutoff = 8,
    LeaderProposeCutoff = 9,
    PreVoteCutoff       = 10,
    CommitCutoff        = 11
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
