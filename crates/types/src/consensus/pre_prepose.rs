use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::{
    contract_bindings::Angstrom::PoolKey,
    on_chain::{Signature, SubmittedOrder}
};
// TODO: change searcher bid to best bid or something that communicates
// optimality of LP payoff.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpDecodable, RlpEncodable)]
pub struct PoolOrders {
    pub pool:         PoolKey,
    pub searcher_bid: SubmittedOrder,
    pub sorted_bids:  Vec<SubmittedOrder>,
    pub sorted_asks:  Vec<SubmittedOrder>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable)]
pub struct PreProposal {
    pub ethereum_height: u64,
    pub pre_bundle:      Vec<PoolOrders>,
    /// the signature is over the ethereum height and the bundle hash
    /// sign(ethereum_height | hash(pre_bundle))
    pub signature:       Signature
}
