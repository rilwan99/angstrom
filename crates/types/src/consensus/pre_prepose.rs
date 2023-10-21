use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::{
    contract_bindings::{Angstrom::Order, PoolManager::PoolKey},
    on_chain::Signature
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PoolOrders {
    pub pool:         PoolKey,
    pub searcher_bid: Order,
    pub sorted_bids:  Vec<Order>,
    pub sorted_asks:  Vec<Order>
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct PreProposal {
    pub ethereum_height: u64,
    pub pre_bundle:      Vec<PoolOrders>,
    /// the signature is over the ethereum height and the bundle hash
    /// sign(ethereum_height | hash(pre_bundle))
    pub signature:       Signature
}
