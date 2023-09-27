use super::ConsensusStage;

pub struct RoundStep {
    height:              u64,
    round:               u64,
    step:                ConsensusStage,
    seconds_since_start: u128,
    last_commit_round:   u32
}
