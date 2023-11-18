use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap}
};

use reth_primitives::B256;
use revm::primitives::HashMap;
use tokio::sync::broadcast;

use super::OrderPrice;
use crate::PooledOrder;

pub struct PendingPool<T: PooledOrder> {
    /// all order hashes
    orders:                   HashMap<B256, T>,
    /// bids are sorted descending by price
    bids:                     BTreeMap<OrderPrice, B256>,
    /// asks are sorted ascending by price
    asks:                     BTreeMap<Reverse<OrderPrice>, B256>,
    // Notifier for new transactions
    new_transaction_notifier: broadcast::Sender<T>
}

impl<T: PooledOrder> PendingPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) {
        let hash = order.hash();
        let price = order.price();
        if order.is_ask() {
            self.asks.insert(Reverse(price), hash);
        } else {
            self.bids.insert(price, hash);
        }

        self.orders.insert(hash, order.clone());
        let _ = self.new_transaction_notifier.send(order);
    }

    pub fn remove_order(&mut self, hash: B256) -> Option<T> {
        let order = self.orders.remove(&hash)?;
        let price = order.price();

        if order.is_bid() {
            self.bids.remove(&price);
        } else {
            self.asks.remove(&Reverse(price));
        }

        Some(order)
    }
}
