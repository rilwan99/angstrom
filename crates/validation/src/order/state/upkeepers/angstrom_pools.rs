use std::collections::HashMap;

use alloy_primitives::{Address, FixedBytes};

#[allow(dead_code)]
pub struct AngstromPools(HashMap<FixedBytes<40>, (bool, usize)>);

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
        self.0
            .insert(self.get_key(currency_in, currency_out), (is_bid, pool_id));
    }

    #[allow(dead_code)]
    pub fn order_info(&self, currency_in: Address, currency_out: Address) -> Option<(bool, usize)> {
        self.0
            .get(&self.get_key(currency_in, currency_out))
            .copied()
    }

    #[allow(dead_code)]
    fn get_key(&self, currency_in: Address, currency_out: Address) -> FixedBytes<40> {
        FixedBytes::concat_const(currency_in.0, currency_out.0)
    }
}
