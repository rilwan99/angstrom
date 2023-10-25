use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::rpc::SubmittedLimitOrder;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<SubmittedLimitOrder>,
    pub reserve_orders: Vec<SubmittedLimitOrder>
}
