use std::collections::HashMap;

use reth_primitives::B256;

use super::{LimitPoolError, TransactionId};
use crate::pool::limit::LimitTx;

pub struct ParkedPool<T: LimitTx> {
    bids: HashMap<TransactionId, T>,
    asks: HashMap<TransactionId, T>
}

impl<T: LimitTx> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        Ok(())
    }
}
