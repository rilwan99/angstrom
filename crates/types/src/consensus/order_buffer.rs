use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use ethers_core::types::H256;
use serde::{Deserialize, Serialize};

use crate::on_chain::SubmittedOrder;

#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<SubmittedOrder>,
    pub reserve_orders: Vec<SubmittedOrder>
}
