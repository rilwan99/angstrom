use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::ConsensusState;

use super::{RoundAction, RoundStateMessage, StateTransition};

/// waiting for next block state
pub struct CompletedState;

impl StateTransition for CompletedState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        Poll::Pending
    }
}
