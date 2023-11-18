mod parked;
mod pending;

pub use parked::*;
pub use pending::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderPrice {
    price: U256
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TransactionId {
    pub user_addr:  Address,
    /// Pool id
    pub pool_id:    PoolId,
    /// Hash of the order. Needed to check for inclusion
    pub order_hash: B256,
    /// Nonce of the order
    pub nonce:      u64,
    /// when the order expires
    pub expiry:     u128
}
