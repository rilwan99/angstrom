use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::{Commit, Time};
use crate::on_chain::SimmedBundle;

pub struct Block {
    pub header:        BlockHeader,
    pub data:          SimmedBundle,
    // TODO move struct
    pub evidence_data: Vec<u8>,
    pub last_commit:   Commit
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct BlockId(
    // merkle root of header
    pub [u8; 32]
);

pub struct BlockHeader {
    pub chain_id:      u64,
    pub height:        u64,
    pub time:          Time,
    pub last_block_id: BlockId,

    // hashes of stored data
    pub last_commit_hash: Vec<u8>,
    pub data_hash:        Vec<u8>,

    pub guard_hashes:      Vec<u8>,
    pub next_guard_hashes: Vec<u8>,
    pub consensus_hash:    Vec<u8>,
    pub app_hash:          Vec<u8>,

    pub last_result_hash: Vec<u8>,
    pub evidence_hash:    Vec<u8>,
    pub proposer_address: PublicKey
}
