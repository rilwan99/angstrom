use std::{collections::HashMap, fmt::Debug};

use reth_primitives::{alloy_primitives::Address, B256, U256};

use self::{composable::ComposableLimitPool, limit::LimitPool};
use crate::{
    common::{OrderId, SizeTracker},
    PooledComposableOrder, PooledLimitOrder, PooledOrder
};

mod composable;
mod limit;
mod parked;
mod pending;
pub use pending::OrderPriorityData;

type PoolId = Address;

pub type RegularAndLimit<T, C> = (Vec<T>, Vec<C>);
pub type RegularAndLimitRef<'a, T, C> = (Vec<&'a T>, Vec<&'a C>);

pub struct LimitOrderPool<T: PooledLimitOrder, C: PooledComposableOrder + PooledLimitOrder> {
    composable_orders:   ComposableLimitPool<C>,
    limit_orders:        LimitPool<T>,
    /// used for easy update operations on Orders.
    all_order_ids:       HashMap<OrderId, LimitOrderLocation>,
    /// used for nonce lookup.
    user_to_id:          HashMap<Address, Vec<OrderId>>,
    /// hash to pool location with identifier.
    order_hash_location: HashMap<B256, (OrderId, LimitOrderLocation)>,
    /// The size of the current transactions.
    size:                SizeTracker
}

impl<T: PooledLimitOrder, C: PooledComposableOrder + PooledLimitOrder> LimitOrderPool<T, C> {
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            composable_orders:   ComposableLimitPool::new(),
            limit_orders:        LimitPool::new(),
            all_order_ids:       HashMap::new(),
            user_to_id:          HashMap::new(),
            order_hash_location: HashMap::new(),
            size:                SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn new_composable_order(&mut self, order: C) -> Result<(), LimitPoolError> {
        let id = order.order_id();

        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize)
        }

        self.check_for_duplicates(&id)?;
        self.composable_orders.new_order(order)?;
        self.add_order_tracking(id, LimitOrderLocation::Composable);

        Ok(())
    }

    pub fn new_limit_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        let id = order.order_id();

        let size = order.size();
        if !self.size.has_space(size) {
            return Err(LimitPoolError::MaxSize)
        }

        self.check_for_duplicates(&id)?;
        let location = self.limit_orders.new_order(order)?;
        self.add_order_tracking(id, location);

        Ok(())
    }

    /// Removes all filled orders from the pools
    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> RegularAndLimit<T, C> {
        // remove from lower level + hash locations;
        let (left, right): (Vec<_>, Vec<_>) = orders
            .iter()
            .filter_map(|order_hash| {
                // remove from order hash loc
                let (id, location) = self.order_hash_location.remove(order_hash)?;
                // remove from all orders
                let _ = self.all_order_ids.remove(&id)?;
                // remove from user;
                self.user_to_id
                    .get_mut(&id.address)
                    .map(|inner| inner.retain(|order| order != &id));

                match location {
                    LimitOrderLocation::Composable => {
                        Some((None, self.composable_orders.remove_order(&id)))
                    }
                    LimitOrderLocation::LimitPending => {
                        Some((self.limit_orders.remove_order(&id, location), None))
                    }
                    _ => {
                        unreachable!()
                    }
                }
            })
            .unzip();

        (self.filter_option_and_adjust_size(left), self.filter_option_and_adjust_size(right))
    }

    /// Removes all orders for a given user when there state changes for
    /// re-validation
    pub fn changed_user_state(&mut self, users: &Vec<Address>) -> RegularAndLimit<T, C> {
        let (left, right): (Vec<_>, Vec<_>) = users
            .iter()
            // remove user
            .filter_map(|user| self.user_to_id.remove(user))
            .flatten()
            .filter_map(|user_order| {
                // remove all orders
                let loc = self.all_order_ids.remove(&user_order)?;
                // remove hash
                let _ = self.order_hash_location.remove(&user_order.hash);
                match loc {
                    LimitOrderLocation::Composable => {
                        Some((None, self.composable_orders.remove_order(&user_order)))
                    }
                    _ => Some((self.limit_orders.remove_order(&user_order, loc), None))
                }
            })
            .unzip();

        (self.filter_option_and_adjust_size(left), self.filter_option_and_adjust_size(right))
    }

    // individual fetches
    pub fn fetch_all_pool_orders(&mut self, id: &PoolId) -> RegularAndLimitRef<T, C> {
        (
            self.limit_orders.fetch_all_pool_orders(id),
            self.composable_orders.fetch_all_pool_orders(id)
        )
    }
}

// Helper functions
impl<T: PooledLimitOrder, C: PooledComposableOrder + PooledLimitOrder> LimitOrderPool<T, C> {
    /// Helper function for unzipping and size adjustment
    fn filter_option_and_adjust_size<O: PooledOrder>(&mut self, order: Vec<Option<O>>) -> Vec<O> {
        order
            .into_iter()
            .filter_map(|order| order)
            .map(|order| {
                self.size.remove_order(order.size());
                order
            })
            .collect()
    }

    /// Helper function for checking for duplicates when adding orders
    fn check_for_duplicates(&self, id: &OrderId) -> Result<(), LimitPoolError> {
        // is new order
        if self.all_order_ids.contains_key(&id) {
            return Err(LimitPoolError::DuplicateOrder)
        }

        // check for duplicate nonce
        if self
            .user_to_id
            .get(&id.address)
            .map(|inner| inner.iter().any(|other_id| other_id.nonce == id.nonce))
            .unwrap_or(false)
        {
            return Err(LimitPoolError::DuplicateNonce(id.clone()))
        }

        Ok(())
    }

    /// Helper function to add new orders to tracking
    fn add_order_tracking(&mut self, id: OrderId, location: LimitOrderLocation) {
        let user = id.address;

        // add to user tracking
        self.user_to_id.entry(user).or_default().push(id.clone());
        // add to hash tracking
        self.order_hash_location
            .insert(id.hash, (id.clone(), location));
        // add to all order id
        self.all_order_ids.insert(id, location);
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

#[derive(Debug, Clone, Copy)]
pub enum LimitOrderLocation {
    Composable,
    LimitParked,
    LimitPending
}
