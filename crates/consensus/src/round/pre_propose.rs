use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::{ConsensusState, IsLeader, COMMIT, PRE_PROPOSE, PROPOSE};
use futures::FutureExt;
use guard_types::on_chain::SimmedBundle;

use super::{
    commit::CommitState, propose::ProposeState, GlobalStateContext, RoundAction, RoundStateMessage,
    StateTransition, Timeout
};

pub struct PreProposeState {
    timeout:         Timeout,
    commited_bundle: SimmedBundle,
    is_leader:       IsLeader
}
impl PreProposeState {
    pub fn new(timeout: Timeout, commited_bundle: SimmedBundle, is_leader: IsLeader) -> Self {
        Self { timeout, commited_bundle, is_leader }
    }
}

impl StateTransition for PreProposeState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        _: GlobalStateContext
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        self.timeout.poll_unpin(cx).map(|_| {
            if self.is_leader.is_leader() {
                (RoundAction::Propose(ProposeState::new(cx.waker().clone())), PROPOSE, None)
            } else {
                (RoundAction::Commit(CommitState::new()), COMMIT, None)
            }
        })
    }
}
