use std::collections::{BTreeMap, HashMap};

use alloy_primitives::{Address, B256};
use composable::ComposableSearcherPool;
use guard_types::{
    orders::{
        OrderId, PooledComposableOrder, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    primitive::PoolId
};

use self::searcher::VanillaSearcherPool;
use crate::common::SizeTracker;

mod composable;
mod searcher;

pub struct SearcherPool<O: PooledSearcherOrder, C: PooledComposableOrder + PooledSearcherOrder> {
    /// used for nonce lookup.
    user_to_id: HashMap<Address, Vec<OrderId>>,
    /// Holds all non composable searcher order pools
    searcher_orders: VanillaSearcherPool<O>,
    /// Holds all composable searcher order pools
    composable_searcher_orders: ComposableSearcherPool<C>,
    /// The size of the current transactions.
    size: SizeTracker
}

impl<O: PooledSearcherOrder, C: PooledSearcherOrder + PooledComposableOrder> SearcherPool<O, C>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>,
    C: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = ValidatedOrder<C, SearcherPriorityData>>
{
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            user_to_id: HashMap::new(),
            searcher_orders: VanillaSearcherPool::new(Some(15)),
            composable_searcher_orders: ComposableSearcherPool::new(),
            size: SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn remove_searcher_order(&mut self, order_hash: &B256) -> Option<O> {
        todo!()
    }

    pub fn remove_composable_searcher_order(&mut self, order_hash: &B256) -> Option<C> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SearcherPoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId),
    #[error("Already have a ordered with {0:?}")]
    DuplicateNonce(OrderId),
    #[error("Duplicate order")]
    DuplicateOrder
}
