use std::ops::{Deref, DerefMut};

use alloy_rlp::{Decodable, Encodable, Error};
use bytes::{Bytes, BytesMut};
use ethers_core::{
    abi::{AbiArrayType, AbiType, ParamType, Token, Tokenizable, TokenizableItem},
    types::{Signature as ESignature, H256, U256}
};
use reth_codecs::{main_codec, Compact};
use reth_primitives::{PeerId, H512};
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, SECP256K1
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ethers_contract::EthAbiCodec,
)]
#[repr(transparent)]
pub struct Signature(pub ESignature);

impl Default for Signature {
    fn default() -> Self {
        Signature(ESignature {
            r: Default::default(),
            s: Default::default(),
            v: Default::default()
        })
    }
}

impl AbiType for Signature {
    fn param_type() -> ethers_core::abi::ParamType {
        ParamType::Bytes
    }

    fn minimum_size() -> usize {
        72
    }
}

impl TokenizableItem for Signature {}

impl AbiArrayType for Signature {}

impl Tokenizable for Signature {
    fn into_token(self) -> ethers_core::abi::Token {
        let mut buf = BytesMut::new();
        U256::from(self.v).to_big_endian(&mut buf);
        self.r.to_big_endian(&mut buf);
        self.s.to_big_endian(&mut buf);
        buf.freeze().into_token()
    }

    fn from_token(
        token: ethers_core::abi::Token
    ) -> Result<Self, ethers_core::abi::InvalidOutputType>
    where
        Self: Sized
    {
        let Token::Bytes(bytes) = token else {
            return Err(ethers_core::abi::InvalidOutputType("not bytes".to_string()))
        };
        let v = U256::from_big_endian(&bytes[0..32]).as_u64();
        let r = U256::from_big_endian(&bytes[32..63]);
        let s = U256::from_big_endian(&bytes[64..95]);

        Ok(Self(ESignature { v, s, r }))
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

impl Encodable for Signature {
    fn encode(&self, out: &mut dyn open_fastrlp::BufMut) {
        open_fastrlp::Encodable::encode(&self.0, out);
    }

    fn length(&self) -> usize {
        open_fastrlp::Encodable::length(&self.0)
    }
}
impl Decodable for Signature {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        let sig = open_fastrlp::Decodable::decode(buf)
            .map_err(|_| Error::Custom("failed to decode sig"))?;

        Ok(Signature(sig))
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
