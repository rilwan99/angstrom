use std::hash::Hash;

use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use ethers_core::abi::{AbiArrayType, AbiType, ParamType, Tokenizable, TokenizableItem};
use revm::primitives::TxEnv;
use serde::{Deserialize, Serialize};

use crate::{
    primitive::Angstrom::{Bundle, LowerBound},
    Signature
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedLowerBound {
    pub lower_bound: LowerBound,
    pub signatures:  Vec<Signature>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionBundle {
    submission_type:    SubmissionType,
    underlying_payload: SubmissionPayload
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerBoundBundle {
    pub bundle:             Bundle,
    pub signed_lower_bound: SignedLowerBound
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubmissionType {
    Vanilla    = 0,
    LowerBound = 1
}

impl Encodable for SubmissionType {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let byte: u8 = unsafe { std::mem::transmute(*self) };
        out.put_u8(byte)
    }
}
impl Decodable for SubmissionType {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        unsafe { std::mem::transmute(u8::decode(buf)) }
    }
}

impl AbiType for SubmissionType {
    fn param_type() -> ethers_core::abi::ParamType {
        ParamType::Bool
    }

    fn minimum_size() -> usize {
        1
    }
}

impl TokenizableItem for SubmissionType {}

impl AbiArrayType for SubmissionType {}

impl Tokenizable for SubmissionType {
    fn into_token(self) -> ethers_core::abi::Token {
        ethers_core::abi::Token::Bool(unsafe { std::mem::transmute(self) })
    }

    fn from_token(
        _token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
        unreachable!("don't think we every abi decode this");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmissionPayload {
    Vanilla(SignedVanillaBundle),
    LowerBound(LowerBoundBundle)
}

impl Encodable for SubmissionPayload {
    fn encode(&self, _out: &mut dyn bytes::BufMut) {
        todo!()
    }
}
impl Decodable for SubmissionPayload {
    fn decode(_buf: &mut &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl AbiType for SubmissionPayload {
    fn param_type() -> ethers_core::abi::ParamType {
        todo!()
    }

    fn minimum_size() -> usize {
        120
    }
}

impl TokenizableItem for SubmissionPayload {}

impl AbiArrayType for SubmissionPayload {}

impl Tokenizable for SubmissionPayload {
    fn into_token(self) -> ethers_core::abi::Token {
        todo!()
    }

    fn from_token(
        _token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
        unreachable!("don't think we every abi decode this");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct SignedVanillaBundle {
    pub bundle:     Bundle,
    pub signatures: Signature
}

/*impl SignedVanillaBundle {
    pub fn new(orders: Vec<Order>, uniswap_data: UniswapData) -> anyhow::Result<Self> {
        let mev_bundle = orders
            .iter()
            .find(|order| !order.preHook.is_empty() || !order.postHook.is_empty());

        if mev_bundle.is_some() {
            anyhow::bail!("found a non_villa order: {:?}", mev_bundle);
        }

        Ok(Self { orders, uniswap_data })
    }
}*/

impl From<Bundle> for TxEnv {
    fn from(_value: Bundle) -> Self {
        todo!()
    }
}

// TODO: Finish type reorganisation to logically isolate them
#[derive(Debug, Clone)]
pub struct BestBundles {
    pub vanilla:     Option<Bundle>,
    pub lower_bound: Option<LowerBound>,
    pub mev_bundle:  Option<Bundle>
}

impl BestBundles {
    pub fn get_weight(&self) -> u128 {
        todo!()
    }
}
