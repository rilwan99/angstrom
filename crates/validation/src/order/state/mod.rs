use std::{collections::HashMap, sync::Arc};

use account::UserAccountProcessor;
use alloy::primitives::{Address, B256};
use angstrom_types::{
    primitive::{NewInitializedPool, PoolId},
    sol_bindings::{ext::RawPoolOrder, grouped_orders::AllOrders}
};
use db_state_utils::StateFetchUtils;
use matching_engine::cfmm::uniswap::{
    pool::EnhancedUniswapPool,
    pool_data_loader::DataLoader,
    tob::{calculate_reward, get_market_snapshot}
};
use parking_lot::RwLock;
use pools::PoolsTracker;

use super::{OrderValidation, OrderValidationResults};

pub mod account;
pub mod config;
pub mod db_state_utils;
pub mod pools;

/// State validation is all validation that requires reading from the Ethereum
/// database, these operations are:
/// 1) validating order nonce,
/// 2) checking token balances
/// 3) checking token approvals
/// 4) deals with possible pending state
pub struct StateValidation<Pools, Fetch> {
    /// tracks everything user related.
    user_account_tracker: Arc<UserAccountProcessor<Fetch>>,
    /// tracks all info about the current angstrom pool state.
    pool_tacker:          Arc<RwLock<Pools>>,
    /// keeps up-to-date with the on-chain pool
    uniswap_pools:
        Arc<HashMap<PoolId, tokio::sync::RwLock<EnhancedUniswapPool<DataLoader<PoolId>, PoolId>>>>
}

impl<Pools, Fetch> Clone for StateValidation<Pools, Fetch> {
    fn clone(&self) -> Self {
        Self {
            user_account_tracker: Arc::clone(&self.user_account_tracker),
            pool_tacker:          Arc::clone(&self.pool_tacker),
            uniswap_pools:        Arc::clone(&self.uniswap_pools)
        }
    }
}

impl<Pools: PoolsTracker, Fetch: StateFetchUtils> StateValidation<Pools, Fetch> {
    pub fn new(
        user_account_tracker: UserAccountProcessor<Fetch>,
        pools: Pools,
        uniswap_pools: Arc<
            HashMap<PoolId, tokio::sync::RwLock<EnhancedUniswapPool<DataLoader<PoolId>, PoolId>>>
        >
    ) -> Self {
        Self {
            pool_tacker: Arc::new(RwLock::new(pools)),
            user_account_tracker: Arc::new(user_account_tracker),
            uniswap_pools
        }
    }

    pub fn new_block(&self, completed_orders: Vec<B256>, address_changes: Vec<Address>) {
        self.user_account_tracker
            .prepare_for_new_block(address_changes, completed_orders)
    }

    pub fn handle_regular_order<O: RawPoolOrder + Into<AllOrders>>(
        &self,
        order: O,
        block: u64
    ) -> OrderValidationResults {
        let order_hash = order.order_hash();
        if !order.is_valid_signature() {
            return OrderValidationResults::Invalid(order_hash)
        }

        let Some(pool_info) = self.pool_tacker.read().fetch_pool_info_for_order(&order) else {
            return OrderValidationResults::Invalid(order_hash);
        };

        self.user_account_tracker
            .verify_order::<O>(order, pool_info, block)
            .map(|o: _| {
                OrderValidationResults::Valid(o.try_map_inner(|inner| Ok(inner.into())).unwrap())
            })
            .unwrap_or_else(|_| OrderValidationResults::Invalid(order_hash))
    }

    pub fn validate_state_of_regular_order(&self, order: OrderValidation, block: u64) {
        match order {
            OrderValidation::Limit(tx, order, _) => {
                let results = self.handle_regular_order(order, block);
                let _ = tx.send(results);
            }
            OrderValidation::Searcher(tx, order, _) => {
                let mut results = self.handle_regular_order(order, block);
                if let OrderValidationResults::Valid(ref mut order_with_storage) = results {
                    let tob_order = order_with_storage
                        .clone()
                        .try_map_inner(|inner| {
                            let AllOrders::TOB(order) = inner else { eyre::bail!("unreachable") };
                            Ok(order)
                        })
                        .expect("should be unreachable");
                    let pool_address = order_with_storage.pool_id;
                    let pool = self
                        .uniswap_pools
                        .get(&pool_address)
                        .map(|pool| pool.blocking_read())
                        .unwrap();
                    let market_snapshot = get_market_snapshot(pool).unwrap();
                    let rewards = calculate_reward(&tob_order, &market_snapshot).unwrap();
                    order_with_storage.tob_reward = rewards.total_reward;
                }

                let _ = tx.send(results);
            }
            _ => unreachable!()
        }
    }

    pub fn index_new_pool(&mut self, pool: NewInitializedPool) {
        self.pool_tacker.write().index_new_pool(pool);
    }
}
