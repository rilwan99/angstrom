use std::collections::HashMap;

use alloy_primitives::Address;

#[allow(dead_code)]
pub struct AngstromPools(HashMap<[u8; 40], (bool, usize)>);

impl AngstromPools {
    #[allow(dead_code)]
    fn new() -> Self {
        AngstromPools(HashMap::new())
    }

    #[allow(dead_code)]
    fn add_mapping(
        &mut self,
        currency_in: Address,
        currency_out: Address,
        pool_id: usize,
        is_bid: bool
    ) {
        self.map
            .insert(self.get_key(currency_in, currency_out), (pool_id, is_bid));
    }

    #[allow(dead_code)]
    pub fn order_info(&self, currency_in: Address, currency_out: Address) -> Option<(usize, bool)> {
        self.map
            .get(&self.get_key(currency_in, currency_out))
            .copied()
    }

    #[allow(dead_code)]
    fn get_key(&self, currency_in: Address, currency_out: Address) -> FixedBytes<40> {
        FixedBytes::concat_const(currency_in.0, currency_out.0)
    }
}
