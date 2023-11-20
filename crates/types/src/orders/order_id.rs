use alloy_primitives::{Address, B256};

use crate::primitive::PoolId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OrderId {
    pub address:  Address,
    /// Pool id
    pub pool_id:  PoolId,
    /// Hash of the order. Needed to check for inclusion
    pub hash:     B256,
    /// Nonce of the order
    pub nonce:    u64,
    /// when the order expires
    pub deadline: u128
}
