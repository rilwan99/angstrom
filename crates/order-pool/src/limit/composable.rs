use std::collections::HashMap;

use guard_types::{
    orders::{OrderId, OrderPriorityData, PooledComposableOrder, PooledLimitOrder, ValidatedOrder},
    primitive::PoolId
};

use super::{pending::PendingPool, LimitPoolError};
use crate::common::{BidAndAsks, ValidOrder};

pub struct ComposableLimitPool<C: PooledComposableOrder + PooledLimitOrder>(
    HashMap<PoolId, PendingPool<C>>
);

impl<O: PooledComposableOrder + PooledLimitOrder> ComposableLimitPool<O>
where
    O: PooledComposableOrder<ValidationData = OrderPriorityData>
{
    pub fn new() -> Self {
        todo!()
    }

    #[allow(dead_code)]
    pub fn add_order(&mut self, order: ValidOrder<O>) -> Result<(), LimitPoolError> {
        let id: OrderId = order.clone().into();
        self.0
            .get_mut(&id.pool_id)
            .map(|pool| pool.add_order(order))
            .ok_or_else(|| LimitPoolError::NoPool(id.pool_id))
    }

    #[allow(dead_code)]
    pub fn remove_order(&mut self, tx_id: &OrderId) -> Option<ValidOrder<O>> {
        self.0.get_mut(&tx_id.pool_id)?.remove_order(tx_id.hash)
    }

    #[allow(dead_code)]
    pub fn fetch_all_pool_orders(&self, id: &PoolId) -> Vec<&ValidOrder<O>> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_all_orders())
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn fetch_all_pool_bids(&self, id: &PoolId) -> Vec<&ValidOrder<O>> {
        self.0.get(id).map(|inner| inner.fetch_all_bids()).unwrap()
    }

    #[allow(dead_code)]
    pub fn fetch_all_pool_asks(&self, id: &PoolId) -> Vec<&ValidOrder<O>> {
        self.0.get(id).map(|inner| inner.fetch_all_asks()).unwrap()
    }

    #[allow(dead_code)]
    /// Fetches supply and demand intersection
    pub fn fetch_pool_intersection(&self, id: &PoolId) -> BidAndAsks<ValidOrder<O>> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_intersection())
            .unwrap()
    }

    #[allow(dead_code)]
    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_pool_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<ValidOrder<O>> {
        todo!("Blocked until added tick impl")
    }

    #[allow(dead_code)]
    pub fn fetch_all_orders(&self) -> Vec<Vec<&ValidOrder<O>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_orders())
            .collect()
    }

    #[allow(dead_code)]
    pub fn fetch_all_bids(&self) -> Vec<Vec<&ValidOrder<O>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_bids())
            .collect()
    }

    #[allow(dead_code)]
    pub fn fetch_all_asks(&self) -> Vec<Vec<&ValidOrder<O>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_asks())
            .collect()
    }

    #[allow(dead_code)]
    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> Vec<BidAndAsks<ValidOrder<O>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_intersection())
            .collect()
    }

    #[allow(dead_code)]
    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> Vec<BidAndAsks<ValidOrder<O>>> {
        todo!("Blocked until added tick impl")
    }
}
