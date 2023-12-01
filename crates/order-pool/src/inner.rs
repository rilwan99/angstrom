use std::{
    collections::HashMap,
    marker::Unpin,
    pin::Pin,
    task::{Context, Poll}
};

use alloy_primitives::{Address, B256, U256};
use futures_util::{Future, Stream, StreamExt};
use guard_types::{
    orders::{
        OrderId, OrderLocation, OrderOrigin, OrderPriorityData, OrderValidationOutcome,
        PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
        SearcherPriorityData, ValidatedOrder, ValidationResults
    },
    primitive::PoolId
};
use tokio::sync::mpsc::Sender;
use validation::order::OrderValidator;

use crate::{
    common::FilledOrder, config::PoolConfig, limit::LimitOrderPool, searcher::SearcherPool,
    validator::Validator
};

pub struct OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    limit_pool:       LimitOrderPool<L, CL>,
    searcher_pool:    SearcherPool<S, CS>,
    _config:          PoolConfig,
    /// Address to order id, used for nonce lookups
    // address_to_orders: HashMap<Address, Vec<OrderId>>,
    /// Order hash to order id, used for order inclusion lookups
    hash_to_order_id: HashMap<B256, OrderId>,
    /// Order Validator
    validator:        Validator<L, CL, S, CS, V>
}

impl<L, CL, S, CS, V> OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder<ValidationData = ValidatedOrder<L, OrderPriorityData>>,
    CL: PooledComposableOrder
        + PooledLimitOrder<ValidationData = ValidatedOrder<CL, OrderPriorityData>>,

    S: PooledSearcherOrder<ValidationData = ValidatedOrder<S, SearcherPriorityData>>,
    CS: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = ValidatedOrder<CS, SearcherPriorityData>>,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    pub(crate) fn new(validator: V, config: PoolConfig) -> Self {
        Self {
            limit_pool:       LimitOrderPool::new(None),
            searcher_pool:    SearcherPool::new(None),
            _config:          config,
            // address_to_orders: HashMap::new(),
            hash_to_order_id: HashMap::new(),
            validator:        Validator::new(validator)
        }
    }

    pub fn new_limit_order(&mut self, origin: OrderOrigin, order: L) {
        self.validator.validate_order(origin, order);
    }

    pub fn new_composable_limit(&mut self, origin: OrderOrigin, order: CL) {
        self.validator.validate_composable_order(origin, order);
    }

    pub async fn new_searcher_order(&mut self, origin: OrderOrigin, order: S) {
        self.validator.validate_searcher_order(origin, order)
    }

    pub async fn new_composable_searcher_order(&mut self, origin: OrderOrigin, order: CS) {
        self.validator
            .validate_composable_searcher_order(origin, order)
    }

    /// Helper function to add new orders to tracking
    // fn add_order_tracking(&mut self, id: OrderId, location: OrderLocation) {
    //     let user = id.address;
    //
    //     // add to user tracking
    //     self.address_to_orders
    //         .entry(user)
    //         .or_default()
    //         .push(id.clone());
    //     // add to hash tracking
    //     self.hash_to_order_id.insert(id.hash, id);
    // }

    /// Helper function for checking for duplicates when adding orders
    // fn check_for_duplicates(&self, id: &OrderId) -> Result<(), PoolError> {
    //     // is new order
    //     if self.hash_to_order_id.contains_key(&id.hash) {
    //         return Err(PoolError::DuplicateOrder)
    //     }
    //
    //     // check for duplicate nonce
    //     if self
    //         .address_to_orders
    //         .get(&id.address)
    //         .map(|inner| inner.iter().any(|other_id| other_id.nonce == id.nonce))
    //         .unwrap_or(false)
    //     {
    //         return Err(PoolError::DuplicateNonce(id.clone()))
    //     }
    //
    //     Ok(())
    // }

    /*
    /// Removes all orders for a given user when there state changes for
    /// re-validation
    pub fn changed_user_state(
        &mut self,
        users: &Vec<Address>
    ) -> RegularAndLimit<ValidatedOrder<O, OrderPriorityData>, ValidatedOrder<C, OrderPriorityData>>
    {
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
                    OrderLocation::Composable => {
                        Some((None, self.composable_orders.remove_order(&user_order)))
                    }
                    _ => Some((self.limit_orders.remove_order(&user_order, loc), None))
                }
            })
            .unzip();

        (self.filter_option_and_adjust_size(left), self.filter_option_and_adjust_size(right))
    } */

    /// Removes all filled orders from the pools
    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> Vec<FilledOrder<L, CL, S, CS>> {
        // remove from lower level + hash locations;
        orders
            .iter()
            .filter_map(|order_hash| {
                let order_id = self.hash_to_order_id.remove(order_hash)?;
                let loc = order_id.location;
                match loc {
                    OrderLocation::Composable => self
                        .limit_pool
                        .remove_composable_limit_order(order_hash)
                        .map(FilledOrder::add_composable_limit),
                    OrderLocation::LimitParked | OrderLocation::LimitPending => self
                        .limit_pool
                        .remove_limit_order(order_hash, loc)
                        .map(FilledOrder::add_limit),
                    OrderLocation::VanillaSearcher => self
                        .searcher_pool
                        .remove_searcher_order(order_id)
                        .inspect_err(|e| eprint!("{e:?}"))
                        .ok()
                        .map(FilledOrder::add_searcher),
                    OrderLocation::ComposableSearcher => self
                        .searcher_pool
                        .remove_composable_searcher_order(order_id)
                        .inspect_err(|e| eprint!("{e:?}"))
                        .ok()
                        .map(FilledOrder::add_composable_searcher)
                }
            })
            .collect()
    }
}

impl<L, CL, S, CS, V> OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    fn handle_validated_order(&mut self, res: ValidationResults<L, CL, S, CS>) {}
}

// impl Future for OrderPoolInner<>
impl<L, CL, S, CS, V> Future for OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator + Unpin
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        while let Poll::Ready(Some(next)) = self.validator.poll_next_unpin(cx) {
            self.handle_validated_order(next)
        }
        Poll::Pending
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId),
    #[error("Already have a ordered with {0:?}")]
    DuplicateNonce(OrderId),
    #[error("Duplicate order")]
    DuplicateOrder
}
