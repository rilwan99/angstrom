use super::{RawBundle, Signature};

/// this is the underlying bundle that gets pushed on-chain
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
    pub bundle:     RawBundle,
    pub signatures: Vec<Signature>
}
