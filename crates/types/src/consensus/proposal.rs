use bytes::BytesMut;
use reth_primitives::{keccak256, H256, H512};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::Block;
use crate::{
    consensus::Time,
    database::{BlockCommit, BlockHeader, BlockId},
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
        let mut buf = BytesMut::new();

        self.height.encode(&mut buf);
        self.round.encode(&mut buf);
        self.timestamp.encode(&mut buf);
        self.header.encode(&mut buf);
        self.evidence_data.encode(&mut buf);
        self.last_commit.encode(&mut buf);
        self.bundle.encode(&mut buf);
        let freeze = buf.freeze();

        keccak256(&freeze[..])
    }

    pub fn validate_signature(&self, public_key: H512) -> bool {
        validate_signature(&self.leader_bundle_signature, self.bundle_hash(), public_key)
            && validate_signature(&self.leader_block_signature, self.block_hash(), public_key)
    }
}

/// only gets sent if we agree with both the bundle and block data proposed
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct SignedLeaderProposal(pub Signature);
