use std::collections::HashMap;

use super::LimitPoolError;
use crate::{identifier::TransactionId, pool::limit::LimitTx};

pub struct ParkedPool<T: LimitTx> {
    bids: HashMap<TransactionId, T>,
    asks: HashMap<TransactionId, T>,
}

impl<T: LimitTx> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {

        Ok(())
    }
}
