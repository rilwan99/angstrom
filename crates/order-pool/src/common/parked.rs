use std::collections::HashMap;

use super::OrderId;
use crate::{common::BidAndAsks, PooledOrder};

pub struct ParkedPool<T: PooledOrder> {
    bids: HashMap<OrderId, T>,
    asks: HashMap<OrderId, T>
}

impl<T: PooledOrder> ParkedPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn remove_order(&mut self, order_id: &OrderId, is_bid: bool) -> Option<T> {
        if is_bid {
            self.bids.remove(order_id)
        } else {
            self.asks.remove(order_id)
        }
    }

    pub fn new_order(&mut self, order: T) {
        let id = order.order_id();
        if order.is_bid() {
            self.bids.insert(id, order);
        } else {
            self.asks.insert(id, order);
        }
    }
}
