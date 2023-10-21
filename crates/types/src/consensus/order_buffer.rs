use ethers_core::types::H256;
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::contract_bindings::Order;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<Order>,
    pub reserve_orders: Vec<Order>
}
