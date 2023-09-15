use secp256k1::PublicKey;

use super::BlockId;
use crate::on_chain::Signature;

pub struct Commit {
    height:     u64,
    round:      u64,
    block_id:   BlockId,
    signatures: Vec<CommitSignature>
}

pub struct CommitSignature {
    leader_address: PublicKey,
    // TODO: make type
    timestamp:      u128,
    signature:      Signature
}
