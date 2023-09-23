use std::collections::HashSet;

use reth_primitives::{keccak256, H256, H512};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, SECP256K1
};
use serde::{Deserialize, Serialize};
use tracing::trace;

use super::{GuardSet, Time};
use crate::{
    get_public_key,
    on_chain::{RawBundle, RecoveryError, Signature, SimmedBundle}
};

/// propagated when we hit more than 2/3 votes for a bundle
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct Valid23Bundle {
    pub votes:  Bundle23Votes,
    pub bundle: SimmedBundle
}

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BundleVote {
    /// keccak256((bundle_hash, height, round));
    pub hash: H256,

    // metadata
    pub bundle_hash: H256,
    pub height:      u64,
    pub round:       u64,
    pub timestamp:   Time,

    pub signature: Signature
}

impl BundleVote {
    pub fn recover_public_key(&self) -> Result<H512, RecoveryError> {
        Ok(get_public_key(self.signature, self.hash))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct Bundle23Votes {
    /// keccak256((bundle_hash, height, round));
    pub hash: H256,

    pub bundle_hash: H256,
    pub height:      u64,
    pub round:       u64,
    pub timestamp:   Time,

    pub signatures: Vec<Signature>
}

impl Bundle23Votes {
    pub fn new(bundle_hash: H256, height: u64, round: u64, signatures: Vec<Signature>) -> Self {
        Self {
            bundle_hash,
            height,
            round,
            hash: keccak256((bundle_hash, height, round)),
            signatures,
            timestamp: Time::now()
        }
    }

    pub fn verify_signatures(&self, guards: &GuardSet) -> bool {
        // TODO: This seems wrong
        if self.signatures.len() % guards.len() < 67 {
            return false
        }

        // check for dups
        if self.signatures.iter().collect::<HashSet<_>>().len() != self.signatures.len() {
            return false
        }

        !self
            .signatures
            .iter()
            .map(|signature| {
                let key = get_public_key(signature, self.hash);

                Some(guards.contains_key(key))
            })
            .any(|y| y.is_none())
    }
}
