use secp256k1::PublicKey;

use super::{BlockId, Time};
use crate::on_chain::Signature;

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BlockCommit {
    pub height:     u64,
    pub round:      u64,
    pub block_id:   BlockId,
    pub signatures: Vec<CommitSignature>
}

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BlockCommitSignature {
    pub leader_address: PublicKey,
    pub timestamp:      Time,
    pub signature:      Signature
}
