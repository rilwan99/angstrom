use std::task::Poll;

use futures::{Future, StreamExt};
use guard_types::orders::{OrderValidationOutcome, PoolOrder, ValidatedOrder, ValidationResults};

use super::{sim::SimValidation, state::StateValidation, OrderValidationRequest, OrderValidator};
use crate::{common::lru_db::RevmLRU, validator::ValidationRequest};

#[allow(dead_code)]
pub struct OrderValidator<DB> {
    sim:   SimValidation<DB>,
    state: StateValidation<DB>
}

impl<DB> OrderValidator<DB> {
    /// only checks state
    pub fn validate_order(&mut self, order: OrderValidationRequest) {
        match order {
            res @ OrderValidationRequest::ValidateLimit(..) => {
                self.state.validate_non_composable_order(res);
            }
            res @ OrderValidationRequest::ValidateSearcher(..) => {
                self.state.validate_non_composable_order(res);
            }
            res @ OrderValidationRequest::ValidateComposableLimit(..) => {}
            res @ OrderValidationRequest::ValidateComposableSearcher(..) => {}
        }
    }
}

impl Future for OrderValidator {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        let _ = self.state.poll_next_unpin(cx);
        Poll::Pending
    }
}
