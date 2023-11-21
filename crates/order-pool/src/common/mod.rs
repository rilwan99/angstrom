use guard_types::orders::{OrderPriorityData, PooledOrder, ValidatedOrder};
mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

pub enum FilledOrders<L, CL, S, CS>
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
