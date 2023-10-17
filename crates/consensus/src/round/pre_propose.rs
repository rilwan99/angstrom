use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use futures::FutureExt;
use guard_types::on_chain::{PoolKey, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement};

use super::{StateTransition, Timeout};

pub struct PreProposeState {
    is_leader:   bool,
    timeout:     Timeout,
    best_bundle: Option<SimmedBundle>
}

impl PreProposeState {
    pub fn new(is_leader: bool, timeout: Timeout) -> Self {
        Self { is_leader, timeout, best_bundle: None }
    }

    // TODO: this will change to all the fields we are voting on
    pub fn new_best_bundle(&mut self, bundle: SimmedBundle) {
        self.best_bundle = self.best_bundle.take().map(|cur_bundle| {
            if bundle.get_cumulative_lp_bribe() > cur_bundle.get_cumulative_lp_bribe() {
                bundle
            } else {
                cur_bundle
            }
        });
    }
}

impl StateTransition for PreProposeState {
    fn should_transition(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool> {
        self.timeout.poll_unpin(cx).map(|_| self.is_leader)
    }
}
