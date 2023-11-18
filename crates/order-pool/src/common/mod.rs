mod size;

use alloy_primitives::{Address, B256};
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct OrderId {
    pub address:  Address,
    /// Pool id
    pub pool_id:  Address,
    /// Hash of the order. Needed to check for inclusion
    pub hash:     B256,
    /// Nonce of the order
    pub nonce:    u64,
    /// when the order expires
    pub deadline: u128
}
