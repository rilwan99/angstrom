use std::collections::HashMap;

use super::OrderId;
use crate::PooledOrder;

pub struct ParkedPool<T: PooledOrder> {
    bids: HashMap<OrderId, T>,
    asks: HashMap<OrderId, T>
}

impl<T: PooledOrder> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) {}
}
