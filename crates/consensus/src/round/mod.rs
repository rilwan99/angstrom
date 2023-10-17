use self::{
    bundle::BundleVoteManager, pre_commit::PreCommitState, propose::ProposeState,
    submit::SubmitState, vote::VoteState
};

pub mod bundle;
pub mod leader;
pub mod pre_commit;
pub mod propose;
pub mod submit;
pub mod vote;

/// The current state and subsequent actions that should be taken
/// for such state in a given round. All state that this contains
/// is transient to the given ethereum block height
pub struct RoundState {
    /// The current ethereum height
    current_height: u64,
    /// the currrent action we should be taking at the moment of
    /// time for this height
    current_state:  RoundAction,
    /// all votes for the given height
    vote_collector: BundleVoteManager
}

impl RoundState {}

/// Representation of a finite-state machine
pub enum RoundAction {
    /// The precommit state actions we
    PreCommit(PreCommitState),
    Propose(ProposeState),
    Vote(VoteState),
    Submit(SubmitState)
}

impl RoundAction {}
