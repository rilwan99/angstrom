use std::{fmt::Debug, future::Future, pin::Pin};

use alloy_primitives::{TxHash, U256};
use derive_more::{AsRef, Deref};
use futures_util::StreamExt;
use guard_types::orders::{
    OrderOrigin, OrderValidationOutcome, PooledComposableOrder, PooledLimitOrder, PooledOrder,
    PooledSearcherOrder, ValidatedOrder, ValidationError
};

pub mod order_validator;
pub mod sim;
pub mod state;

pub enum OrderValidationRequest {}

/// Provides support for validating transaction at any given state of the chain
pub trait OrderValidator: Send + Sync + Clone + Debug + Unpin + 'static {
    /// The order type of the limit order pool
    type LimitOrder: PooledOrder;

    /// The transaction type of the searcher order pool
    type SearcherOrder: PooledOrder;

    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder: PooledOrder;

    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder: PooledOrder;

    /// Validates the order and returns a [`OrderValidationOutcome`]
    /// describing the validity of the given order
    ///
    /// This will be used by the order-pool to check whether the
    /// transaction should be inserted into the pool or discarded right
    /// away.

    fn validate_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::LimitOrder
    ) -> Pin<Box<dyn Future<Output = OrderValidationOutcome<Self::LimitOrder>> + Send + Sync>>;

    /// Validates a batch of orders.
    ///
    /// Must return all outcomes for the given orders in the same order.

    fn validate_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::LimitOrder)>
    ) -> Pin<Box<dyn Future<Output = Vec<OrderValidationOutcome<Self::LimitOrder>>> + Send + Sync>>
    {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_order(origin, tx))
        ))
    }

    fn validate_composable_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableLimitOrder
    ) -> Pin<
        Box<dyn Future<Output = OrderValidationOutcome<Self::ComposableLimitOrder>> + Send + Sync>
    >;

    fn validate_composable_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableLimitOrder)>
    ) -> Pin<
        Box<
            dyn Future<Output = Vec<OrderValidationOutcome<Self::ComposableLimitOrder>>>
                + Send
                + Sync
        >
    > {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_composable_order(origin, tx))
        ))
    }

    fn validate_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::SearcherOrder
    ) -> Pin<Box<dyn Future<Output = OrderValidationOutcome<Self::SearcherOrder>> + Send + Sync>>;

    fn validate_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::SearcherOrder)>
    ) -> Pin<Box<dyn Future<Output = Vec<OrderValidationOutcome<Self::SearcherOrder>>> + Send + Sync>> {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_searcher_order(origin, tx))
        ))
    }

    fn validate_composable_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableSearcherOrder
    ) -> Pin<
        Box<
            dyn Future<Output = OrderValidationOutcome<Self::ComposableSearcherOrder>>
                + Send
                + Sync
        >
    >;

    fn validate_composable_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableSearcherOrder)>
    ) -> Pin<
        Box<
            dyn Future<Output = Vec<OrderValidationOutcome<Self::ComposableSearcherOrder>>>
                + Send
                + Sync
        >
    > {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_composable_searcher_order(origin, tx))
        ))
    }
}
