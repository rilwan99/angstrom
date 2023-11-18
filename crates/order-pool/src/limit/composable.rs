use std::collections::HashMap;

use reth_primitives::B256;

use super::{LimitOrderLocation, LimitPoolError, PoolId};
use crate::{
    common::{OrderId, PendingPool},
    PooledComposableOrder
};

pub struct ComposableLimitPool<T: PooledComposableOrder>(HashMap<PoolId, PendingPool<T>>);

impl<T: PooledComposableOrder> ComposableLimitPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<LimitOrderLocation, LimitPoolError> {
        Ok(LimitOrderLocation::Composable)
    }

    pub fn remove_order(&mut self, tx_id: &OrderId) -> Option<T> {
        self.0
            .get_mut(&tx_id.pool_id)?
            .remove_order(tx_id.order_hash)
    }
}
