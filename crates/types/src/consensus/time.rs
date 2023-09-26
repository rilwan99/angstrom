use reth_codecs::{main_codec, Compact};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[main_codec]
#[derive(Debug, Clone, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time {}

impl Time {
    pub fn now() -> Self {
        todo!()
    }

    pub fn unix_epoch() -> Self {
        todo!()
    }
}
