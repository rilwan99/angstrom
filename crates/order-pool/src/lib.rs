mod common;
mod config;
mod inner;
mod limit;
mod searcher;
mod validator;
use std::{collections::HashMap, sync::Arc};

use alloy_primitives::TxHash;
use config::PoolConfig;
use guard_types::{
    orders::{
        OrderOrigin, OrderPriorityData, PoolOrder, PooledComposableOrder, PooledLimitOrder,
        PooledOrder, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    primitive::PoolId
};
pub use guard_utils::*;
pub use inner::OrderPoolInner;
use validation::order::OrderValidator;

pub struct OrderSet<Limit: PooledLimitOrder, Searcher: PooledSearcherOrder> {
    pub limit_vanilla:    HashMap<PoolId, BidsAndAsks<Limit>>,
    pub searcher_vanilla: HashMap<PoolId, ValidatedOrder<Searcher, Searcher::ValidationData>>
}

pub struct BidsAndAsks<O: PoolOrder> {
    pub bids: Vec<ValidatedOrder<O, O::ValidationData>>,
    pub asks: Vec<ValidatedOrder<O, O::ValidationData>>
}

pub struct AllOrders<
    Limit: PooledLimitOrder,
    Searcher: PooledSearcherOrder,
    LimitCompose: PooledLimitOrder,
    SearcherCompose: PooledSearcherOrder
> {
    pub vanilla:    OrderSet<Limit, Searcher>,
    pub composable: OrderSet<LimitCompose, SearcherCompose>
}

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

    // New order functionality.
    fn new_limit_order(&self, origin: OrderOrigin, order: Self::LimitOrder);
    fn new_searcher_order(&self, origin: OrderOrigin, order: Self::SearcherOrder);
    fn new_composable_limit_order(&self, origin: OrderOrigin, order: Self::ComposableLimitOrder);
    fn new_composable_searcher_order(
        &self,
        origin: OrderOrigin,
        order: Self::ComposableSearcherOrder
    );

    async fn get_pooled_orders_by_hashes(
        &self,
        tx_hashes: Vec<TxHash>,
        limit: Option<usize>
    ) -> Vec<PooledOrder>;

    // Queries for fetching all orders. Will be used for quoting
    // and consensus.

    // fetches all vanilla orders
    async fn get_all_vanilla_orders(&self) -> OrderSet<Self::LimitOrder, Self::SearcherOrder>;
    // fetches all vanilla orders where for each pool the bids and asks overlap plus
    // a buffer on each side
    async fn get_all_vanilla_orders_intersection(
        &self,
        buffer: usize
    ) -> OrderSet<Self::LimitOrder, Self::SearcherOrder>;

    async fn get_all_composable_orders(
        &self
    ) -> OrderSet<Self::ComposableLimitOrder, Self::ComposableSearcherOrder>;

    async fn get_all_composable_orders_intersection(
        &self,
        buffer: usize
    ) -> OrderSet<Self::ComposableLimitOrder, Self::ComposableSearcherOrder>;

    async fn get_all_orders(
        &self
    ) -> AllOrders<
        Self::LimitOrder,
        Self::SearcherOrder,
        Self::ComposableLimitOrder,
        Self::ComposableSearcherOrder
    >;

    async fn get_all_orders_intersection(
        &self,
        buffer: usize
    ) -> AllOrders<
        Self::LimitOrder,
        Self::SearcherOrder,
        Self::ComposableLimitOrder,
        Self::ComposableSearcherOrder
    >;
}
