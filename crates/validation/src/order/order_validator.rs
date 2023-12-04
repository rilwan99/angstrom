use guard_types::orders::{OrderValidationOutcome, PoolOrder, ValidatedOrder, ValidationResults};

use super::{sim::SimValidation, state::StateValidation};

#[allow(dead_code)]
pub struct OrderValidator<DB> {
    sim:   SimValidation<DB>,
    state: StateValidation<DB>
}

impl<DB> OrderValidator<DB> {
    /// only checks state
    pub fn validate_limit_order<O: PoolOrder>(&mut self, order: O) -> OrderValidationOutcome<O> {
        todo!()
    }

    pub fn validate_composable_order<O: PoolOrder>(
        &mut self,
        order: O
    ) -> OrderValidationOutcome<O> {
        todo!()
    }
}
