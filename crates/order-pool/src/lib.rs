mod common;
mod config;
mod inner;
mod limit;
mod searcher;
mod validator;
use std::{collections::HashMap, sync::Arc};

use alloy_primitives::TxHash;
use common::ValidOrder;
pub use config::PoolConfig;
use futures_util::future::BoxFuture;
use guard_types::{
    orders::{
        OrderConversion, OrderOrigin, OrderPriorityData, PoolOrder, PooledComposableOrder,
        PooledLimitOrder, PooledOrder, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    primitive::PoolId
};
pub use guard_utils::*;
pub use inner::*;

#[derive(Debug)]
pub struct OrderSet<Limit: PoolOrder, Searcher: PoolOrder> {
    pub limit_vanilla:    Vec<BidsAndAsks<Limit>>,
    pub searcher_vanilla: Vec<ValidOrder<Searcher>>
}

#[derive(Debug)]
pub struct BidsAndAsks<O: PoolOrder> {
    pub bids: Vec<ValidOrder<O>>,
    pub asks: Vec<ValidOrder<O>>
}

#[derive(Debug)]
pub struct AllOrders<
    Limit: PoolOrder,
    Searcher: PoolOrder,
    LimitCompose: PoolOrder,
    SearcherCompose: PoolOrder
> {
    pub vanilla:    OrderSet<Limit, Searcher>,
    pub composable: OrderSet<LimitCompose, SearcherCompose>
}

/// The OrderPool Trait is how other processes can interact with the orderpool
/// asyncly. This allows for requesting data and providing data from different
/// threads efficiently.
// #[auto_impl::auto_impl(Arc)]
pub trait OrderPoolHandle: Send + Sync + Clone + Unpin + 'static {
    /// The transaction type of the limit order pool
    type LimitOrder: PoolOrder;

    /// The transaction type of the searcher order pool
    type SearcherOrder: PoolOrder;

    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder: PoolOrder;

    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder: PoolOrder;

    // New order functionality.
    fn new_limit_order(
        &self,
        origin: OrderOrigin,
        order: <Self::LimitOrder as OrderConversion>::Order
    );
    fn new_searcher_order(
        &self,
        origin: OrderOrigin,
        order: <Self::SearcherOrder as OrderConversion>::Order
    );
    fn new_composable_limit_order(
        &self,
        origin: OrderOrigin,
        order: <Self::ComposableLimitOrder as OrderConversion>::Order
    );
    fn new_composable_searcher_order(
        &self,
        origin: OrderOrigin,
        order: <Self::ComposableSearcherOrder as OrderConversion>::Order
    );

    // Queries for fetching all orders. Will be used for quoting
    // and consensus.

    // fetches all vanilla orders
    fn get_all_vanilla_orders(&self) -> BoxFuture<OrderSet<Self::LimitOrder, Self::SearcherOrder>>;
    // fetches all vanilla orders where for each pool the bids and asks overlap plus
    // a buffer on each side
    fn get_all_vanilla_orders_intersection(
        &self,
        buffer: usize
    ) -> BoxFuture<OrderSet<Self::LimitOrder, Self::SearcherOrder>>;

    fn get_all_composable_orders(
        &self
    ) -> BoxFuture<OrderSet<Self::ComposableLimitOrder, Self::ComposableSearcherOrder>>;

    fn get_all_composable_orders_intersection(
        &self,
        buffer: usize
    ) -> BoxFuture<OrderSet<Self::ComposableLimitOrder, Self::ComposableSearcherOrder>>;

    fn get_all_orders(
        &self
    ) -> BoxFuture<
        AllOrders<
            Self::LimitOrder,
            Self::SearcherOrder,
            Self::ComposableLimitOrder,
            Self::ComposableSearcherOrder
        >
    >;

    fn get_all_orders_intersection(
        &self,
        buffer: usize
    ) -> BoxFuture<
        AllOrders<
            Self::LimitOrder,
            Self::SearcherOrder,
            Self::ComposableLimitOrder,
            Self::ComposableSearcherOrder
        >
    >;
}
