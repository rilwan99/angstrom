use alloy_primitives::{hex, Address, FixedBytes};
use revm_primitives::HashSet;

const ANGSTROM_ADDRESS: Address =
    Address(FixedBytes(hex!("680A025Da7b1be2c204D7745e809919bCE074026")));

pub struct AngstromTokens {
    current_tokens: HashSet<Address>
}

impl AngstromTokens {
    pub fn new<DB>(db: DB) -> Self {
        let mut this = Self { current_tokens: HashSet::default() };
        this
    }

    pub fn get_all_tokens(&self) -> &HashSet<Address> {}

    pub fn check_for_new_tokens<DB>(&mut self, db: DB) -> HashSet<Address> {
        todo!()
    }
}
