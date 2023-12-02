use std::fmt;

use alloy_primitives::{Address, Bytes, TxHash, U128, U256};
use guard_types::{
    orders::{
        OrderId, OrderPriorityData, PoolOrder, PooledComposableOrder, PooledLimitOrder,
        PooledOrder, PooledSearcherOrder
    },
    primitive::{ComposableOrder, Order, PoolKey},
    rpc::{
        EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
        EcRecoveredSearcherOrder, SignedComposableLimitOrder
    }
};

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
