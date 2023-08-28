use std::collections::HashMap;

use ethers_core::types::{transactions::eip712::TypedData, Address, U256};
use shared::UserOrder;

pub struct SimulatedTransaction {
    typed_data: TypedData,
    details:    UserOrder
}

impl SimulatedTransaction {
    // TODO: we need a better way to represent this for low cap
    // as we need rem
    pub fn get_limit_price(&self) -> u128 {
        self.details.amount_in / self.details.amount_out_min
    }

    pub fn token_out(&self) -> Address {
        self.details.token_out
    }
}

pub struct CowSolver {
    all_transactions: HashMap<Address, Vec<SimulatedTransaction>>,
    best_bundle:      SealedBundle
}
