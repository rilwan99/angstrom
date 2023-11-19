use std::collections::HashMap;

use guard_types::orders::PooledOrder;

use super::{OrderId, ValidOrder};

pub struct ParkedPool<T: PooledOrder>(HashMap<OrderId, ValidOrder<T>>);

impl<T: PooledOrder> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(&mut self, order_id: &OrderId) -> Option<ValidOrder<T>> {
        self.0.remove(order_id)
    }

    pub fn new_order(&mut self, order: ValidOrder<T>) {
        let id = order.order_id();
        self.0.insert(id, order);
    }
}
