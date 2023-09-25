use bytes::BytesMut;
use reth_primitives::H512;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

use super::{BlockCommit, Time, Valid23Bundle};
use crate::on_chain::{SimmedBundle, SubmissionBundle};
use crate::dtabase::BlockHeader,

#[derive(Debug, Clone, Serialize, Deserialize, RlpDecodable, RlpEncodable, PartialEq, Eq, Hash)]
pub struct Block {
    pub header:        BlockHeader,
    pub data:          SubmissionBundle,
    // TODO move struct
    pub evidence_data: Vec<u8>,
    pub last_commit:   BlockCommit
}

