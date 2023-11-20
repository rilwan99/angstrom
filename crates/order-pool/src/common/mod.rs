use guard_types::orders::{OrderPriorityData, PooledOrder, ValidatedOrder};
mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);
