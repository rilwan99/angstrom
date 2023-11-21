use std::collections::HashMap;

use alloy_primitives::B256;
use guard_types::orders::{OrderPriorityData, PooledOrder, ValidatedOrder};

use super::OrderId;
pub struct ParkedPool<O: PooledOrder>(HashMap<B256, ValidatedOrder<O, OrderPriorityData>>);

impl<O: PooledOrder> ParkedPool<O> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(
        &mut self,
        order_hash: &B256
    ) -> Option<ValidatedOrder<O, OrderPriorityData>> {
        self.0.remove(order_hash)
    }

    pub fn new_order(&mut self, order: ValidatedOrder<O, OrderPriorityData>) {
        self.0.insert(order.hash(), order);
    }
}
