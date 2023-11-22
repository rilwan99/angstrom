use std::{collections::HashMap, sync::Arc};

use alloy_primitives::{Address, U256};
use parking_lot::RwLock;

#[derive(Clone)]
pub struct Approvals {
    token_approval_slot: Arc<RwLock<HashMap<Address, U256>>>
}

impl Approvals {
    pub fn fetch_approval_balance_for_token(&self, user: Address, token: Address) -> Option<U256> {
        None
    }
}
