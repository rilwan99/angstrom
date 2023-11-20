use guard_types::orders::PooledOrder;
use validation::order::ValidatedOrder;

mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);

pub type ValidOrder<O> = ValidatedOrder<O, <O as PooledOrder>::ValidationData>;
