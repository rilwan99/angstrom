use std::collections::HashMap;

use reth_primitives::B256;

use super::{pending::PendingPool, LimitPoolError, LimitTx, PoolId};

pub struct ComposableLimitPool<T: LimitTx>(HashMap<PoolId, PendingPool<T>>);

impl<T: LimitTx> ComposableLimitPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        Ok(())
    }

    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> Vec<T> {
        vec![]
    }
}
