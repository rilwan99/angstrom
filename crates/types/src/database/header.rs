use bytes::BytesMut;
use reth_codecs::{main_codec, Compact};
use reth_primitives::{H256, H512};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::BlockCommit;
use crate::{
    consensus::{Time, Valid23Bundle},
    on_chain::{SimmedBundle, SubmissionBundle}
};

#[derive(
    Debug,
    Clone,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
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
    pub proposer_address: H512
}

#[main_codec]
#[derive(Debug, Clone, Copy, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId {
    // merkle root of header
    pub id: H256
}
