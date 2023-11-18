use std::collections::HashMap;

use reth_primitives::B256;

use super::{LimitOrderLocation, LimitPoolError, PoolId};
use crate::{
    common::{BidAndAsks, OrderId, ParkedPool, PendingPool},
    PooledLimitOrder
};

pub struct LimitPool<T: PooledLimitOrder> {
    pending_orders: HashMap<PoolId, PendingPool<T>>,
    parked_orders:  HashMap<PoolId, ParkedPool<T>>
}

impl<T: PooledLimitOrder> LimitPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<LimitOrderLocation, LimitPoolError> {
        let pool_addr = order.order_id().pool_id;

        if order.is_valid() {
            self.pending_orders
                .get_mut(&pool_addr)
                .map(|pool| pool.new_order(order))
                .ok_or_else(|| LimitPoolError::NoPool(pool_addr))?;
            Ok(LimitOrderLocation::LimitPending)
        } else {
            self.parked_orders
                .get_mut(&pool_addr)
                .map(|pool| pool.new_order(order))
                .ok_or_else(|| LimitPoolError::NoPool(pool_addr))?;
            Ok(LimitOrderLocation::LimitParked)
        }
    }

    pub fn remove_order(&mut self, order_id: &OrderId, location: LimitOrderLocation) -> Option<T> {
        todo!()
        // match location {
        //     LimitOrderLocation::LimitParked => {
        //         self.parked_orders.get_mut(order_id.pool_id).map(|inner|
        //     },
        //     LimitOrderLocation::LimitPending => {
        //     },
        //     _ => unreachable!()
        // }
        // self.o
    }

    pub fn fetch_all_orders(&self, id: &PoolId) -> Vec<&T> {
        self.pending_orders
            .get(id)
            .map(|inner| inner.fetch_all_orders())
            .unwrap()
    }

    pub fn fetch_all_bids(&self, id: &PoolId) -> Vec<&T> {
        self.pending_orders
            .get(id)
            .map(|inner| inner.fetch_all_bids())
            .unwrap()
    }

    pub fn fetch_all_asks(&self, id: &PoolId) -> Vec<&T> {
        self.pending_orders
            .get(id)
            .map(|inner| inner.fetch_all_asks())
            .unwrap()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self, id: &PoolId) -> BidAndAsks<T> {
        self.pending_orders
            .get(id)
            .map(|inner| inner.fetch_intersection())
            .unwrap()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<T> {
        todo!("Blocked until added tick impl")
    }
}
