use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::{BlockCommit, Time, Valid23Bundle};
use crate::on_chain::{SimmedBundle, SubmissionBundle};

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct Block {
    pub header:        BlockHeader,
    pub data:          SubmissionBundle,
    // TODO move struct
    pub evidence_data: Vec<u8>,
    pub last_commit:   BlockCommit
}

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash,
)]
pub struct BlockId(
    // merkle root of header
    pub [u8; 32]
);

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash,
)]
pub struct BlockHeader {
    pub chain_id:      u64,
    pub height:        u64,
    pub time:          Time,
    pub last_block_id: BlockId,

    pub ethereum_height: u64,

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
