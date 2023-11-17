use super::{LimitPoolError, LimitTx};

pub struct PendingPool<T: LimitTx> {
    bids: Vec<T>,
    asks: Vec<T>
}

impl<T: LimitTx> PendingPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {

        Ok(())
    }
}
