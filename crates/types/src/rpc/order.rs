use std::collections::HashMap;

use alloy_primitives::{Address, U256};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::primitive::{Angstrom::Order, Signature};

/// Submitted order pre-processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable)]
pub struct SubmittedLimitOrder {
    /// The original order from the user.
    pub details:   Order,
    /// The user's EIP-712 signature of the Order.
    pub signature: Signature
}

impl SubmittedLimitOrder {
    pub fn get_ethereum_address(&self) -> Address {
        todo!()
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CallerInfo {
    pub address:   Address,
    pub nonce:     u64,
    pub overrides: HashMap<Address, HashMap<U256, U256>>
}
