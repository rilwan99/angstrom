use std::collections::HashMap;

use super::OrderId;
use crate::PooledOrder;

pub struct ParkedPool<T: PooledOrder>(HashMap<OrderId, T>);

impl<T: PooledOrder> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(&mut self, order_id: &OrderId) -> Option<T> {
        self.0.remove(order_id)
    }

    pub fn new_order(&mut self, order: T) {
        let id = order.order_id();
        self.0.insert(id, order);
    }
}
