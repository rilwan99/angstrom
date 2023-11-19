use std::{cmp::Reverse, collections::BTreeMap};

use bitflags::Flags;
use guard_types::orders::{OrderPriorityData, PooledOrder};
use reth_primitives::B256;
use revm::primitives::HashMap;
use tokio::sync::broadcast;

use crate::common::BidAndAsks;

pub struct PendingPool<T: PooledOrder> {
    /// all order hashes
    orders:                   HashMap<B256, T>,
    /// bids are sorted descending by price, TODO: This should be binned into
    /// ticks based off of the underlying pools params
    bids:                     BTreeMap<Reverse<OrderPriorityData>, B256>,
    /// asks are sorted ascending by price,  TODO: This should be binned into
    /// ticks based off of the underlying pools params
    asks:                     BTreeMap<OrderPriorityData, B256>,
    /// Notifier for new transactions
    new_transaction_notifier: broadcast::Sender<T>
}

impl<T: PooledOrder> PendingPool<T> {
    pub fn new(notifier: broadcast::Sender<T>) -> Self {
        Self {
            orders:                   HashMap::new(),
            bids:                     BTreeMap::new(),
            asks:                     BTreeMap::new(),
            new_transaction_notifier: notifier
        }
    }

    pub fn new_order(&mut self, order: T) {
        let hash = order.hash();
        let priority = order.order_priority_data();

        if order.is_bid() {
            if self.bids.contains_key(&Reverse(priority)) {
                return //TODO: error this
            }

            self.bids.insert(Reverse(priority), hash);
        } else {
            if self.asks.contains_key(&priority) {
                return //TODO: error this
            }

            self.asks.insert(priority, hash);
        }

        self.orders.insert(hash, order.clone());
        // notifiy new orders
        let _ = self.new_transaction_notifier.send(order);
    }

    pub fn remove_order(&mut self, hash: B256) -> Option<T> {
        let order = self.orders.remove(&hash)?;
        let priority = order.order_priority_data();

        if order.is_bid() {
            self.bids.remove(&Reverse(priority))?;
        } else {
            self.asks.remove(&priority)?;
        }

        Some(order)
    }

    pub fn fetch_all_orders(&self) -> Vec<&T> {
        self.orders.values().collect()
    }

    pub fn fetch_all_bids(&self) -> Vec<&T> {
        self.bids
            .values()
            .map(|v| {
                self.orders
                    .get(v)
                    .expect("Had key but no value, this is a error")
            })
            .collect()
    }

    pub fn fetch_all_asks(&self) -> Vec<&T> {
        self.asks
            .values()
            .map(|v| {
                self.orders
                    .get(v)
                    .expect("Had key but no value, this is a error")
            })
            .collect()
    }

    pub fn fetch_all_bids_meta(&self) -> Vec<(OrderPriorityData, B256)> {
        self.bids.iter().map(|k| (k.0 .0, *k.1)).collect()
    }

    pub fn fetch_all_asks_meta(&self) -> Vec<(OrderPriorityData, B256)> {
        self.asks.iter().map(|k| (*k.0, *k.1)).collect()
    }

    /// Fetches supply and demand intersection
    pub fn fetch_intersection(&self) -> BidAndAsks<T> {
        // TODO: this will change when we tick bin, waiting till then
        // self.bids
        //     .iter()
        //     .map(|(price, addr)| (price, self.orders.get(addr).unwrap()))
        //     .zip(
        //         self.asks
        //             .iter()
        //             .map(|(price, addr)| (price,
        // self.orders.get(addr).unwrap()))     )
        //     .map_while(
        //         |((bid_p, bid), (ask_p, ask))| {
        //             if ask_p.price.le(&bid_p.0.price) {
        //                 Some((bid, ask))
        //             } else {
        //                 None
        //             }
        //         }
        //     )
        //     .unzip()
        todo!()
    }

    /// Fetches supply and demand intersection with a tick price buffer
    pub fn fetch_intersection_with_buffer(&self, _buffer: u8) -> BidAndAsks<T> {
        todo!("Blocked until added tick impl")
    }
}

#[cfg(test)]
pub mod test {
    use std::cmp::Ordering;

    use alloy_primitives::{Address, U256};
    use guard_types::orders::*;
    use rand::Rng;

    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct NoopOrder {
        price:    u128,
        volume:   u128,
        gas:      u128,
        hash:     B256,
        is_bid:   bool,
        nonce:    u64,
        deadline: u128
    }

    impl PooledOrder for NoopOrder {
        type ValidationData = ();

        fn order_priority_data(&self) -> OrderPriorityData {
            OrderPriorityData { price: self.price, volume: self.volume, gas: self.gas }
        }

        fn is_bid(&self) -> bool {
            self.is_bid
        }

        fn is_valid(&self) -> bool {
            true
        }

        fn order_id(&self) -> OrderId {
            OrderId {
                address:  Address::ZERO,
                pool_id:  Address::ZERO,
                hash:     self.hash,
                nonce:    self.nonce,
                deadline: self.deadline
            }
        }

        fn hash(&self) -> alloy_primitives::TxHash {
            self.hash
        }

        fn from(&self) -> Address {
            Address::ZERO
        }

        fn size(&self) -> usize {
            69
        }

        fn nonce(&self) -> alloy_primitives::U256 {
            U256::from(self.nonce)
        }

        fn deadline(&self) -> alloy_primitives::U256 {
            U256::from(self.deadline)
        }

        fn chain_id(&self) -> Option<u64> {
            None
        }

        fn amount_in(&self) -> u128 {
            0
        }

        fn limit_price(&self) -> u128 {
            self.price
        }

        fn amount_out_min(&self) -> u128 {
            0
        }

        fn encoded_length(&self) -> usize {
            4123
        }
    }

    pub fn generate_noop_orders(bids: usize, asks: usize) -> Vec<NoopOrder> {
        let mut res = Vec::with_capacity(bids + asks);
        let mut rng = rand::thread_rng();
        for _ in 0..bids {
            res.push(NoopOrder {
                price:    rng.gen(),
                volume:   rng.gen(),
                deadline: rng.gen(),
                nonce:    rng.gen(),
                hash:     rng.gen(),
                is_bid:   true,
                gas:      rng.gen()
            })
        }

        for _ in 0..asks {
            res.push(NoopOrder {
                price:    rng.gen(),
                volume:   rng.gen(),
                deadline: rng.gen(),
                nonce:    rng.gen(),
                hash:     rng.gen(),
                is_bid:   false,
                gas:      rng.gen()
            })
        }

        res
    }

    pub fn init_pool_with_data(
        sender: broadcast::Sender<NoopOrder>,
        bids: usize,
        asks: usize
    ) -> PendingPool<NoopOrder> {
        let mut pool = PendingPool::new(sender);
        let orders = generate_noop_orders(bids, asks);
        orders.into_iter().for_each(|order| pool.new_order(order));

        pool
    }

    macro_rules! order_priority {
        ($price:expr, $vol:expr, $gas:expr) => {
            OrderPriorityData { price: $price, volume: $vol, gas: $gas }
        };
    }

    #[test]
    pub fn verify_order_priority_cmp() {
        let p0 = order_priority!(101, 1000, 10);
        let p1 = order_priority!(100, 1001, 10);
        let p2 = order_priority!(99, 1001, 10);

        let p3 = order_priority!(100, 1001, 10);
        let p4 = order_priority!(100, 1000, 10);
        let p5 = order_priority!(100, 999, 10);

        let p6 = order_priority!(100, 1000, 11);
        let p7 = order_priority!(100, 1000, 10);
        let p8 = order_priority!(100, 1000, 9);

        let gt = Ordering::Greater;
        let lt = Ordering::Less;
        let eq = Ordering::Equal;

        assert_eq!(p1.cmp(&p0), lt);
        assert_eq!(p1.cmp(&p1), eq);
        assert_eq!(p1.cmp(&p2), gt);

        assert_eq!(p4.cmp(&p3), lt);
        assert_eq!(p4.cmp(&p4), eq);
        assert_eq!(p4.cmp(&p5), gt);

        assert_eq!(p7.cmp(&p6), lt);
        assert_eq!(p7.cmp(&p7), eq);
        assert_eq!(p7.cmp(&p8), gt);
    }

    macro_rules! is_ordered {
        ($list:expr, $cmp:tt, $field:ident) => {
            let mut last = $list.remove(0);
            for entry in $list {
                assert!(entry.$field $cmp last.$field);
                last = entry ;
            }
        };
    }

    #[test]
    pub fn verify_order_insertion() {
        let (tx, mut rx) = broadcast::channel(15);
        let pool = init_pool_with_data(tx, 5, 5);

        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }
        // verify all orders where sent via channel
        assert_eq!(count, 10);

        // assert price order
        let mut asks = pool.asks.keys().collect::<Vec<_>>();
        is_ordered!(asks, >=, price);

        // assert price order
        let mut bids = pool.bids.keys().map(|k| k.0).collect::<Vec<_>>();
        is_ordered!(bids, <=, price);
    }

    #[test]
    pub fn test_remove_bid() {
        let (tx, mut rx) = broadcast::channel(1500);
        let mut pool = init_pool_with_data(tx, 500, 500);

        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }

        let mut bids = pool.fetch_all_bids_meta();
        let (_, hash) = bids.remove(0);

        let order = pool.remove_order(hash);

        assert!(order.is_some());

        assert!(pool
            .fetch_all_bids_meta()
            .into_iter()
            .find(|(_, has_hash)| has_hash == &hash)
            .is_none());

        // verify all orders where sent via channel
        assert_eq!(count, 1000);
    }

    #[test]
    pub fn test_remove_ask() {
        let (tx, mut rx) = broadcast::channel(1500);
        let mut pool = init_pool_with_data(tx, 500, 500);

        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }

        // verify all orders where sent via channel
        assert_eq!(count, 1000);

        let mut asks = pool.fetch_all_asks_meta();
        let (_, hash) = asks.remove(0);

        let order = pool.remove_order(hash);

        assert!(order.is_some());

        assert!(pool
            .fetch_all_asks_meta()
            .into_iter()
            .find(|(_, has_hash)| has_hash == &hash)
            .is_none());
    }
}
