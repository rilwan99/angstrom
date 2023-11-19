use std::time::Instant;

use alloy_primitives::{TxHash, U256};
use order_pool::{
    OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder
};

use crate::common::error::ValidationError;

/// A valid order in the pool.
///
/// This is used as the internal representation of a order inside the
/// pool.
pub enum OrderValidationOutcome<O: PooledOrder> {
    /// The transaction is considered _currently_ valid and can be inserted into
    /// the pool.
    Valid {
        /// Balance of the sender at the current point.
        balance:     U256,
        /// Current nonce of the sender.
        state_nonce: u64,
        /// The validated transaction.
        ///
        /// See also [ValidTransaction].
        ///
        /// If this is a _new_ EIP-4844 blob transaction, then this must contain
        /// the extracted sidecar.
        order:       ValidatedOrder<O, O::ValidationData>,
        /// Whether to propagate the transaction to the network.
        propagate:   bool
    },
    /// The transaction is considered invalid indefinitely: It violates
    /// constraints that prevent this transaction from ever becoming valid.
    Invalid(O, ValidationError),
    /// An error occurred while trying to validate the transaction
    Error(TxHash, Box<dyn std::error::Error + Send + Sync>)
}

pub struct ValidatedOrder<O: PooledOrder, Data> {
    pub order: O,
    pub data:  Data
}

/// Provides support for validating transaction at any given state of the chain
#[async_trait::async_trait]
pub trait OrderValidator: Send + Sync {
    /// The order type of the limit order pool
    type LimitOrder: PooledLimitOrder;

    /// The transaction type of the searcher order pool
    type SearcherOrder: PooledSearcherOrder;

    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder: PooledComposableOrder + PooledLimitOrder;

    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder: PooledComposableOrder + PooledSearcherOrder;

    /// Validates the transaction and returns a [`OrderValidationOutcome`]
    /// describing the validity of the given transaction.
    ///
    /// This will be used by the transaction-pool to check whether the
    /// transaction should be inserted into the pool or discarded right
    /// away.
    ///
    /// Implementers of this trait must ensure that the transaction is
    /// well-formed, i.e. that it complies at least all static constraints,
    /// which includes checking for:
    ///
    ///    * chain id
    ///    * gas limit
    ///    * max cost
    ///    * nonce >= next nonce of the sender
    ///    * ...
    ///
    /// See [InvalidTransactionError](reth_primitives::InvalidTransactionError)
    /// for common errors variants.
    ///
    /// The transaction pool makes no additional assumptions about the validity
    /// of the transaction at the time of this call before it inserts it
    /// into the pool. However, the validity of this transaction is still
    /// subject to future (dynamic) changes enforced by the pool, for
    /// example nonce or balance changes. Hence, any validation checks must be
    /// applied in this function.
    ///
    /// See [TransactionValidationTaskExecutor] for a reference implementation.
    async fn validate_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::LimitOrder
    ) -> OrderValidationOutcome<Self::LimitOrder>;

    /// Validates a batch of transactions.
    ///
    /// Must return all outcomes for the given transactions in the same order.
    ///
    /// See also [Self::validate_transaction].
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
