use std::collections::{BTreeMap, HashMap};

use alloy_primitives::{Address, B256};
use composable::ComposableSearcherPool;
use guard_types::{
    orders::{
        OrderId, PooledComposableOrder, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    primitive::{Order, PoolId}
};

use self::searcher::VanillaSearcherPool;
use crate::common::SizeTracker;

mod composable;
mod searcher;

pub const SEARCHER_POOL_MAX_SIZE: usize = 15;
pub const V1_LP_POOlS: usize = 5;

pub struct SearcherPool<S: PooledSearcherOrder, CS: PooledComposableOrder + PooledSearcherOrder> {
    /// Holds all non composable searcher order pools
    searcher_orders: VanillaSearcherPool<S>,
    /// Holds all composable searcher order pools
    composable_searcher_orders: ComposableSearcherPool<CS>,
    /// The size of the current transactions.
    size: SizeTracker
}

impl<S: PooledSearcherOrder, CS: PooledSearcherOrder + PooledComposableOrder> SearcherPool<S, CS>
where
    S: PooledSearcherOrder<ValidationData = ValidatedOrder<S, SearcherPriorityData>>,
    CS: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = ValidatedOrder<CS, SearcherPriorityData>>
{
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            searcher_orders: VanillaSearcherPool::new(None),
            composable_searcher_orders: ComposableSearcherPool::new(None),
            size: SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn add_searcher_order(
        &mut self,
        order: ValidatedOrder<S, SearcherPriorityData>
    ) -> Result<(), SearcherPoolError> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(SearcherPoolError::MaxSize)
        }

        self.searcher_orders.add_order(order)?;
        Ok(())
    }

    pub fn add_composable_searcher_order(
        &mut self,
        order: ValidatedOrder<CS, SearcherPriorityData>
    ) -> Result<(), SearcherPoolError> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(SearcherPoolError::MaxSize)
        }

        self.composable_searcher_orders.add_order(order)?;
        Ok(())
    }

    pub fn remove_searcher_order(&mut self, id: OrderId) -> Result<S, SearcherPoolError> {
        self.searcher_orders.remove_order(id)
    }

    pub fn remove_composable_searcher_order(
        &mut self,
        id: OrderId
    ) -> Result<CS, SearcherPoolError> {
        self.composable_searcher_orders.remove_order(id)
    }

    pub fn get_winning_orders(&self) -> Vec<(Option<S>, Option<CS>)> {
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
    DuplicateOrder,
    #[error("Order Not Found")]
    OrderNotFound(B256)
}