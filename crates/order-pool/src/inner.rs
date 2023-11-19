use std::marker::PhantomData;

use guard_types::orders::{
    OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder
};
use tokio::sync::mpsc::Sender;
use validation::order::{OrderValidationOutcome, OrderValidator};

use crate::{
    limit::LimitOrderPool, ComposableLimitOrderValidation, LimitOrderValidation,
    SearcherOrderValidation
};

pub struct OrderPoolInner<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    /// limit pool
    limit_pool:      LimitOrderPool<L, CL>,
    /// event listeners
    event_listeners: Vec<Sender<()>>,

    validator: V,

    _p: PhantomData<(S, CS, V)>
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
    <CS as PooledOrder>::ValidationData: ComposableLimitOrderValidation
{
    pub async fn validate_limit(
        &mut self,
        origin: OrderOrigin,
        order: L
    ) -> OrderValidationOutcome<L> {
        let res = self.validator.validate_order(origin, order).await;
        match res {
            OrderValidationOutcome::Valid { balance, state_nonce, order, propagate } => {
                let a = order.data.data();
            }
            _ => todo!()
        }
        todo!()
    }
}
