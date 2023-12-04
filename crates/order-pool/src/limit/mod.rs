use std::{collections::HashMap, fmt::Debug};

use guard_types::{
    orders::{
        OrderId, OrderLocation, OrderPriorityData, PoolOrder, PooledComposableOrder,
        PooledLimitOrder, ValidatedOrder
    },
    primitive::PoolId
};
use reth_primitives::B256;

use self::{composable::ComposableLimitPool, limit::LimitPool};
use crate::common::SizeTracker;

mod composable;
mod limit;
mod parked;
mod pending;
#[allow(dead_code)]
pub type RegularAndLimitRef<'a, T, C> = (Vec<&'a T>, Vec<&'a C>);

#[allow(dead_code)]
pub struct LimitOrderPool<O, C>
where
    O: PooledLimitOrder,
    C: PooledComposableOrder + PooledLimitOrder
{
    /// Sub-pool of all limit orders
    limit_orders:      LimitPool<O>,
    /// Sub-pool of all composable orders
    composable_orders: ComposableLimitPool<C>,
    /// The size of the current transactions.
    size:              SizeTracker
}

impl<O: PooledLimitOrder, C: PooledComposableOrder + PooledLimitOrder> LimitOrderPool<O, C>
where
    O: PooledLimitOrder<ValidationData = ValidatedOrder<O, OrderPriorityData>>,
    C: PooledComposableOrder
        + PooledLimitOrder<ValidationData = ValidatedOrder<C, OrderPriorityData>>
{
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            composable_orders: ComposableLimitPool::new(),
            limit_orders:      LimitPool::new(),
            size:              SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn add_composable_order(
        &mut self,
        order: ValidatedOrder<C, OrderPriorityData>
    ) -> Result<(), LimitPoolError> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize)
        }

        self.composable_orders.add_order(order)?;

        Ok(())
    }

    pub fn add_limit_order(
        &mut self,
        order: ValidatedOrder<O, OrderPriorityData>
    ) -> Result<(), LimitPoolError> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize)
        }

        let _location = self.limit_orders.add_order(order)?;

        Ok(())

        // TODO: What do we want to return, how do we want to wire it up
        // so it bubbles up to the highest level
    }

    // individual fetches
    pub fn fetch_all_pool_orders(
        &mut self,
        id: &PoolId
    ) -> RegularAndLimitRef<
        ValidatedOrder<O, OrderPriorityData>,
        ValidatedOrder<C, OrderPriorityData>
    > {
        (
            self.limit_orders.fetch_all_pool_orders(id),
            self.composable_orders.fetch_all_pool_orders(id)
        )
    }

    #[allow(dead_code)]
    pub fn remove_limit_order(&mut self, order_hash: &B256, location: OrderLocation) -> Option<O> {
        todo!()
    }

    pub fn remove_composable_limit_order(&mut self, order_hash: &B256) -> Option<C> {
        todo!()
    }
}

// Helper functions
impl<T, C> LimitOrderPool<T, C>
where
    T: PooledLimitOrder,
    C: PooledComposableOrder + PooledLimitOrder
{
    /// Helper function for unzipping and size adjustment
    fn filter_option_and_adjust_size<O: PoolOrder>(
        &mut self,
        order: Vec<Option<ValidatedOrder<O, OrderPriorityData>>>
    ) -> Vec<ValidatedOrder<O, OrderPriorityData>> {
        order
            .into_iter()
            .filter_map(|order| order)
            .map(|order| {
                self.size.remove_order(order.size());
                order
            })
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LimitPoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId),
    #[error("Already have a ordered with {0:?}")]
    DuplicateNonce(OrderId),
    #[error("Duplicate order")]
    DuplicateOrder
}
