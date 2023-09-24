use reth_primitives::{keccak256, H256, H512};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::{Block, BlockCommit, BlockHeader, Time};
use crate::{
    on_chain::{Signature, SimmedBundle},
    validate_signature
};

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
    pub bundle:                  SimmedBundle,
    pub leader_bundle_signature: Signature,
    pub leader_block_signature:  Signature
}

impl LeaderProposal {
    pub fn bundle_hash(&self) -> H256 {
        self.bundle.hash()
    }

    fn block_hash(&self) -> H256 {
        keccak256(vec![
            self.height.to_be_bytes(),
            self.round.to_be_bytes(),
            self.timestamp.into_bytes(),
            self.header.into_bytes(),
            self.evidence_data,
            self.last_commit.to_bytes(),
            self.bundle.hash(),
        ])
    }

    pub fn validate_signature(&self, leader_pub_key: H512) -> bool {
        validate_signature(self.leader_bundle_signature, self.bundle_hash(), public_key)
            && validate_signature(self.leader_block_signature, self.block_hash(), public_key)
    }
}

/// only gets sent if we agree with both the bundle and block data proposed
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct SignedLeaderProposal(pub Signature);
