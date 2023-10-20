use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::{ConsensusState, COMMIT, PROPOSE};
use futures::FutureExt;

use super::{
    commit::CommitState, propose::ProposeState, GlobalStateContext, RoundAction, RoundStateMessage,
    StateTransition, Timeout
};

/// Given we have pre-proposed. we now wait the Timeout
/// before transitioning to the next state
pub struct PreProposeState {
    timeout: Timeout
}
impl PreProposeState {
    pub fn new(timeout: Timeout) -> Self {
        Self { timeout }
    }
}

impl StateTransition for PreProposeState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        gs_context: GlobalStateContext
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        self.timeout.poll_unpin(cx).map(|_| {
            if gs_context.is_leader.is_leader() {
                (RoundAction::Propose(ProposeState::new(cx.waker().clone())), PROPOSE, None)
            } else {
                (RoundAction::Commit(CommitState::new()), COMMIT, None)
            }
        })
    }
}
