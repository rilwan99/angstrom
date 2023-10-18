use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::ConsensusState;

use super::{RoundAction, RoundStateMessage, StateTransition};

pub struct CommitState {}

impl StateTransition for CommitState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        todo!()
    }
}
