use alloy_primitives::{Address, B256, U256};
use alloy_rlp_derive::{RlpEncodable, RlpDecodable};
use bytes::{Bytes, BytesMut};
use hex_literal::hex;
use serde::{Deserialize, Serialize};

use super::{Currency, ExternalStateSim, Signature};
use crate::contract_bindings::Angstrom::Order;

/// Signed order with actual execution amounts.

#[derive(Debug, Clone, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable)]
pub struct SubmittedOrder {
    /// The original order from the user.
    pub details:   Order,
    /// The user's EIP-712 signature of the Order.
    pub signature: Signature
}

impl SubmittedOrder {
    pub fn get_ethereum_address(&self) -> Address {
        todo!()
    }
}
