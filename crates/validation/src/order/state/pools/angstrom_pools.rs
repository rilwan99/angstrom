use std::collections::HashMap;

use alloy_primitives::{Address, FixedBytes};
use angstrom_types::primitive::{NewInitializedPool, PoolId, PoolIdWithDirection};
use dashmap::DashMap;

pub type PoolIdWithDirection = (bool, PoolId);

#[derive(Clone)]
pub struct AngstromPools(DashMap<FixedBytes<40>, PoolIdWithDirection>);

impl AngstromPools {
    pub fn new(setup: DashMap<FixedBytes<40>, PoolIdWithDirection>) -> Self {
        AngstromPools(setup)
    }

    pub fn order_info(
        &self,
        currency_in: Address,
        currency_out: Address
    ) -> Option<PoolIdWithDirection> {
        tracing::debug!(shit=?self.0);
        self.0
            .get(&Self::get_key(currency_in, currency_out))
            .map(|inner| *inner)
    }

    #[inline(always)]
    pub fn get_key(currency_in: Address, currency_out: Address) -> FixedBytes<40> {
        FixedBytes::concat_const(currency_in.0, currency_out.0)
    }

    pub fn new_pool(&mut self, pool: NewInitializedPool) {
        self.0
            .insert(self.get_key(pool.currency_in, pool.currency_out), pool.id);
    }
}
