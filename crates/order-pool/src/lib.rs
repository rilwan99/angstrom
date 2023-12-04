mod common;
mod config;
mod inner;
mod limit;
mod searcher;
pub mod traits;
mod validator;
use std::sync::Arc;

use alloy_primitives::TxHash;
use config::PoolConfig;
use guard_types::orders::{
    OrderPriorityData, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
    SearcherPriorityData, ValidatedOrder
};
pub use guard_utils::*;
use inner::OrderPoolInner;
use traits::OrderPool;
use validation::order::OrderValidator;

#[derive(Clone)]
pub struct Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    pool: Arc<OrderPoolInner<L, CL, S, CS, V>>
}

impl<L, CL, S, CS, V> Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder<ValidationData = ValidatedOrder<L, OrderPriorityData>>,
    CL: PooledComposableOrder
        + PooledLimitOrder<ValidationData = ValidatedOrder<CL, OrderPriorityData>>,

    S: PooledSearcherOrder<ValidationData = ValidatedOrder<S, SearcherPriorityData>>,
    CS: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = ValidatedOrder<CS, SearcherPriorityData>>,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    pub fn new(validator: V, config: PoolConfig) -> Self {
        let pool = Arc::new(OrderPoolInner::new(validator, config));

        Self { pool }
    }

    pub fn inner(&self) -> &OrderPoolInner<L, CL, S, CS, V> {
        &self.pool
    }
}

//TODO: Tmrw, finish the impl of the pool handle that impls the Pool trait
// which will be the pool api

impl<L, CL, S, CS, V> OrderPool for Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    type ComposableLimitOrder = CL;
    type ComposableSearcherOrder = CS;
    type LimitOrder = L;
    type SearcherOrder = S;

    #[allow(dead_code)]
    fn get_pooled_orders_by_hashes(
        &self,
        tx_hashes: Vec<TxHash>,
        limit: Option<usize>
    ) -> Vec<PooledOrder> {
        todo!()
    }
}
