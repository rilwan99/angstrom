use std::{collections::HashMap, sync::Arc};

use alloy_primitives::{keccak256, Address, B256, U256};
use parking_lot::RwLock;
use reth_provider::StateProvider;

#[derive(Clone)]
pub struct Approvals {
    token_approval_slot: Arc<RwLock<HashMap<Address, U256>>>
}

impl Approvals {
    pub fn fetch_approval_balance_for_token<DB: StateProvider>(
        &self,
        user: Address,
        token: Address,
        db: DB
    ) -> Option<U256> {
        let slot_i: U256 = self.token_approval_slot.read().get(&token)?.clone();
        let mut slot = user.to_vec();
        slot.extend(slot_i.to_be_bytes::<32>().to_vec());
        let slot_addr = keccak256(slot);

        db.storage(token, slot_addr).unwrap()
    }
}
