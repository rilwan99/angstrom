use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct Time {}

impl Time {
    pub fn now() -> Self {
        todo!()
    }

    pub fn unix_epoch() -> Self {
        todo!()
    }
}
