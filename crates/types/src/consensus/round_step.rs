use super::ConsensusStage;

/// the message that is sent out to other peered guards in order to inform them
/// that we have taken a step and the things we are currently processing have
/// changed
#[derive(Debug, Default)]
pub struct RoundStep {
    height:              u64,
    round:               u64,
    step:                ConsensusStage,
    seconds_since_start: u128,
    last_commit_round:   u64
}
