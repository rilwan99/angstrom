use std::collections::HashMap;

use reth_primitives::B256;

use super::{
    pending::PendingPool, LimitOrderLocation, LimitPoolError, LimitTx, PoolId, TransactionId
};

pub struct ComposableLimitPool<T: LimitTx>(HashMap<PoolId, PendingPool<T>>);

impl<T: LimitTx> ComposableLimitPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<LimitOrderLocation, LimitPoolError> {
        Ok(LimitOrderLocation::Composable)
    }

    pub fn filled_order(&mut self, tx_id: &TransactionId) -> Option<T> {
        self.0.get_mut(&tx_id.pool_id)?.filled_order(tx_id.order_hash)
    }
}
