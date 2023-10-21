use ethers_core::types::H256;
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::SubmittedOrder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<SubmittedOrder>,
    pub reserve_orders: Vec<SubmittedOrder>
}
