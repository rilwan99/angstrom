use alloy_primitives::{hex, Address, U256};
use reth_provider::StateProvider;

// TODO: move to actual angstrom address
const ANGSTROM_ADDRESS: Address =
    Address(alloy_primitives::FixedBytes(hex!("680A025Da7b1be2c204D7745e809919bCE074026")));

/// The nonce location for quick db lookup
const ANGSTROM_NONCE_LOCATION: U256 = U256::ZERO;

pub struct Nonces;

impl Nonces {
    pub fn is_valid_nonce<DB: StateProvider>(
        &self,
        user: Address,
        nonce: U256,
        db: Arc<RevmLRU<DB>>
    ) -> bool {
        todo!()
    }
}
