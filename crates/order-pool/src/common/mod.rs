mod parked;
mod pending;

use alloy_primitives::{Address, B256};
pub use parked::*;
pub use pending::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OrderId {
    pub user_addr:  Address,
    /// Pool id
    pub pool_id:    Address,
    /// Hash of the order. Needed to check for inclusion
    pub order_hash: B256,
    /// Nonce of the order
    pub nonce:      u64,
    /// when the order expires
    pub expiry:     u128
}
