use bytes::Bytes;
use reth_codecs::{main_codec, Compact};
use serde::{Deserialize, Serialize};

use super::BlockId;
use crate::consensus::{GuardSet, Time};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    // basic info
    pub chain_id:          u64,
    pub inital_height:     u64,
    pub last_block_height: u64,
    pub last_block_id:     BlockId,
    pub last_block_time:   Time,

    // guard info
    pub next_guards:                Vec<GuardSet>,
    pub guards:                     Vec<GuardSet>,
    pub last_guards:                Vec<GuardSet>,
    pub last_height_guards_changed: u64,

    pub consensus_params:                     u8,
    pub last_height_consensus_params_changed: u64,

    /// Merkle root of the results from executing prev block
    pub last_root: Bytes,
    pub app_hash:  Bytes
}
