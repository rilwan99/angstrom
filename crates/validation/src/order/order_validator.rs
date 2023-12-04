use super::{sim::SimValidation, state::StateValidation};

#[allow(dead_code)]
pub struct OrderValidator<DB> {
    sim:   SimValidation<DB>,
    state: StateValidation<DB>
}

impl<DB> OrderValidator<DB> {}
