use std::{collections::HashMap, sync::Arc};

use alloy_primitives::{Address, B256, U256};
use parking_lot::RwLock;
use reth_provider::StateProvider;

#[derive(Clone)]
pub struct Balances {
    token_balances_slot: Arc<RwLock<HashMap<Address, U256>>>
}

impl Balances {
    pub fn fetch_balance_for_token<DB: StateProvider>(
        &self,
        user: Address,
        token: Address,
        db: DB
    ) -> Option<U256> {
        None
    }
}
