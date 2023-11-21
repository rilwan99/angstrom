use std::mem::transmute;

use guard_types::orders::{
    OrderLocation, OrderPriorityData, PooledComposableOrder, PooledLimitOrder, PooledOrder,
    PooledSearcherOrder, ValidatedOrder
};
mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

pub enum FilledOrder<L, CL, S, CS>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder
{
    Limit(L),
    ComposableLimit(CL),
    Searcher(S),
    ComposableSearcher(CS)
}

impl<L, CL, S, CS> FilledOrder<L, CL, S, CS>
where
    L: PooledOrder,
    CL: PooledOrder,
    S: PooledOrder,
    CS: PooledOrder
{
    pub fn new_limit(order: L) -> Self {
        Self::Limit(order)
    }

    pub fn new_composable_limit(order: CL) -> Self {
        Self::ComposableLimit(order)
    }

    pub fn new_searcher(order: S) -> Self {
        Self::Searcher(order)
    }

    pub fn new_composable_searcher(order: CS) -> Self {
        Self::Limit(order)
    }
}
