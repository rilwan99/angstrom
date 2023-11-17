use std::{cmp::Reverse, collections::BinaryHeap};

use super::{LimitPoolError, LimitTx};

pub struct PendingPool<T: LimitTx> {
    /// bids are sorted descending by price
    bids: BinaryHeap<T>,
    /// asks are sorted ascending by price
    asks: BinaryHeap<Reverse<T>>
}

impl<T: LimitTx> PendingPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        if order.is_ask() {
            self.asks.push(Reverse(order));
        } else {
            self.bids.push(order);
        }

        Ok(())
    }
}
