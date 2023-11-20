use std::collections::HashMap;

use guard_types::orders::PooledOrder;

use super::OrderId;
use crate::common::ValidOrder;

pub struct ParkedPool<O: PooledOrder>(HashMap<OrderId, ValidOrder<O>>);

impl<O: PooledOrder> ParkedPool<O> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(&mut self, order_id: &OrderId) -> Option<ValidOrder<O>> {
        self.0.remove(order_id)
    }

    pub fn new_order(&mut self, order: ValidOrder<O>) {
        let id = order.order_id();
        self.0.insert(id, order);
    }
}
