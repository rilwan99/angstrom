use bytes::Bytes;
use reth_primitives::H256;

pub struct Rewards {
    hash:      H256,
    last_root: Bytes
}

pub struct RewardHeader {
    cumulative_lvr_bribe: u128
}
