use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::Signature;
use crate::contract_bindings::PoolManager::PoolKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedLowerBound {
    pub lower_bound: LowerBound,
    pub signatures:  Vec<Signature>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerBound {
    pub searcher_bids: Vec<PoolValue>,
    pub volume:        Vec<PoolValue>
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolValue {
    pub pool_key: PoolKey,
    pub quantity: u128
}
