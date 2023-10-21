use alloy_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use bytes::Bytes;
use ethers_core::{
    abi::{AbiArrayType, AbiType, ParamType, Token, Tokenizable, TokenizableItem},
    types::{H256, U256},
    utils::keccak256
};
use serde::{Deserialize, Serialize};

use super::{MevBundle, Signature, SignedLowerBound, SignedVanillaBundle, VanillaBundle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionBundle {
    submission_type:    SubmissionType,
    underlying_payload: SubmissionPayload
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerBoundBundle {
    pub bundle:             MevBundle,
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
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
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
        token: ethers_core::abi::Token
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
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        todo!()
    }
}
impl Decodable for SubmissionPayload {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
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
        token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
        unreachable!("don't think we every abi decode this");
    }
}
