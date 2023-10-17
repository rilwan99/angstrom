use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::ConsensusState;

use super::{RoundAction, RoundStateMessage, StateTransition};
/// This state is only reached if this guard is the leader
pub struct SubmitState {}

impl StateTransition for SubmitState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        todo!()
    }
}
