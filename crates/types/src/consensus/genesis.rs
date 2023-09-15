use secp256k1::PublicKey;
use serde_json::Value;

pub struct GenesisData {
    // TODO: move over to internal lib
    time:             u128,
    chain_id:         u64,
    initial_height:   u64,
    // TODO: fix placeholder
    consensus_params: u8,
    validators:       Vec<GenesisValidator>,
    app_hash:         Vec<u8>,
    app_state:        Value
}

pub struct GenesisValidator {
    address: PublicKey,
    power:   u64
}
