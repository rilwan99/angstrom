use std::collections::HashMap;

use alloy_primitives::{Address, FixedBytes};

#[allow(dead_code)]
pub struct PoolMapping {
    map: HashMap<FixedBytes<40>, (usize, bool)>
}

impl PoolMapping {
    #[allow(dead_code)]
    fn new() -> Self {
        PoolMapping { map: HashMap::new() }
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
    fn order_info(&self, currency_in: Address, currency_out: Address) -> Option<(usize, bool)> {
        self.map
            .get(&self.get_key(currency_in, currency_out))
            .copied()
    }

    #[allow(dead_code)]
    fn get_key(&self, currency_in: Address, currency_out: Address) -> FixedBytes<40> {
        FixedBytes::concat_const(currency_in.0, currency_out.0)
    }
}
