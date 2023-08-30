use std::{borrow::Borrow, sync::Arc};

use ethers_core::types::transaction::eip712::TypedData;
use reth_primitives::bytes;
use reth_rlp::{
    length_of_length, BufMut, Decodable, DecodeError, Encodable, Header, RlpDecodable, RlpEncodable
};
use serde::{Deserialize, Serialize};

fn rlp_list_header<E, K>(v: &[K]) -> Header
where
    E: Encodable + ?Sized,
    K: Borrow<E>
{
    let mut h = Header { list: true, payload_length: 0 };
    for x in v {
        h.payload_length += x.borrow().length();
    }
    h
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Eip712 {
    typed_data: TypedData
}

impl Encodable for Eip712 {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let coverted = serde_json::to_vec(&self.typed_data).unwrap();
        let header = rlp_list_header(&coverted);

        header.encode(out);
        coverted.encode(out);
    }

    fn length(&self) -> usize {
        let coverted = serde_json::to_vec(&self.typed_data).unwrap();
        let header = rlp_list_header(&coverted);
        header.payload_length
    }
}

impl Decodable for Eip712 {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let header = Header::decode(buf)?;
        if !header.list {
            return Err(DecodeError::NonCanonicalSize)
        }

        let length = header.payload_length;

        let decoded_rlp: Vec<u8> = Vec::<u8>::decode(buf)?;

        Ok(serde_json::from_slice(&decoded_rlp).unwrap())
    }
}
