use std::{cmp::Reverse, collections::BTreeMap};

use reth_primitives::B256;
use revm::primitives::HashMap;
use tokio::sync::broadcast;

use crate::{common::BidAndAsks, PooledOrder};

pub struct PendingPool<T: PooledOrder> {
    /// all order hashes
    orders:                   HashMap<B256, T>,
    /// bids are sorted descending by price, TODO: This should be binned into
    /// ticks based off of the underlying pools params
    bids:                     BTreeMap<u128, B256>,
    /// asks are sorted ascending by price,  TODO: This should be binned into
    /// ticks based off of the underlying pools params
    asks:                     BTreeMap<Reverse<u128>, B256>,
    /// Notifier for new transactions
    new_transaction_notifier: broadcast::Sender<T>
}

impl<T: PooledOrder> PendingPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) {
        let hash = order.hash();
        let price = order.limit_price();
        if order.is_bid() {
            self.bids.insert(price, hash);
        } else {
            self.asks.insert(Reverse(price), hash);
        }

        self.orders.insert(hash, order.clone());
        // notifiy new orders
        let _ = self.new_transaction_notifier.send(order);
    }

    pub fn remove_order(&mut self, hash: B256) -> Option<T> {
        let order = self.orders.remove(&hash)?;
        let price = order.limit_price();

        if order.is_bid() {
            self.bids.remove(&price);
        } else {
            self.asks.remove(&Reverse(price));
        }

        Some(order)
    }

    pub fn fetch_all_orders(&self) -> Vec<&T> {
        self.orders.values().collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<&T> {
        self.bids
            .values()
            .map(|key| {
                self.orders
                    .get(key)
                    .expect("Had key but no value, this is a error")
            })
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<&T> {
        self.asks
            .values()
            .map(|key| {
                self.orders
                    .get(key)
                    .expect("Had key but no value, this is a error")
            })
            .collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> BidAndAsks<T> {
        self.bids
            .iter()
            .map(|(price, addr)| (price, self.orders.get(addr).unwrap()))
            .zip(
                self.asks
                    .iter()
                    .map(|(price, addr)| (price, self.orders.get(addr).unwrap()))
            )
            .map_while(
                |((bid_p, bid), (ask_p, ask))| {
                    if ask_p.0.le(bid_p) {
                        Some((bid, ask))
                    } else {
                        None
                    }
                }
            )
            .unzip()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<T> {
        todo!("Blocked until added tick impl")
    }
}
