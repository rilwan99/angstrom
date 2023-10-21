use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{PoolKey, Signature};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct SignedLowerBound {
    pub lower_bound: LowerBound,
    pub signatures:  Vec<Signature>
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct LowerBound {
    pub searcher_bids: Vec<PoolValue>,
    pub volume:        Vec<PoolValue>
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    ethers_contract::EthAbiType,
    ethers_contract::EthAbiCodec,
)]
pub struct PoolValue {
    pub pool_key: PoolKey,
    pub quantity: u128
}
