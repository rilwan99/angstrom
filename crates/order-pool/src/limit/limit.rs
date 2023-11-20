use std::collections::HashMap;

use guard_types::{
    orders::{OrderId, OrderPriorityData, PooledLimitOrder, PooledOrder, ValidatedOrder},
    primitive::PoolId
};

use super::{parked::ParkedPool, pending::PendingPool, LimitOrderLocation, LimitPoolError};
use crate::common::BidAndAsks;
pub struct LimitPool<T: PooledLimitOrder> {
    pending_orders: Vec<PendingPool<T>>,
    parked_orders:  Vec<ParkedPool<T>>
}

impl<O: PooledLimitOrder> LimitPool<O>
where
    O: PooledOrder<ValidationData = ValidatedOrder<O, OrderPriorityData>>
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, OrderPriorityData>
    ) -> Result<LimitOrderLocation, LimitPoolError> {
        let pool_id = order.pool_id();

        if order.is_valid() {
            self.pending_orders
                .get_mut(pool_id)
                .map(|pool| pool.add_order(order))
                .ok_or_else(|| LimitPoolError::NoPool(pool_id))?;
            Ok(LimitOrderLocation::LimitPending)
        } else {
            self.parked_orders
                .get_mut(pool_id)
                .map(|pool| pool.new_order(order))
                .ok_or_else(|| LimitPoolError::NoPool(pool_id))?;
            Ok(LimitOrderLocation::LimitParked)
        }
    }

    pub fn remove_order(
        &mut self,
        order_id: &OrderId,
        location: LimitOrderLocation
    ) -> Option<ValidatedOrder<O, OrderPriorityData>> {
        match location {
            LimitOrderLocation::LimitPending => self
                .pending_orders
                .get_mut(order_id.pool_id)
                .and_then(|pool: &mut _| pool.remove_order(order_id.hash)),
            LimitOrderLocation::LimitParked => self
                .parked_orders
                .get_mut(order_id.pool_id)
                .and_then(|pool: &mut _| pool.remove_order(order_id)),
            _ => unreachable!()
        }
    }

    pub fn fetch_all_pool_orders(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_orders())
            .unwrap()
    }

    pub fn fetch_all_pool_bids(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_bids())
            .unwrap()
    }

    pub fn fetch_all_pool_asks(&self, id: &PoolId) -> Vec<&ValidatedOrder<O, OrderPriorityData>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_asks())
            .unwrap()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_pool_intersection(
        &self,
        id: &PoolId
    ) -> BidAndAsks<ValidatedOrder<O, OrderPriorityData>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_intersection())
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
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_orders())
            .collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<Vec<&ValidatedOrder<O, OrderPriorityData>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_bids())
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<Vec<&ValidatedOrder<O, OrderPriorityData>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_asks())
            .collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> Vec<BidAndAsks<ValidatedOrder<O, OrderPriorityData>>> {
        self.pending_orders
            .iter()
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
