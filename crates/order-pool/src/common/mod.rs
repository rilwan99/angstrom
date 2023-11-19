mod size;
use alloy_primitives::Address;
pub use size::*;

pub type BidAndAsks<'a, T> = (Vec<&'a T>, Vec<&'a T>);
pub type PoolId = Address;
