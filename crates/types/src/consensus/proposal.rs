use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{Block, BlockCommit, BlockHeader, Time};
use crate::on_chain::{Signature, SimmedBundle};

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct LeaderProposal {
    pub height:    u64,
    pub round:     u64,
    pub timestamp: Time,

    // block details
    pub header:        BlockHeader,
    pub evidence_data: Vec<u8>,
    pub last_commit:   BlockCommit,

    // bundle
    pub bundle:           SimmedBundle,
    pub leader_signature: Signature
}

/// only gets sent if we agree with both the bundle and block data proposed
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct SignedLeaderProposal(pub Signature);
