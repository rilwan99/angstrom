use super::{sim::SimValidation, state::StateValidation};

pub struct OrderValidator<DB> {
    sim:   SimValidation,
    state: StateValidation<DB>
}

impl<DB> OrderValidator<DB> {
}
