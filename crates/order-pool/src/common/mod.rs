use guard_types::orders::{
    OrderLocation, OrderPriorityData, PoolOrder, PooledComposableOrder, PooledLimitOrder,
    PooledSearcherOrder, ValidatedOrder
};
mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

pub enum FilledOrder<L, CL, S, CS>
where
    L: PoolOrder,
    CL: PoolOrder,
    S: PoolOrder,
    CS: PoolOrder
{
    Limit(L),
    ComposableLimit(CL),
    Searcher(S),
    ComposableSearcher(CS)
}

impl<L, CL, S, CS> FilledOrder<L, CL, S, CS>
where
    L: PoolOrder,
    CL: PoolOrder,
    S: PoolOrder,
    CS: PoolOrder
{
    pub fn add_limit(order: L) -> Self {
        Self::Limit(order)
    }

    pub fn add_composable_limit(order: CL) -> Self {
        Self::ComposableLimit(order)
    }

    pub fn add_searcher(order: S) -> Self {
        Self::Searcher(order)
    }

    pub fn add_composable_searcher(order: CS) -> Self {
        Self::ComposableSearcher(order)
    }
}
