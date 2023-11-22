use std::fmt::Debug;

use alloy_primitives::{TxHash, U256};
use derive_more::{AsRef, Deref};
use guard_types::orders::{
    OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
    ValidatedOrder
};

use crate::common::error::ValidationError;

pub mod order_validator;
pub mod sim;
pub mod state;

pub enum OrderValidationRequest {}

/// A valid order in the pool.
pub enum OrderValidationOutcome<O: PooledOrder> {
    /// The transaction is considered _currently_ valid and can be inserted into
    /// the pool.
    Valid {
        /// The validated order
        order:     ValidatedOrder<O, O::ValidationData>,
        /// Whether to propagate the order to the network.
        propagate: bool
    },
    /// The transaction is considered invalid indefinitely: It violates
    /// constraints that prevent this transaction from ever becoming valid.
    Invalid(O, ValidationError),
    /// An error occurred while trying to validate the transaction
    Error(TxHash, Box<dyn std::error::Error + Send + Sync>)
}

/// Provides support for validating transaction at any given state of the chain
#[async_trait::async_trait]
pub trait OrderValidator: Send + Sync + Clone + Unpin + 'static {
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

    async fn validate_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::LimitOrder
    ) -> OrderValidationOutcome<Self::LimitOrder>;

    /// Validates a batch of orders.
    ///
    /// Must return all outcomes for the given orders in the same order.

    async fn validate_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::LimitOrder)>
    ) -> Vec<OrderValidationOutcome<Self::LimitOrder>> {
        futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_order(origin, tx))
        )
        .await
    }

    async fn validate_composable_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableLimitOrder
    ) -> OrderValidationOutcome<Self::ComposableLimitOrder>;

    async fn validate_composable_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableLimitOrder)>
    ) -> Vec<OrderValidationOutcome<Self::ComposableLimitOrder>> {
        futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_composable_order(origin, tx))
        )
        .await
    }

    async fn validate_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::SearcherOrder
    ) -> OrderValidationOutcome<Self::SearcherOrder>;

    async fn validate_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::SearcherOrder)>
    ) -> Vec<OrderValidationOutcome<Self::SearcherOrder>> {
        futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_searcher_order(origin, tx))
        )
        .await
    }

    async fn validate_composable_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableSearcherOrder
    ) -> OrderValidationOutcome<Self::ComposableSearcherOrder>;

    async fn validate_composable_searcher_orders(
        &self,
        transactions: Vec<(OrderOrigin, Self::ComposableSearcherOrder)>
    ) -> Vec<OrderValidationOutcome<Self::ComposableSearcherOrder>> {
        futures_util::future::join_all(
            transactions
                .into_iter()
                .map(|(origin, tx)| self.validate_composable_searcher_order(origin, tx))
        )
        .await
    }
}
