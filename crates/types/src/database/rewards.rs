use bytes::Bytes;
use reth_codecs::{main_codec, Compact};
use reth_primitives::{H256, H512};
use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardHeader {
    hash: H256,

    cumulative_lvr_bribe: u128,
    block_number:         u64,

    leader_pub_key: H512,
    attestations:   Vec<H512>
}
