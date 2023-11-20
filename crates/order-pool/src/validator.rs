use std::{
    pin::Pin,
    task::{Context, Poll}
};

use futures_util::{stream::FuturesUnordered, Future, Stream, StreamExt};
use guard_types::orders::{OrderOrigin, PooledOrder};
use validation::order::{OrderValidationOutcome, OrderValidator};

pub enum ValidationResults<L, CL, S, CS>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder
{
    Limit(OrderValidationOutcome<L>),
    ComposableLimit(OrderValidationOutcome<CL>),
    Searcher(OrderValidationOutcome<S>),
    ComposableSearcher(OrderValidationOutcome<CS>)
}



pub struct Validator<L, CL, S, CS, V>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder,
    V: OrderValidator
{
    validator: V,
    pending: FuturesUnordered<
        Pin<Box<dyn Future<Output = ValidationResults<L, CL, S, CS>> + Send + Sync>>
    >
}

impl<L, CL, S, CS, V> Validator<L, CL, S, CS, V>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    fn validate_order(&self, origin: OrderOrigin, transaction: L) {
        self.validator.validate_order(origin, transaction).map(|
        todo!()
    }

    /// Validates a batch of orders.
    ///
    /// Must return all outcomes for the given orders in the same order.

    fn validate_orders(&self, transactions: Vec<(OrderOrigin, L)>) {
    }

    fn validate_composable_order(&self, origin: OrderOrigin, transaction: CL) {
        todo!()
    }

    fn validate_composable_orders(&self, transactions: Vec<(OrderOrigin, CL)>) {
        transactions
            .into_iter()
            .for_each(|(origin, tx)| self.validate_composable_order(origin, tx))
    }

    fn validate_searcher_order(&self, origin: OrderOrigin, transaction: S) {
        todo!()
    }

    fn validate_searcher_orders(&self, transactions: Vec<(OrderOrigin, S)>) {
        transactions
            .into_iter()
            .for_each(|(origin, tx)| self.validate_searcher_order(origin, tx))
    }

    fn validate_composable_searcher_order(&self, origin: OrderOrigin, transaction: CS) {
        todo!()
    }

    fn validate_composable_searcher_orders(&self, transactions: Vec<(OrderOrigin, CS)>) {
        transactions
            .into_iter()
            .for_each(|(origin, tx)| self.validate_composable_searcher_order(origin, tx))
    }
}

impl<L, CL, S, CS, V> Stream for Validator<L, CL, S, CS, V>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    type Item = ValidationResults<L, CL, S, CS>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.pending.poll_next_unpin(cx)
    }
}
