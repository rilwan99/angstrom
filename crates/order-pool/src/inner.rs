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
    >,
    <L as PooledOrder>::ValidationData: LimitOrderValidation,
    <CL as PooledOrder>::ValidationData: ComposableLimitOrderValidation,
    <S as PooledOrder>::ValidationData: SearcherOrderValidation,
    <CS as PooledOrder>::ValidationData: ComposableSearcherOrderValidation
{
    pub async fn validate_limit(
        &mut self,
        origin: OrderOrigin,
        order: L
    ) -> OrderValidationOutcome<L> {
        let res = self.validator.validate_order(origin, order).await;
        match res {
            OrderValidationOutcome::Valid { order, propagate } => {
                let a = order.data.priority_data();
            }
            _ => todo!()
        }
        todo!()
    }
}
