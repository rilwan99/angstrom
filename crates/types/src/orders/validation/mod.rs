use std::fmt::Debug;

use derive_more::{AsRef, Deref};
pub mod limit;
pub mod searcher;
pub use limit::*;
pub use searcher::*;

use crate::orders::PooledOrder;

#[derive(Debug, AsRef, Deref, Clone)]
pub struct ValidatedOrder<O: PooledOrder, Data: Clone + Debug> {
    #[deref]
    #[as_ref]
    pub order:   O,
    pub pool_id: usize,
    pub is_bid:  bool,
    pub data:    Data
}
