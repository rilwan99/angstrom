use bytes::{Bytes, BytesMut};
use reth_codecs::{main_codec, Compact};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use super::BlockId;
use crate::consensus::{Block, GuardSet, Time};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, RlpDecodable, RlpEncodable,
)]
pub struct State {
    // basic info
    pub chain_id:          u64,
    pub inital_height:     u64,
    pub last_block_height: u64,
    pub last_block_id:     BlockId,
    pub last_block_time:   Time,

    // guard info
    pub next_guards:                GuardSet,
    pub guards:                     GuardSet,
    pub last_guards:                GuardSet,
    pub last_height_guards_changed: u64,

    pub consensus_params:                     u8,
    pub last_height_consensus_params_changed: u64,

    /// Merkle root of the results from executing prev block
    pub last_root: Bytes,
    pub app_hash:  Bytes
}

impl State {
    /// moves to the new block state. returns old state.
    pub fn transition(&mut self, block: Block) -> State {
        todo!()
    }
    
    fn hash_guards(guards: &GuardSet) -> Bytes {
        let mut buf = BytesMut::new();
        guards.encode(&mut buf);
        buf.freeze()
    }


    // TODO: what other fields do we need to verify here.
    // rest seems trivial
    pub fn verify_block(&self, block: &Block) -> bool {

        block.header.guard_hashes == Self::hash_guards(&self.guards) 
            && block.header.next_guard_hashes == Self::hash_guards(&self.next_guards)
            && block.header.last_block_id == self.last_block_id
            && block.header.consensus_hash == Bytes::from_static(&self.consensus_params.to_be_bytes())
    }
}
