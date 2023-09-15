use secp256k1::PublicKey;

use super::{BlockId, Time};
use crate::on_chain::Signature;

pub struct Commit {
    pub height:     u64,
    pub round:      u64,
    pub block_id:   BlockId,
    pub signatures: Vec<CommitSignature>
}

pub struct CommitSignature {
    pub leader_address: PublicKey,
    pub timestamp:      Time,
    pub signature:      Signature
}
