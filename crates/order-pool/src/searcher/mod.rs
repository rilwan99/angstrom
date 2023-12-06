use alloy_primitives::B256;
use composable::ComposableSearcherPool;
use guard_types::{
    orders::{
        OrderId, PooledComposableOrder, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    primitive::PoolId
};

use self::searcher::VanillaSearcherPool;
use crate::common::{SizeTracker, ValidOrder};

mod composable;
mod searcher;

pub const SEARCHER_POOL_MAX_SIZE: usize = 15;
#[allow(non_upper_case_globals)]
pub const V1_LP_POOlS: usize = 5;

pub struct SearcherPool<S: PooledSearcherOrder, CS: PooledComposableOrder + PooledSearcherOrder> {
    /// Holds all non composable searcher order pools
    searcher_orders: VanillaSearcherPool<S>,
    /// Holds all composable searcher order pools
    composable_searcher_orders: ComposableSearcherPool<CS>,
    /// The size of the current transactions.
    _size: SizeTracker
}

impl<S, CS> SearcherPool<S, CS>
where
    S: PooledSearcherOrder<ValidationData = SearcherPriorityData>,
    CS: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = SearcherPriorityData>
        + PooledComposableOrder
{
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            searcher_orders: VanillaSearcherPool::new(None),
            composable_searcher_orders: ComposableSearcherPool::new(None),
            _size: SizeTracker { max: max_size, current: 0 }
        }
    }

    #[allow(dead_code)]
    pub fn add_searcher_order(&mut self, order: ValidOrder<S>) -> Result<(), SearcherPoolError> {
        let size = order.size();
        if !self._size.has_space(size) {
            return Err(SearcherPoolError::MaxSize)
        }

        self.searcher_orders.add_order(order)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn add_composable_searcher_order(
        &mut self,
        order: ValidOrder<CS>
    ) -> Result<(), SearcherPoolError> {
        let size = order.size();
        if !self._size.has_space(size) {
            return Err(SearcherPoolError::MaxSize)
        }

        self.composable_searcher_orders.add_order(order)?;
        Ok(())
    }

    pub fn remove_searcher_order(
        &mut self,
        id: OrderId
    ) -> Result<ValidOrder<S>, SearcherPoolError> {
        self.searcher_orders.remove_order(id)
    }

    pub fn remove_composable_searcher_order(
        &mut self,
        id: OrderId
    ) -> Result<ValidOrder<CS>, SearcherPoolError> {
        self.composable_searcher_orders.remove_order(id)
    }

    #[allow(dead_code)]
    pub fn get_winning_orders(&self) -> Vec<(Option<ValidOrder<S>>, Option<ValidOrder<CS>>)> {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
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
