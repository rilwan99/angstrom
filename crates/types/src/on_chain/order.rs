use ethers_core::{
    abi::{AbiArrayType, AbiType, Tokenizable, TokenizableItem},
    types::{Bytes, U256}
};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{Currency, Signature};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        let byte: u8 = unsafe { std::mem::transmute(self) };
    }
}
impl Decodable for OrderType {}

impl AbiType for OrderType {
    fn param_type() -> ethers_core::abi::ParamType {
        todo!()
    }

    fn minimum_size() -> usize {
        72
    }
}

impl TokenizableItem for OrderType {}

impl AbiArrayType for OrderType {}

impl Tokenizable for OrderType {
    fn into_token(self) -> ethers_core::abi::Token {
        todo!()
    }

    fn from_token(
        token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
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
    nonce:          U256,
    /// The order's type, can enable partial fills or fallible hooks.
    order_type:     OrderType,
    /// The input currency for the order.
    currency_in:    Currency,
    /// The output currency for the order.
    currency_out:   Currency,
    /// The (maximum) amount of input currency.
    amount_in:      u128,
    /// The minimum amount of output currency.
    amount_out_min: u128,
    /// The order cannot be executed after this timestamp.
    deadline:       U256,
    /// An optional user provided hook to run before collecting input
    /// payment.
    pre_hook:       Bytes,
    /// An optional user provided hook to run after paying the output.
    post_hook:      Bytes
}

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
    /// The original order from the user.
    order:             OrderDetails,
    /// The user's EIP-712 signature of the Order.
    signature:         Signature,
    /// The actual executed input amount.
    amount_in_actual:  U256,
    /// The actual executed output amount.
    amount_out_actual: U256
}
