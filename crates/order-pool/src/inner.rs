use guard_types::orders::{
    ComposableLimitOrderValidation, ComposableSearcherOrderValidation, LimitOrderValidation,
    OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
    SearcherOrderValidation
};
use tokio::sync::mpsc::Sender;
use validation::order::{OrderValidationOutcome, OrderValidator};

use crate::{limit::LimitOrderPool, searcher::SearcherPool};

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

    validator: V
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
    pub async fn validate_limit(
        &mut self,
        origin: OrderOrigin,
        order: L
    ) -> OrderValidationOutcome<L> {
        todo!()
    }
}
