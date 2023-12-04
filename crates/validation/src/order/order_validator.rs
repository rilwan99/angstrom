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
    pub fn validate_non_composable(&mut self, order: OrderValidationRequest) {
        self.validate_non_composable(order);
    }

    pub fn validate_composable_order<O: PoolOrder>(
        &mut self,
        order: O
    ) -> OrderValidationOutcome<O> {
        todo!()
    }
}
