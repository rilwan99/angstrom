use std::borrow::Borrow;

use ethers_core::types::transaction::eip712::TypedData;
use reth_primitives::bytes;
use reth_rlp::{Decodable, DecodeError, Encodable, Header};
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
pub struct Eip712(pub TypedData);

impl Encodable for Eip712 {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let coverted = serde_json::to_vec(&self.0).unwrap();
        let header = rlp_list_header(&coverted);

        header.encode(out);
        coverted.encode(out);
    }

    fn length(&self) -> usize {
        let coverted = serde_json::to_vec(&self.0).unwrap();
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
        let decoded_rlp: Vec<u8> = Vec::<u8>::decode(buf)?;

        Ok(serde_json::from_slice(&decoded_rlp).unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::eip712::bytes::BytesMut;

    #[test]
    fn test_encode_decode() {
        let json = serde_json::json!({
          "types": {
            "EIP712Domain": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "version",
                "type": "string"
              },
              {
                "name": "chainId",
                "type": "uint256"
              },
              {
                "name": "verifyingContract",
                "type": "address"
              }
            ],
            "OrderComponents": [
              {
                "name": "offerer",
                "type": "address"
              },
              {
                "name": "zone",
                "type": "address"
              },
              {
                "name": "offer",
                "type": "OfferItem[]"
              },
              {
                "name": "startTime",
                "type": "uint256"
              },
              {
                "name": "endTime",
                "type": "uint256"
              },
              {
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "name": "salt",
                "type": "uint256"
              },
              {
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "name": "counter",
                "type": "uint256"
              }
            ],
            "OfferItem": [
              {
                "name": "token",
                "type": "address"
              }
            ],
            "ConsiderationItem": [
              {
                "name": "token",
                "type": "address"
              },
              {
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "name": "recipient",
                "type": "address"
              }
            ]
          },
          "primaryType": "OrderComponents",
          "domain": {
            "name": "Seaport",
            "version": "1.1",
            "chainId": "1",
            "verifyingContract": "0x00000000006c3852cbEf3e08E8dF289169EdE581"
          },
          "message": {
            "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "offer": [
              {
                "token": "0xA604060890923Ff400e8c6f5290461A83AEDACec"
              }
            ],
            "startTime": "1658645591",
            "endTime": "1659250386",
            "zone": "0x004C00500000aD104D7DBd00e3ae0A5C00560C00",
            "zoneHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "salt": "16178208897136618",
            "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
            "totalOriginalConsiderationItems": "2",
            "counter": "0"
          }
        }
                );

        let typed_data: TypedData = serde_json::from_value(json).unwrap();
        let eip = Eip712(typed_data);
        let mut bytes = BytesMut::new();
        eip.encode(&mut bytes);
        let mut bytes = &(*bytes.freeze());

        let decode = Eip712::decode(&mut bytes).unwrap();

        assert_eq!(eip, decode);
    }
}
