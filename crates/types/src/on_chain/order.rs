use bytes::{Bytes, BytesMut};
use ethers_core::{
    abi::{AbiArrayType, AbiType, ParamType, Tokenizable, TokenizableItem},
    types::{transaction::eip712::TypedData, Address, H256, U256},
    utils::keccak256
};
use hex_literal::hex;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{Currency, ExternalStateSim, Signature};
use crate::contract_bindings::Order;

pub const ORDER_TYPE_HASH: H256 =
    H256(hex!("83a8a6ca1711f7deba4a7af7849103c6eeeea4e0a9d366881856d7e4e8e81365"));

/// Signed order with actual execution amounts.

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct SubmittedOrder {
    /// The original order from the user.
    pub details:   Order,
    /// The user's EIP-712 signature of the Order.
    pub signature: Signature
}

impl SubmittedOrder {
    pub fn get_ethereum_address(&self) -> Address {
        todo!()
    }
}
