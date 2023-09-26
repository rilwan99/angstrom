use reth_codecs::{main_codec, Compact};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    RlpDecodable,
    RlpEncodable,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct Time {
    u: u128
}

impl Time {
    pub fn now() -> Self {
        todo!()
    }

    pub fn unix_epoch() -> Self {
        todo!()
    }
}
