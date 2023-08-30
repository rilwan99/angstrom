use reth_codecs::derive_arbitrary;
use reth_primitives::{bytes, Signature};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleSignature {
    sig: Signature
}

impl Encodable for BundleSignature {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.sig.encode(out)
    }

    fn length(&self) -> usize {
        self.sig.payload_len()
    }
}

/// This `Decodable` implementation only supports decoding rlp encoded
/// transactions as it's used by p2p.
///
/// CAUTION: this expects that the given buf contains rlp
impl Decodable for BundleSignature {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(BundleSignature { sig: Signature::decode(buf)? })
    }
}
