use std::{
    pin::Pin,
    task::{Context, Poll}
};

use futures_util::{Future, Stream, StreamExt};
use guard_types::orders::{
    ComposableLimitOrderValidation, ComposableSearcherOrderValidation, LimitOrderValidation,
    OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
    SearcherOrderValidation
};
use tokio::sync::mpsc::Sender;
use validation::order::{OrderValidationOutcome, OrderValidator};

use crate::{
    limit::LimitOrderPool,
    searcher::SearcherPool,
    validator::{ValidationResults, Validator}
};

pub struct OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    limit_pool:   LimitOrderPool<L, CL>,
    sercher_pool: SearcherPool<S, CS>,

    //
    validator: Validator<L, CL, S, CS, V>
}

impl<L, CL, S, CS, V> OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    pub fn new_limit_order(&mut self, origin: OrderOrigin, order: L) {
        self.validator.validate_order(origin, order);
    }

    pub fn new_composable_limit(&mut self, origin: OrderOrigin, order: CL) {
        self.validator.validate_composable_order(origin, order);
    }
}

// impl Future for OrderPoolInner<>
impl<L, CL, S, CS, V> Stream for OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator,
    <L as PooledOrder>::ValidationData: LimitOrderValidation,
    <CL as PooledOrder>::ValidationData: ComposableLimitOrderValidation
{
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(next)) = self.validator.poll_next_unpin(cx) {
            match next {
                ValidationResults::Limit(limit) => {}
                ValidationResults::ComposableLimit(composable_limit) => {}
                ValidationResults::Searcher(searcher) => {}
                ValidationResults::ComposableSearcher(composable_searcher) => {}
            }
        }
        Poll::Pending
    }
}
