use std::{ops::Deref, sync::Arc};

use alloy_primitives::{Address, U256};
use angstrom_pools::AngstromPools;
use angstrom_types::{
    primitive::{NewInitializedPool, PoolIdWithDirection},
    sol_bindings::grouped_orders::{PoolOrder, RawPoolOrder}
};
use index_to_address::{AssetIndexToAddress, AssetIndexToAddressWrapper};
use parking_lot::RwLock;

use super::config::ValidationConfig;

pub mod angstrom_pools;
pub mod index_to_address;

#[derive(Debug, Clone)]
pub struct UserOrderPoolInfo {
    // token in for pool
    pub token:   Address,
    pub is_bid:  bool,
    pub pool_id: usize
}

#[derive(Clone)]
pub struct AngstromPoolsTracker(Arc<RwLock<AngstromPoolsTrackerInner>>);

impl AngstromPoolsTracker {
    pub fn new(config: ValidationConfig) -> Self {
        Self(Arc::new(RwLock::new(AngstromPoolsTrackerInner::new(config))))
    }
}

impl AngstromPoolsTracker {
    pub fn read_with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&AngstromPoolsTrackerInner) -> R
    {
        let this = self.0.read_arc();
        f(&this)
    }

    pub fn write_with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut AngstromPoolsTrackerInner) -> R
    {
        let mut this = self.0.write_arc();
        f(&mut this)
    }
}

/// keeps track of all valid pools and the mappings of asset id to pool id
pub struct AngstromPoolsTrackerInner {
    pub asset_index_to_address: AssetIndexToAddress,
    pub pools:                  AngstromPools
}

impl AngstromPoolsTrackerInner {
    fn new(config: ValidationConfig) -> Self {
        todo!()
    }

    /// None if no pool was found
    pub fn fetch_pool_info_for_order<O: RawPoolOrder>(
        &self,
        order: O
    ) -> Option<(UserOrderPoolInfo, AssetIndexToAddressWrapper<O>)> {
        let wrapped = self.asset_index_to_address.wrap(order)?;
        let (is_bid, pool_id) = self
            .pools
            .order_info(wrapped.token_in(), wrapped.token_out())?;

        let user_info = UserOrderPoolInfo { pool_id, is_bid, token: wrapped.token_in() };

        Some((user_info, wrapped))
    }

    pub fn index_new_pool(&mut self, pool: NewInitializedPool) {
        self.set_pool(pool);
        self.set_assets(pool);
    }

    fn set_pool(&mut self, pool: NewInitializedPool) {
        self.pools.new_pool(pool)
    }

    fn set_assets(&mut self, pool: NewInitializedPool) {
        self.asset_index_to_address.try_set_new_pool_assets(pool);
    }
}
