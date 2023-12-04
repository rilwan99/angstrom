use std::{fmt::Debug, future::Future, pin::Pin};

use guard_types::orders::{OrderOrigin, OrderValidationOutcome, PoolOrder};

pub mod order_validator;
pub mod sim;
pub mod state;
mod validator;
pub use validator::*;

pub type ValidationFuture<O> =
    Pin<Box<dyn Future<Output = OrderValidationOutcome<O>> + Send + Sync>>;

pub type ValidationsFuture<O> =
    Pin<Box<dyn Future<Output = Vec<OrderValidationOutcome<O>>> + Send + Sync>>;

pub enum OrderValidationRequest {}

/// Provides support for validating transaction at any given state of the chain
pub trait OrderValidator: Send + Sync + Clone + Debug + Unpin + 'static {
    /// The order type of the limit order pool
    type LimitOrder: PoolOrder;

    /// The transaction type of the searcher order pool
    type SearcherOrder: PoolOrder;

    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder: PoolOrder;

    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder: PoolOrder;

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
    ) -> ValidationFuture<Self::LimitOrder>;

    /// Validates a batch of orders.
    ///
    /// Must return all outcomes for the given orders in the same order.

    fn validate_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::LimitOrder)>
    ) -> ValidationsFuture<Self::LimitOrder> {
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
    ) -> ValidationFuture<Self::ComposableLimitOrder>;

    fn validate_composable_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableLimitOrder)>
    ) -> ValidationsFuture<Self::ComposableLimitOrder> {
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
    ) -> ValidationFuture<Self::SearcherOrder>;

    fn validate_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::SearcherOrder)>
    ) -> ValidationsFuture<Self::SearcherOrder> {
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
    ) -> ValidationFuture<Self::ComposableSearcherOrder>;

    fn validate_composable_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableSearcherOrder)>
    ) -> ValidationsFuture<Self::ComposableSearcherOrder> {
        Box::pin(futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_composable_searcher_order(origin, tx))
        ))
    }
}
