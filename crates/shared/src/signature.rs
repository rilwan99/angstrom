use reth_primitives::{bytes, Signature};
use reth_rlp::{Decodable, DecodeError, Encodable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatchSignature {
    sig: Signature
}

impl Encodable for BatchSignature {
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
impl Decodable for BatchSignature {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(BatchSignature { sig: Signature::decode(buf)? })
    }
}
