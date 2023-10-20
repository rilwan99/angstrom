use ethers_core::types::H256;
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::Signature;

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct ProposalCommit {
    block_height: u64,

    vanilla_bundle_hash: H256,
    lower_bound_hash:    H256,
    /// This signature is (block_height | vanilla_bundle_hash |
    /// lower_bound_hash)
    message_sig:         Signature,
    /// if nil vote then None
    vanilla_bundle_sig:  Option<Signature>,
    /// if nil vote then None
    lower_bound_sig:     Option<Signature>
}
