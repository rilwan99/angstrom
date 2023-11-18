use std::{collections::HashMap, fmt::Debug};

use reth_primitives::{alloy_primitives::Address, B256, U256};

use self::{composable::ComposableLimitPool, limit::LimitPool};
use crate::{common::OrderId, PooledComposableOrder, PooledLimitOrder, PooledOrder};

mod composable;
mod limit;

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

type PoolId = Address;

struct SizeTracker {
    pub max:     Option<usize>,
    pub current: usize
}

impl SizeTracker {
    pub fn has_space(&mut self, size: usize) -> bool {
        if let Some(max) = self.max {
            if self.current + size <= max {
                self.current += size;
                true
            } else {
                false
            }
        } else {
            self.current += size;
            true
        }
    }
}

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

    fn check_for_duplicates(&self, id: &OrderId) -> Result<(), LimitPoolError> {
        // is new order
        if self.all_order_ids.contains_key(&id) {
            return Err(LimitPoolError::DuplicateOrder)
        }

        // check for duplicate nonce
        if self
            .user_to_id
            .get(&id.user_addr)
            .map(|inner| inner.iter().any(|other_id| other_id.nonce == id.nonce))
            .unwrap_or(false)
        {
            return Err(LimitPoolError::DuplicateNonce(id.clone()))
        }

        Ok(())
    }

    fn add_order_tracking(&mut self, id: OrderId, location: LimitOrderLocation) {
        let user = id.user_addr;

        // add to user tracking
        self.user_to_id.entry(user).or_default().push(id.clone());
        // add to hash tracking
        self.order_hash_location
            .insert(id.order_hash, (id.clone(), location));
        // add to all order id
        self.all_order_ids.insert(id, location);
    }

    /// Removes all filled orders from the pools
    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> (Vec<T>, Vec<C>) {
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
                    .get_mut(&id.user_addr)
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

        (
            left.into_iter().filter_map(|order| order).collect(),
            right.into_iter().filter_map(|order| order).collect()
        )
    }

    /// Removes all orders for a given user when there state changes for
    /// re-validation
    pub fn changed_user_state(&mut self, users: &Vec<Address>) -> (Vec<T>, Vec<C>) {
        let (left, right): (Vec<_>, Vec<_>) = users
            .iter()
            // remove user
            .filter_map(|user| self.user_to_id.remove(user))
            .flatten()
            .filter_map(|user_order| {
                // remove all orders
                let loc = self.all_order_ids.remove(&user_order)?;
                // remove hash
                let _ = self.order_hash_location.remove(&user_order.order_hash);
                match loc {
                    LimitOrderLocation::Composable => {
                        Some((None, self.composable_orders.remove_order(&user_order)))
                    }
                    _ => Some((self.limit_orders.remove_order(&user_order, loc), None))
                }
            })
            .unzip();

        (
            left.into_iter().filter_map(|order| order).collect(),
            right.into_iter().filter_map(|order| order).collect()
        )
    }

    pub fn get_all_order(&mut self) -> (Vec<T>, Vec<C>) {
        todo!()
    }

    pub fn get_overlap_with_buffer(&mut self, tick_buffer: u8) -> (Vec<T>, Vec<C>) {
        todo!()
    }

    // TODO: add ability to fetch composable and non-composable orders
}
