use bytes::BytesMut;
use reth_codecs::{main_codec, Compact};
use reth_primitives::{Bytes, H512};
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::header::BlockId;
use crate::{consensus::Time, on_chain::Signature};

#[main_codec]
#[derive(Debug, Clone, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct BlockCommit {
    pub height:     u64,
    pub round:      u64,
    pub signatures: Vec<BlockCommitSignature>,
    pub block_id:   BlockId
}

#[main_codec]
#[derive(Debug, Clone, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash, Default)]
pub struct BlockCommitSignature {
    pub signature:      Signature,
    pub timestamp:      u128,
    pub leader_address: Bytes
}
