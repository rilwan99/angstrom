mod common;
mod config;
mod inner;
mod limit;
mod searcher;
mod validator;
use std::sync::Arc;

use alloy_primitives::TxHash;
use config::PoolConfig;
use guard_types::orders::{
    OrderPriorityData, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
    SearcherPriorityData, ValidatedOrder
};
pub use guard_utils::*;
pub use inner::OrderPoolInner;

//TODO: Impl order pool api
#[auto_impl::auto_impl(Arc)]
pub trait OrderPool: Send + Sync + Clone {
    /// The transaction type of the limit order pool
    type LimitOrder: PooledLimitOrder;

    /// The transaction type of the searcher order pool
    type SearcherOrder: PooledSearcherOrder;

    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder: PooledComposableOrder + PooledLimitOrder;

    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder: PooledComposableOrder + PooledSearcherOrder;

    fn get_pooled_orders_by_hashes(
        &self,
        tx_hashes: Vec<TxHash>,
        limit: Option<usize>
    ) -> Vec<PooledOrder>;
}
