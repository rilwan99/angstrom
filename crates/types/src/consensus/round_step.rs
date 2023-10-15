use super::ConsensusStage;

/// the message that is sent out to other peered guards in order to inform them
/// that we have taken a step and the things we are currently processing have
/// changed
#[derive(Debug, Default)]
pub struct RoundStep {
    pub height:              u64,
    pub round:               u64,
    pub step:                ConsensusStage,
    pub seconds_since_start: u128,
    pub last_commit_round:   u64
}
