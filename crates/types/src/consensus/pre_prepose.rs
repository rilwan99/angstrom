use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::{
    primitive::{Angstrom::PoolKey, Signature},
    rpc::SubmittedLimitOrder
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpDecodable, RlpEncodable)]
pub struct PoolOrders {
    pub pool:         PoolKey,
    pub searcher_bid: SubmittedLimitOrder,
    pub sorted_bids:  Vec<SubmittedLimitOrder>,
    pub sorted_asks:  Vec<SubmittedLimitOrder>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable)]
pub struct PreProposal {
    pub ethereum_height: u64,
    pub pre_bundle:      Vec<PoolOrders>,
    /// the signature is over the ethereum height and the bundle hash
    /// sign(ethereum_height | hash(pre_bundle))
    pub signature:       Signature
}
