use ethers_core::types::H256;
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::Order;

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<Order>,
    pub reserve_orders: Vec<Order>
}
