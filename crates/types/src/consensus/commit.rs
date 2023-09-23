use reth_primitives::H512;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::{BlockId, Time};
use crate::on_chain::Signature;

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BlockCommit {
    pub height:     u64,
    pub round:      u64,
    pub block_id:   BlockId,
    pub signatures: Vec<BlockCommitSignature>
}

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BlockCommitSignature {
    pub leader_address: H512,
    pub timestamp:      Time,
    pub signature:      Signature
}
