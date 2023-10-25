use std::{
    collections::HashMap,
    ops::{Add, Deref, DerefMut}
};

use alloy_primitives::{Address, U256};
use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use alloy_sol_types::SolStruct;
use reth_primitives::{recover_signer, Signature as ESignature};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::primitive::{Angstrom::Order, Signature, ANGSTROM_DOMAIN};

/// Submitted order pre-processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, RlpEncodable, RlpDecodable)]
pub struct SignedLimitOrder {
    /// The original order from the user.
    pub details:   Order,
    /// The user's EIP-712 signature of the Order.
    pub signature: Signature
}

impl SignedLimitOrder {
    pub fn recover_signer(&self) -> Option<Address> {
        let hash = self.details.eip712_signing_hash(&ANGSTROM_DOMAIN);
        Some(self.signature.0.recover_signer(hash))?
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CallerInfo {
    pub address:   Address,
    pub nonce:     u64,
    pub overrides: HashMap<Address, HashMap<U256, U256>>
}
