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

pub const ORDER_TYPE_HASH: H256 =
    H256(hex!("83a8a6ca1711f7deba4a7af7849103c6eeeea4e0a9d366881856d7e4e8e81365"));

/// Signed order with actual execution amounts.
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
pub struct Order {
    /// The order that was submitted though our endpoint
    pub order:             SubmittedOrder,
    /// The actual executed input amount.
    pub amount_in_actual:  U256,
    /// The actual executed output amount.
    pub amount_out_actual: U256
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
pub struct SubmittedOrder {
    /// The original order from the user.
    pub order:     OrderDetails,
    /// The user's EIP-712 signature of the Order.
    pub signature: Signature
}

impl SubmittedOrder {
    pub fn get_ethereum_address(&self) -> Address {
        todo!()
    }
}

/// The struct that the user signs using EIP-721.
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
pub struct OrderDetails {
    /// The user provided nonce, can only be used once.
    pub nonce:          U256,
    /// The order's type, can enable partial fills or fallible hooks.
    pub order_type:     OrderType,
    /// The input currency for the order.
    pub currency_in:    Currency,
    /// The output currency for the order.
    pub currency_out:   Currency,
    /// The (maximum) amount of input currency.
    pub amount_in:      u128,
    /// The minimum amount of output currency.
    pub amount_out_min: u128,
    /// The order cannot be executed after this timestamp.
    pub deadline:       U256,
    /// An optional user provided hook to run before collecting input
    /// payment.
    pub pre_hook:       Bytes,
    /// An optional user provided hook to run after paying the output.
    pub post_hook:      Bytes
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderType {
    User             = 0,
    Searcher         = 1,
    Limit            = 2,
    UserFallible     = 3,
    SearcherFallible = 4
}

impl Encodable for OrderType {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let byte: u8 = unsafe { std::mem::transmute(*self) };
        out.put_u8(byte)
    }
}
impl Decodable for OrderType {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        unsafe { std::mem::transmute(u8::decode(buf)) }
    }
}

impl AbiType for OrderType {
    fn param_type() -> ethers_core::abi::ParamType {
        ParamType::Bool
    }

    fn minimum_size() -> usize {
        1
    }
}

impl TokenizableItem for OrderType {}

impl AbiArrayType for OrderType {}

impl Tokenizable for OrderType {
    fn into_token(self) -> ethers_core::abi::Token {
        ethers_core::abi::Token::Bool(unsafe { std::mem::transmute(self) })
    }

    fn from_token(
        token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
        unreachable!("don't think we every abi decode this");
    }
}
