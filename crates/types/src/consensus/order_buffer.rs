use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::SubmittedOrder;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<SubmittedOrder>,
    pub reserve_orders: Vec<SubmittedOrder>
}
