use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use ethers_core::types::H256;
use serde::{Deserialize, Serialize};

use crate::on_chain::Signature;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Commit {
    pub block_height: u64,

    pub vanilla_bundle_hash: H256,
    pub lower_bound_hash:    H256,
    pub order_buffer_hash:   H256,
    /// This signature is (block_height | vanilla_bundle_hash |
    /// lower_bound_hash | order_buffer_hash)
    pub message_sig:         Signature,
    /// is default if none. We have to due this due to the rlp requirements
    pub vanilla_bundle_sig:  Signature,
    /// is default if none. We have to due this due to the rlp requirements
    pub lower_bound_sig:     Signature,
    /// is default if none. We have to due this due to the rlp requirements
    pub order_buffer_sig:    Signature
}
