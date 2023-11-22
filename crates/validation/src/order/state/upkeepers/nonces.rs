use alloy_primitives::{Address, U256};

// TODO: move to actual angstrom address
const ANGSTROM_ADDRESS: Address =
    Address(alloy_primitives::FixedBytes(hex!("680A025Da7b1be2c204D7745e809919bCE074026")));

/// The nonce location for quick db lookup
const ANGSTROM_NONCE_LOCATION: U256 = U256::ZERO;

pub struct Nonces;


