use bytes::Bytes;
use reth_primitives::{H256, H512};
use secp256k1::PublicKey;

#[derive(Debug)]
pub struct Rewards {
    hash:      H256,
    header:    RewardHeader,
    last_root: Bytes
}

#[derive(Debug)]
pub struct RewardHeader {
    cumulative_lvr_bribe: u128,
    block_number:         u64,

    leader_pub_key: H512,
    attestations:   Vec<H512>
}
