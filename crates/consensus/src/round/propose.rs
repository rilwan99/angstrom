use std::{
    pin::Pin,
    task::{Context, Poll, Waker}
};

use common::{ConsensusState, SUBMIT};

use super::{
    submit::SubmitState, GlobalStateContext, RoundAction, RoundStateMessage, StateTransition
};

/// This state is only reached if this guard is the leader
pub struct ProposeState {
    proposal: ()
}

impl ProposeState {
    pub fn new(waker: Waker) -> Self {
        waker.wake();

        Self { proposal: () }
    }
}

impl StateTransition for ProposeState {
    fn should_transition(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _: GlobalStateContext
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        Poll::Ready((
            RoundAction::Submit(SubmitState::new()),
            SUBMIT,
            Some(RoundStateMessage::Proposal())
        ))
    }
}
