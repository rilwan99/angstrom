use std::hash::Hash;

use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use revm::primitives::TxEnv;
use serde::{Deserialize, Serialize};

use crate::{
    primitive::Angstrom::{Bundle, LowerBound},
    Signature
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedLowerBound {
    pub lower_bound: LowerBound,
    pub signatures:  Vec<Signature>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerBoundBundle {
    pub bundle:             Bundle,
    pub signed_lower_bound: SignedLowerBound
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RlpEncodable, RlpDecodable)]
pub struct SignedVanillaBundle {
    pub bundle:     Bundle,
    pub signatures: Signature
}

impl From<Bundle> for TxEnv {
    fn from(_value: Bundle) -> Self {
        todo!()
    }
}

// TODO: Finish type reorganisation to logically isolate them
#[derive(Debug, Clone)]
pub struct BestBundles {
    pub vanilla:     Option<Bundle>,
    pub lower_bound: Option<LowerBound>,
    pub mev_bundle:  Option<Bundle>
}

impl BestBundles {
    pub fn get_weight(&self) -> u128 {
        todo!()
    }
}
