use reth_primitives::{bytes, PeerId, Signature, H256, H512};
use reth_rlp::{Decodable, DecodeError, Encodable};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, SECP256K1
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::trace;

#[derive(Debug, Error)]
pub enum RecoveryError {
    #[error("failed to decode signature: {0:#?}")]
    UnableToDecodeSignature(String),
    #[error("failed to decode signer: {0:#?}")]
    UnableToRecoverSigner(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatchSignature {
    /// the signature of the bundle
    pub sig:  Signature,
    /// is the hash of the signature
    pub hash: H256
}
impl BatchSignature {
    pub fn recover_key(&self) -> Result<PeerId, RecoveryError> {
        let sig = RecoverableSignature::from_compact(
            &self.sig.to_bytes()[0..64],
            RecoveryId::from_i32(self.sig.to_bytes()[64] as i32)
                .map_err(|e| RecoveryError::UnableToDecodeSignature(e.to_string()))?
        )
        .map_err(|err| RecoveryError::UnableToDecodeSignature(err.to_string()))?;

        trace!(?sig, "Validating Signature -- RECOVERING PUBLIC KEY");
        // secp256k1 public key
        SECP256K1
            .recover_ecdsa(
                &Message::from_slice(&self.hash[..32])
                    .map_err(|e| RecoveryError::UnableToRecoverSigner(e.to_string()))?,
                &sig
            )
            .map(|public_key| H512::from_slice(&public_key.serialize_uncompressed()[1..]))
            .map_err(|err| RecoveryError::UnableToRecoverSigner(err.to_string()))
    }
}

impl Encodable for BatchSignature {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.sig.encode(out);
        self.hash.encode(out)
    }

    fn length(&self) -> usize {
        self.sig.payload_len() + self.hash.length()
    }
}

/// This `Decodable` implementation only supports decoding rlp encoded
/// transactions as it's used by p2p.
///
/// CAUTION: this expects that the given buf contains rlp
impl Decodable for BatchSignature {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(BatchSignature { sig: Signature::decode(buf)?, hash: H256::decode(buf)? })
    }
}
