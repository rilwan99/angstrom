use secp256k1::PublicKey;
use serde_json::Value;

use super::Time;

pub struct GenesisData {
    pub time:             Time,
    pub chain_id:         u64,
    pub initial_height:   u64,
    // TODO: fix placeholder
    pub consensus_params: u8,
    pub validators:       Vec<GenesisValidator>,
    pub app_hash:         Vec<u8>,
    pub app_state:        Value
}

pub struct GenesisValidator {
    pub address: PublicKey,
    pub power:   u64
}
