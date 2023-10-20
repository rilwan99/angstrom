use bytes::Bytes;
use ethers_core::{
    abi::{AbiArrayType, AbiType, ParamType, Tokenizable, TokenizableItem},
    types::{H256, U256}
};
use hex_literal::hex;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{Currency, Signature};

const ORDER_TYPE_HASH: H256 =
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
    /// The original order from the user.
    order:             OrderDetails,
    /// The user's EIP-712 signature of the Order.
    signature:         Signature,
    /// The actual executed input amount.
    amount_in_actual:  U256,
    /// The actual executed output amount.
    amount_out_actual: U256
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

impl TryInto<HookSim> for OrderDetails {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<HookSim, Self::Error> {
        let mut msg = Vec::new();
        msg.extend(USER_TYPE_HASH.to_fixed_bytes());
        msg.extend(self.order.token_in.to_fixed_bytes());
        msg.extend(self.order.token_out.to_fixed_bytes());
        msg.extend(self.order.amount_in.to_be_bytes());
        msg.extend(self.order.amount_out_min.to_be_bytes());

        let mut deadbuf = BytesMut::new();
        self.order.deadline.to_big_endian(&mut deadbuf);
        msg.extend(deadbuf.to_vec());
        let mut bribe = BytesMut::new();
        self.order.gas_cap.to_big_endian(&mut bribe);
        msg.extend(bribe.to_vec());
        msg.extend(keccak256(&self.order.pre_hook));
        msg.extend(keccak256(&self.order.post_hook));

        let digest = keccak256(msg);
        let addr = self.signature.recover(digest)?;

        Ok(HookSim {
            tx: super::SearcherOrUser::User(self.clone()),
            pre_hook: self.order.pre_hook,
            amount_in_req: self.order.amount_in,
            amount_in_token: self.order.token_in,
            post_hook: self.order.post_hook,
            amount_out_lim: self.order.amount_out_min,
            amount_out_token: self.order.token_out,
            addr
        })
    }
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
