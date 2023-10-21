use reth_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::on_chain::{Signature, VanillaBundle};

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq)]
pub struct PrePreposeBundle {
    pub ethereum_height: u64,
    pub bundle:          VanillaBundle,
    /// the signature is over the ethereum height and the bundle hash
    /// sign(ethereum_height | hash(bundle))
    pub signature:       Signature
}
