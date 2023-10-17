use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::{ConsensusState, IsLeader, PRE_PROPOSE};
use futures::FutureExt;
use guard_types::on_chain::SimmedBundle;

use super::{pre_propose::PreProposeState, RoundAction, StateTransition, Timeout};

pub struct OrderAccumulationState {
    timeout:     Timeout,
    best_bundle: Option<SimmedBundle>,
    is_leader:   IsLeader
}
impl OrderAccumulationState {
    pub fn new(timeout: Timeout, is_leader: IsLeader) -> Self {
        Self { timeout, best_bundle: None, is_leader }
    }
}

impl StateTransition for OrderAccumulationState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<(RoundAction, ConsensusState)> {
        self.timeout.poll_unpin(cx).map(|best_bundle| {
            (
                RoundAction::PrePropose(PreProposeState::new(
                    self.timeout,
                    self.best_bundle.take(),
                    self.is_leader.clone()
                )),
                PRE_PROPOSE
            )
        })
    }
}
