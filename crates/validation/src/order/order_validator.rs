use std::task::Poll;

use angstrom_types::orders::{
    OrderValidationOutcome, PoolOrder, ValidatedOrder, ValidationResults
};
use futures::{Future, StreamExt};
use reth_provider::StateProviderFactory;

use super::{sim::SimValidation, state::StateValidation, OrderValidationRequest};
use crate::{common::lru_db::RevmLRU, validator::ValidationRequest};

#[allow(dead_code)]
pub struct OrderValidator<DB> {
    sim:   SimValidation<DB>,
    state: StateValidation<DB>
}

impl<DB> OrderValidator<DB>
where
    DB: StateProviderFactory + Unpin + 'static
{
    /// only checks state
    pub fn validate_order(&mut self, order: OrderValidationRequest) {
        match order {
            res @ OrderValidationRequest::ValidateLimit(..) => {
                self.state.validate_non_composable_order(res);
            }
            res @ OrderValidationRequest::ValidateSearcher(..) => {
                self.state.validate_non_composable_order(res);
            }
            res @ OrderValidationRequest::ValidateComposableLimit(..) => {
                todo!()
            }
            res @ OrderValidationRequest::ValidateComposableSearcher(..) => {
                todo!()
            }
        }
    }
}

impl<DB> Future for OrderValidator<DB>
where
    DB: StateProviderFactory + Unpin + 'static
{
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        while let Poll::Ready(Some(_)) = self.state.poll_next_unpin(cx) {}

        Poll::Pending
    }
}
