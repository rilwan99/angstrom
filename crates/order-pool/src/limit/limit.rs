use std::collections::HashMap;

use guard_types::{
    orders::{LimitOrderValidation, OrderId, PooledLimitOrder, PooledOrder},
    primitive::PoolId
};

use super::{parked::ParkedPool, pending::PendingPool, LimitOrderLocation, LimitPoolError};
use crate::common::{BidAndAsks, ValidOrder};

pub struct LimitPool<T: PooledLimitOrder>
where
    <T as PooledOrder>::ValidationData: LimitOrderValidation
{
    pending_orders: Vec<PendingPool<T>>,
    parked_orders:  Vec<ParkedPool<T>>
}

impl<T: PooledLimitOrder> LimitPool<T>
where
    <T as PooledOrder>::ValidationData: LimitOrderValidation
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_order(
        &mut self,
        order: ValidOrder<T>
    ) -> Result<LimitOrderLocation, LimitPoolError> {
        let pool_id = order.data.pool_id();

        if order.is_valid() {
            self.pending_orders
                .get_mut(pool_id)
                .map(|pool| pool.new_order(order))
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
    ) -> Option<ValidOrder<T>> {
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

    pub fn fetch_all_pool_orders(&self, id: &PoolId) -> Vec<&ValidOrder<T>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_orders())
            .unwrap()
    }

    pub fn fetch_all_pool_bids(&self, id: &PoolId) -> Vec<&ValidOrder<T>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_bids())
            .unwrap()
    }

    pub fn fetch_all_pool_asks(&self, id: &PoolId) -> Vec<&ValidOrder<T>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_all_asks())
            .unwrap()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_pool_intersection(&self, id: &PoolId) -> BidAndAsks<ValidOrder<T>> {
        self.pending_orders
            .get(*id)
            .map(|inner: &_| inner.fetch_intersection())
            .unwrap()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_pool_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<ValidOrder<T>> {
        todo!("Blocked until added tick impl")
    }

    pub fn fetch_all_orders(&self) -> Vec<Vec<&ValidOrder<T>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_orders())
            .collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<Vec<&ValidOrder<T>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_bids())
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<Vec<&ValidOrder<T>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_all_asks())
            .collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> Vec<BidAndAsks<ValidOrder<T>>> {
        self.pending_orders
            .iter()
            .map(|inner| inner.fetch_intersection())
            .collect()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> Vec<BidAndAsks<ValidOrder<T>>> {
        todo!("Blocked until added tick impl")
    }
}
