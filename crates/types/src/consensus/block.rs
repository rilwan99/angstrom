use secp256k1::PublicKey;

use super::Commit;
use crate::on_chain::SimmedBundle;

pub struct Block {
    header:        BlockHeader,
    data:          SimmedBundle,
    // TODO move struct
    evidence_data: Vec<u8>,
    last_commit:   Commit
}

#[repr(transparent)]
pub struct BlockId(
    // merkle root of header
    [u8; 32]
);

pub struct BlockHeader {
    chain_id:      u64,
    height:        u64,
    // TODO
    time:          u128,
    last_block_id: BlockId,

    // hashes of stored data
    last_commit_hash: Vec<u8>,
    data_hash:        Vec<u8>,

    guard_hashes:      Vec<u8>,
    next_guard_hashes: Vec<u8>,
    consensus_hash:    Vec<u8>,
    app_hash:          Vec<u8>,

    last_result_hash: Vec<u8>,
    evidence_hash:    Vec<u8>,
    proposer_address: PublicKey
}
