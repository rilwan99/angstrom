use std::collections::{BTreeMap, HashMap};

use alloy_primitives::{Address, B256};
use composable::ComposableSearcherPool;
use guard_types::{
    orders::{OrderId, PooledComposableOrder, PooledSearcherOrder},
    primitive::PoolId
};

use self::searcher::VanillaSearcherPool;
use crate::common::SizeTracker;

mod composable;
mod searcher;

pub struct SearcherPool<T: PooledSearcherOrder, C: PooledComposableOrder + PooledSearcherOrder> {
    /// Holds all non composable searcher order pools
    searcher_orders: VanillaSearcherPool<T>,
    /// Holds all composable searcher order pools
    composable_searcher_orders: ComposableSearcherPool<C>,
    /// The size of the current transactions.
    size: SizeTracker
}

pub enum SearcherOrderLocation {
    Vanilla,
    Composable
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
