use reth_codecs::derive_arbitrary;
use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[derive_arbitrary(rlp)]
#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct TeeAddress {
    url:  String,
    port: u16
}
