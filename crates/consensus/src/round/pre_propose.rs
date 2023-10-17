use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::{ConsensusState, IsLeader, PROPOSE};
use futures::FutureExt;
use guard_types::on_chain::{PoolKey, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement};

use super::{propose::ProposeState, RoundAction, StateTransition, Timeout};

pub struct PreProposeState {
    timeout:         Timeout,
    commited_bundle: Option<SimmedBundle>,
    is_leader:       IsLeader
}

impl PreProposeState {
    pub fn new(
        timeout: Timeout,
        commited_bundle: Option<SimmedBundle>,
        is_leader: IsLeader
    ) -> Self {
        Self { timeout, commited_bundle, is_leader }
    }
}

impl StateTransition for PreProposeState {
    fn should_transition(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<(RoundAction, ConsensusState)> {
        self.timeout
            .poll_unpin(cx)
            .map(|best_bundle| (RoundAction::Propose(ProposeState {}), PROPOSE))
    }
}
