use std::{
    collections::HashMap,
    ops::{Deref, DerefMut}
};

use alloy_primitives::{Address, U256};
use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpDecodableWrapper, RlpEncodable, RlpEncodableWrapper};
use reth_primitives::{recover_signer, Signature as ESignature};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::primitive::ANGSTROM_DOMAIN;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Signature(pub ESignature);

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CallerInfo {
    pub address:   Address,
    pub nonce:     u64,
    pub overrides: HashMap<Address, HashMap<U256, U256>>
}

impl Default for Signature {
    fn default() -> Self {
        Signature(ESignature::default())
    }
}

impl Encodable for Signature {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        self.0.encode(out);
    }

    fn length(&self) -> usize {
        self.0.payload_len()
    }
}
impl Decodable for Signature {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        let sig = ESignature::decode(buf)?;
        Ok(Signature(sig))
    }
}

impl Deref for Signature {
    type Target = ESignature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("failed to decode signature: {0:#?}")]
    UnableToDecodeSignature(String),
    #[error("failed to decode signer: {0:#?}")]
    UnableToRecoverSigner(String),
    #[error("message doesn't match")]
    MessageDoesntMatch
}
