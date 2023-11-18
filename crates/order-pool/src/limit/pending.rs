use std::{cmp::Reverse, collections::BTreeMap};

use reth_primitives::B256;
use revm::primitives::HashMap;
use tokio::sync::broadcast;

use crate::{common::BidAndAsks, PooledOrder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderPriorityData {
    pub price:  u128,
    pub volume: u128,
    pub gas:    u128
}

impl PartialOrd for OrderPriorityData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.price.cmp(&other.price).then_with(|| {
            self.volume
                .cmp(&other.volume)
                .then_with(|| self.gas.cmp(&other.gas))
        }))
    }
}

impl Ord for OrderPriorityData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct PendingPool<T: PooledOrder> {
    /// all order hashes
    orders:                   HashMap<B256, T>,
    /// bids are sorted descending by price, TODO: This should be binned into
    /// ticks based off of the underlying pools params
    bids:                     BTreeMap<OrderPriorityData, Vec<B256>>,
    /// asks are sorted ascending by price,  TODO: This should be binned into
    /// ticks based off of the underlying pools params
    asks:                     BTreeMap<Reverse<OrderPriorityData>, Vec<B256>>,
    /// Notifier for new transactions
    new_transaction_notifier: broadcast::Sender<T>
}

impl<T: PooledOrder> PendingPool<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn new_order(&mut self, order: T) {
        let hash = order.hash();
        let priority = order.order_priority_data();

        if order.is_bid() {
            self.bids.entry(priority).or_default().push(hash);
        } else {
            self.asks.entry(Reverse(priority)).or_default().push(hash);
        }

        self.orders.insert(hash, order.clone());
        // notifiy new orders
        let _ = self.new_transaction_notifier.send(order);
    }

    pub fn remove_order(&mut self, hash: B256) -> Option<T> {
        let order = self.orders.remove(&hash)?;
        let priority = order.order_priority_data();

        if order.is_bid() {
            self.bids
                .entry(priority)
                .and_modify(|data| data.retain(|order_hash| order_hash != &hash));
        } else {
            self.asks
                .entry(Reverse(priority))
                .and_modify(|data| data.retain(|order_hash| order_hash != &hash));
        }

        Some(order)
    }

    pub fn fetch_all_orders(&self) -> Vec<&T> {
        self.orders.values().collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<&T> {
        self.bids
            .values()
            .flatten()
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
            .flatten()
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
            .flat_map(|(price, addrs)| {
                addrs
                    .into_iter()
                    .map(|addr| self.orders.get(addr).unwrap())
                    .zip(std::iter::repeat(price))
                    .collect::<Vec<_>>()
            })
            .zip(self.asks.iter().flat_map(|(price, addrs)| {
                addrs
                    .into_iter()
                    .map(|addr| self.orders.get(addr).unwrap())
                    .zip(std::iter::repeat(price))
                    .collect::<Vec<_>>()
            }))
            .map_while(
                |((bid, bid_p), (ask, ask_p))| {
                    if ask_p.0.price.le(&bid_p.price) {
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
