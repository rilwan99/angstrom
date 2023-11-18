use std::collections::HashMap;

use reth_primitives::B256;

use super::TransactionId;
use crate::{pool::limit::LimitTx, PooledOrder};

pub struct ParkedPool<T: PooledOrder> {
    bids: HashMap<TransactionId, T>,
    asks: HashMap<TransactionId, T>
}

impl<T: PooledOrder> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        Ok(())
    }
}
