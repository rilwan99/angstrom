use std::collections::HashMap;

use guard_types::{
    orders::{OrderId, OrderPriorityData, PooledComposableOrder, PooledLimitOrder, ValidatedOrder},
    primitive::PoolId
};

use super::{pending::PendingPool, LimitPoolError};
use crate::common::BidAndAsks;

pub struct ComposableLimitPool<C: PooledComposableOrder + PooledLimitOrder>(
    HashMap<PoolId, PendingPool<C>>
);

impl<O: PooledComposableOrder + PooledLimitOrder> ComposableLimitPool<O>
where
    O: PooledComposableOrder<ValidationData = ValidatedOrder<O, OrderPriorityData>>
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, OrderPriorityData>
    ) -> Result<(), LimitPoolError> {
        let id: OrderId = order.into();
        self.0
            .get_mut(&id.pool_id)
            .map(|pool| pool.add_order(order))
            .ok_or_else(|| LimitPoolError::NoPool(id.pool_id))
    }

    pub fn remove_order(
        &mut self,
        tx_id: &OrderId
    ) -> Option<ValidatedOrder<O, OrderPriorityData>> {
        self.0.get_mut(&tx_id.pool_id)?.remove_order(tx_id.hash)
    }

    pub fn fetch_all_pool_orders(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_all_orders())
            .unwrap()
    }

    pub fn fetch_all_pool_bids(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.0.get(id).map(|inner| inner.fetch_all_bids()).unwrap()
    }

    pub fn fetch_all_pool_asks(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.0.get(id).map(|inner| inner.fetch_all_asks()).unwrap()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_pool_intersection(
        &self,
        id: &PoolId
    ) -> BidAndAsks<ValidatedOrder<O, OrderPriorityData>> {
        self.0
            .get(id)
            .map(|inner| inner.fetch_intersection())
            .unwrap()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_pool_intersection_with_buffer(
        &self,
        _buffer: u8
    ) -> BidAndAsks<ValidatedOrder<O, OrderPriorityData>> {
        todo!("Blocked until added tick impl")
    }

    pub fn fetch_all_orders(&self) -> Vec<Vec<&ValidatedOrder<O, OrderPriorityData>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_orders())
            .collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<Vec<&ValidatedOrder<O, OrderPriorityData>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_bids())
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<Vec<&ValidatedOrder<O, OrderPriorityData>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_all_asks())
            .collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> Vec<BidAndAsks<ValidatedOrder<O, OrderPriorityData>>> {
        self.0
            .values()
            .map(|inner| inner.fetch_intersection())
            .collect()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(
        &self,
        _buffer: u8
    ) -> Vec<BidAndAsks<ValidatedOrder<O, OrderPriorityData>>> {
        todo!("Blocked until added tick impl")
    }
}
