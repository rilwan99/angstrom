use std::fmt::Debug;

use derive_more::{AsRef, Deref};
pub mod limit;
pub mod searcher;
use alloy_primitives::{Address, B256, U256};
pub use limit::*;
pub use searcher::*;

use crate::{orders::PooledOrder, primitive::PoolId};

#[derive(Debug, AsRef, Deref, Clone)]
pub struct ValidatedOrder<O: PooledOrder, Data: Clone + Debug> {
    #[deref]
    #[as_ref]
    pub order:    O,
    pub pool_id:  usize,
    pub is_bid:   bool,
    pub location: OrderLocation,
    pub data:     Data
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId {
    pub address:  Address,
    /// Pool id
    pub pool_id:  PoolId,
    /// Hash of the order. Needed to check for inclusion
    pub hash:     B256,
    /// Nonce of the order
    pub nonce:    U256,
    /// when the order expires
    pub deadline: U256,
    /// Order Location
    pub location: OrderLocation
}

impl<O: PooledOrder, Data: Clone + Debug> From<ValidatedOrder<O, Data>> for OrderId {
    fn from(order: ValidatedOrder<O, Data>) -> Self {
        Self {
            address:  order.order.from(),
            pool_id:  order.pool_id,
            hash:     order.order.hash(),
            nonce:    order.order.nonce(),
            deadline: order.order.deadline(),
            location: order.location
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderLocation {
    Composable,
    LimitParked,
    LimitPending,
    VanillaSearcher,
    ComposableSearcher
}
