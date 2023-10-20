use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::{LowerBound, Signature, VanillaBundle};

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct LeaderProposal {
    pub ethereum_block:   u64,
    pub vanilla_bundle:   VanillaBundle,
    pub lower_bound:      LowerBound,
    /// This signature is over (etheruem_block | hash(vanilla_bundle) |
    /// hash(lower_bound))
    pub leader_signature: Signature
}
