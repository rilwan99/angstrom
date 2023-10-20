use std::{
    pin::Pin,
    task::{Context, Poll, Waker}
};

use common::{ConsensusState, ORDER_ACCUMULATION, WAITING_NEXT_BLOCK};
use guard_types::on_chain::SimmedBundle;
use reth_primitives::H512;

use super::{
    completed::CompletedState, order_accumulation::OrderAccumulationState, GlobalStateContext,
    RoundAction, RoundStateMessage, StateTransition
};

pub enum CommitVote {
    Commit(()),
    Nil(())
}

pub struct CommitState {
    proposal:    Option<()>,
    best_bundle: SimmedBundle,
    waker:       Waker,
    vote:        Option<CommitVote>
}

impl CommitState {
    pub fn new() -> Self {
        todo!()
    }

    pub fn on_proposal(&mut self, proposal: ()) {
        // some code here to check the proposal against our bundle
        // to make sure that the lower bound and other stuff is
        // up to standard
        self.waker.wake_by_ref();
        todo!()
    }
}

impl StateTransition for CommitState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _: GlobalStateContext
    ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
        if let Some(vote) = self.vote.take() {
            return Poll::Ready((
                RoundAction::Completed(CompletedState),
                WAITING_NEXT_BLOCK,
                Some(RoundStateMessage::Commit())
            ))
        }
        Poll::Pending
    }
}
