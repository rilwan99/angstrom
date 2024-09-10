use alloy_primitives::Log;
use reth_primitives::Address;

use crate::contract_bindings::poolmanager::PoolManager::Initialize;

pub type PoolId = usize;

pub type PoolIdWithDirection = (bool, PoolId);

/// just a placeholder type so i can implement the general architecture
#[derive(Debug, Clone, Copy)]
pub struct NewInitializedPool {
    pub currency_in:        Address,
    pub currency_in_index:  u16,
    pub currency_out:       Address,
    pub currency_out_index: u16,
    pub id:                 PoolIdWithDirection
}

impl From<Log<Initialize>> for NewInitializedPool {
    fn from(_value: Log<Initialize>) -> Self {
        // waiting on a bug fix from the alloy team
        todo!()
    }
}
