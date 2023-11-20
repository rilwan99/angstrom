use std::collections::HashMap;

use guard_types::orders::{OrderPriorityData, PooledOrder, ValidatedOrder};

use super::OrderId;

pub struct ParkedPool<O: PooledOrder>(HashMap<OrderId, ValidatedOrder<O, OrderPriorityData>>);

impl<O: PooledOrder> ParkedPool<O> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(
        &mut self,
        order_id: &OrderId
    ) -> Option<ValidatedOrder<O, OrderPriorityData>> {
        self.0.remove(order_id)
    }

    pub fn new_order(&mut self, order: ValidatedOrder<O, OrderPriorityData>) {
        let id = order.order_id();
        self.0.insert(id, order);
    }
}
