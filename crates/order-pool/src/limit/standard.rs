use std::collections::HashMap;

use angstrom_metrics::VanillaLimitOrderPoolMetrics;
use angstrom_types::{
    primitive::PoolId,
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};

use super::{parked::ParkedPool, pending::PendingPool};
use crate::limit::LimitPoolError;

#[derive(Default)]
pub struct LimitPool {
    pending_orders: HashMap<PoolId, PendingPool<GroupedVanillaOrder>>,
    parked_orders:  HashMap<PoolId, ParkedPool>,
    metrics:        VanillaLimitOrderPoolMetrics
}

impl LimitPool {
    pub fn new(ids: &[PoolId]) -> Self {
        let parked = ids.iter().map(|id| (*id, ParkedPool::new())).collect();
        let pending = ids.iter().map(|id| (*id, PendingPool::new())).collect();

        Self {
            parked_orders:  parked,
            pending_orders: pending,
            metrics:        VanillaLimitOrderPoolMetrics::default()
        }
    }

    pub fn add_order(
        &mut self,
        order: OrderWithStorageData<GroupedVanillaOrder>
    ) -> Result<(), LimitPoolError> {
        let pool_id = order.pool_id;
        let err = || LimitPoolError::NoPool(pool_id);

        if order.is_currently_valid {
            self.pending_orders
                .get_mut(&pool_id)
                .ok_or_else(err)?
                .add_order(order);
            self.metrics.incr_pending_orders(pool_id, 1);
        } else {
            self.parked_orders
                .get_mut(&pool_id)
                .ok_or_else(err)?
                .new_order(order);
            self.metrics.incr_parked_orders(pool_id, 1);
        }

        Ok(())
    }

    pub fn remove_order(
        &mut self,
        pool_id: PoolId,
        order_id: alloy_primitives::FixedBytes<32>
    ) -> Option<OrderWithStorageData<GroupedVanillaOrder>> {
        self.pending_orders
            .get_mut(&pool_id)
            .and_then(|pool| {
                pool.remove_order(order_id).map(|order| {
                    self.metrics.decr_pending_orders(pool_id, 1);
                    order
                })
            })
            .or_else(|| {
                self.parked_orders.get_mut(&pool_id).and_then(|pool| {
                    pool.remove_order(order_id).map(|order| {
                        self.metrics.decr_parked_orders(pool_id, 1);
                        order
                    })
                })
            })
    }

    pub fn get_all_orders(&self) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
        self.pending_orders
            .values()
            .flat_map(|p| p.get_all_orders())
            .collect()
    }
}
