use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::rpc::SignedLimitOrder;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, RlpEncodable, RlpDecodable)]
pub struct OrderBuffer {
    pub excess_orders:  Vec<SignedLimitOrder>,
    pub reserve_orders: Vec<SignedLimitOrder>
}
