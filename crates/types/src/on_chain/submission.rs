use bytes::Bytes;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{NonVanillaBundle, Signature, SignedLowerBound, SignedVanillaBundle, VanillaBundle};

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
pub struct SubmissionBundle {
    submission_type:    SubmissionType,
    underlying_payload: SubmissionPayload
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
pub struct LowerBoundBundle {
    pub bundle:             NonVanillaBundle,
    pub signed_lower_bound: SignedLowerBound
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SubmissionType {
    Vanilla    = 0,
    LowerBound = 1
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubmissionPayload {
    Vanilla(SignedVanillaBundle),
    LowerBound(LowerBoundBundle)
}
