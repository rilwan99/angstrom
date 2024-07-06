use std::fmt::Debug;

use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{Decodable, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::primitive::{Order, PoolId};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable,
)]
pub struct OrderId {
    pub address:  Address,
    /// Pool id
    pub pool_id:  PoolId,
    /// Hash of the order. Needed to check for inclusion
    pub hash:     B256,
    /// Nonce of the order
    pub nonce:    U256,
    /// when the order expires
    pub deadline: U256,
    /// Order Location
    pub location: OrderLocation
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable,
)]
pub struct OrderPriorityData {
    pub price:  u128,
    pub volume: u128,
    pub gas:    u128
}

impl PartialOrd for OrderPriorityData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderPriorityData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price
            .cmp(&other.price)
            .then_with(|| self.volume.cmp(&other.volume))
            .then_with(|| self.gas.cmp(&other.gas))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OrderLocation {
    #[default]
    Limit,
    Searcher
}

impl Encodable for OrderLocation {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Self::Limit => 0_u8.encode(out),
            Self::Searcher => 1_u8.encode(out)
        }
    }

    fn length(&self) -> usize {
        u8::length(&1_u8)
    }
}

impl Decodable for OrderLocation {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let v = u8::decode(buf)?;
        match v {
            0 => Ok(Self::Limit),
            1 => Ok(Self::Searcher),
            _ => Err(alloy_rlp::Error::Custom("Unknown value decoding OrderLocation"))
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ValidationError {
    #[error("{0}")]
    StateValidationError(#[from] StateValidationError),
    #[error("bad signer")]
    BadSigner
}

#[derive(Debug, Error, Clone)]
pub enum StateValidationError {
    #[error("order: {0:?} nonce was invalid: {1}")]
    InvalidNonce(B256, U256),
    #[error("order: {0:?} did not have enough of {1:?}")]
    NotEnoughApproval(B256, Address),
    #[error("order: {0:?} did not have enough of {1:?}")]
    NotEnoughBalance(B256, Address)
}
