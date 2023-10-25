use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::{
    consensus::order_buffer::OrderBuffer,
    primitive::{
        Angstrom::{Bundle, LowerBound},
        Signature
    }
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct Proposal {
    pub ethereum_block:   u64,
    pub vanilla_bundle:   Bundle,
    pub lower_bound:      LowerBound,
    pub order_buffer:     Vec<OrderBuffer>,
    /// This signature is over (etheruem_block | hash(vanilla_bundle) |
    /// hash(lower_bound))
    pub leader_signature: Signature
}
