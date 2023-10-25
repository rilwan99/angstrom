use std::hash::Hash;

use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct SignedVanillaBundle {
    pub bundle:     Bundle,
    pub signatures: Signature
}

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
