use std::collections::HashMap;

use alloy_primitives::{Address, FixedBytes, U256};
use angstrom_pools::AngstromPools;
use angstrom_types::{
    primitive::PoolId,
    sol_bindings::grouped_orders::{PoolOrder, RawPoolOrder}
};
use dashmap::DashMap;
use index_to_address::{AssetIndexToAddress, AssetIndexToAddressWrapper};

use super::config::{HashMethod, ValidationConfig};

pub mod angstrom_pools;
pub mod index_to_address;

pub trait PoolsTracker: Clone + Send + Unpin {
    /// Returns None if no pool is found
    fn fetch_pool_info_for_order<O: RawPoolOrder>(
        &self,
        order: O
    ) -> Option<(UserOrderPoolInfo, AssetIndexToAddressWrapper<O>)>;
}

#[derive(Debug, Clone)]
pub struct UserOrderPoolInfo {
    // token in for pool
    pub token:   Address,
    pub is_bid:  bool,
    pub pool_id: PoolId
}

#[derive(Clone)]
/// keeps track of all valid pools and the mappings of asset id to pool id
pub struct AngstromPoolsTracker {
    pub asset_index_to_address: AssetIndexToAddress,
    pub pools:                  AngstromPools
}

impl PoolsTracker for AngstromPoolsTracker {
    fn fetch_pool_info_for_order<O: RawPoolOrder>(
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
}

impl AngstromPoolsTracker {
    pub fn new(config: ValidationConfig) -> Self {
        let pools = config
            .pools
            .iter()
            .map(|pool| (AngstromPools::build_key(pool.token0, pool.token1), pool.pool_id))
            .collect::<DashMap<_, _>>();
        let angstrom_pools = AngstromPools::new(pools);

        let assets = config
            .asset_to_indexes
            .into_iter()
            .collect::<DashMap<_, _>>();
        let assets = AssetIndexToAddress::new(assets);

        Self { pools: angstrom_pools, asset_index_to_address: assets }
    }

    /// Get the token addresses for a pool specified by Uniswap PoolId.  By
    /// Uniswap convention, these token addresses will be sorted, therefore the
    /// return from this method is `Option<(t0, t1)>`
    pub fn get_pool_addresses(&self, poolid: PoolId) -> Option<(Address, Address)> {
        self.pools.get_addresses(poolid)
    }
}

#[cfg(test)]
pub mod pool_tracker_mock {
    use alloy_primitives::{Address, FixedBytes};
    use dashmap::DashMap;

    use super::*;

    #[derive(Clone, Default)]
    pub struct MockPoolTracker {
        asset_index_to_address: DashMap<u16, Address>,
        pools:                  DashMap<(Address, Address), PoolId>
    }

    impl MockPoolTracker {
        pub fn add_asset(&self, index: u16, address: Address) {
            self.asset_index_to_address.insert(index, address);
        }

        pub fn get_key(token0: Address, token1: Address) -> (Address, Address) {
            match token0.cmp(&token1) {
                std::cmp::Ordering::Greater => (token1, token0),
                std::cmp::Ordering::Less => (token0, token1),
                std::cmp::Ordering::Equal => panic!("Can't have a pool with equal tokens")
            }
        }

        pub fn add_pool(&self, token0: Address, token1: Address) -> PoolId {
            let key = Self::get_key(token0, token1);
            let pool_id = FixedBytes::random();
            match self.pools.entry(key) {
                dashmap::Entry::Occupied(_) => panic!("Tried to double-add a pool entry"),
                dashmap::Entry::Vacant(e) => {
                    e.insert(pool_id);
                }
            }
            pool_id
        }
    }

    impl PoolsTracker for MockPoolTracker {
        fn fetch_pool_info_for_order<O: RawPoolOrder>(
            &self,
            order: O
        ) -> Option<(UserOrderPoolInfo, AssetIndexToAddressWrapper<O>)> {
            let token_in = *self
                .asset_index_to_address
                .get(&order.get_token_in())?
                .value();
            let token_out = *self
                .asset_index_to_address
                .get(&order.get_token_out())?
                .value();

            let key = Self::get_key(token_in, token_out);
            let value = self.pools.get(&(key))?;
            let is_bid = token_in > token_out;
            let pool_id = value.value();
            let wrapped = AssetIndexToAddressWrapper { token_out, token_in, order };
            let info = UserOrderPoolInfo { pool_id: *pool_id, is_bid, token: token_in };

            Some((info, wrapped))
        }
    }
}
