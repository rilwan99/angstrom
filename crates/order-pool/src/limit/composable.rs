use std::collections::HashMap;

use angstrom_metrics::ComposableLimitOrderPoolMetrics;
use angstrom_types::{
    primitive::PoolId,
    sol_bindings::grouped_orders::{GroupedComposableOrder, OrderWithStorageData}
};

use super::{pending::PendingPool, LimitPoolError};

#[derive(Default)]
pub struct ComposableLimitPool {
    map:     HashMap<PoolId, PendingPool<GroupedComposableOrder>>,
    metrics: ComposableLimitOrderPoolMetrics
}

impl ComposableLimitPool {
    pub fn new(ids: &[PoolId]) -> Self {
        let map = ids.iter().map(|id| (*id, PendingPool::new())).collect();
        Self { map, metrics: ComposableLimitOrderPoolMetrics::default() }
    }

    pub fn add_order(
        &mut self,
        order: OrderWithStorageData<GroupedComposableOrder>
    ) -> Result<(), LimitPoolError> {
        let pool_id = order.pool_id;
        self.map
            .get_mut(&pool_id)
            .ok_or_else(|| LimitPoolError::NoPool(pool_id))?
            .add_order(order);

        self.metrics.incr_all_orders(pool_id, 1);

        Ok(())
    }

    pub fn remove_order(
        &mut self,
        pool_id: PoolId,
        tx_id: alloy_primitives::FixedBytes<32>
    ) -> Option<OrderWithStorageData<GroupedComposableOrder>> {
        self.map
            .get_mut(&pool_id)?
            .remove_order(tx_id)
            .map(|order| {
                self.metrics.decr_all_orders(pool_id, 1);
                order
            })
    }
}
