//! basic book impl so we can benchmark

use malachite::num::basic::traits::Zero;

use crate::cfmm::UniswapV4Book;

pub mod volume;
pub mod order;

#[derive(Debug)]
pub struct Order {
    amount: f64,
    price:  f64
}

impl Order {
    pub fn new(amount: f64, price: f64) -> Self {
        Self { amount, price }
    }
}

trait OrderList {
    fn next_limit(&self) -> Option<f64>;
}

impl OrderList for Vec<Order> {
    fn next_limit(&self) -> Option<f64> {
        let order = self.last()?;
        Some(order.price)
    }
}

#[derive(Debug)]
pub struct BigBoyBook<const BOUND: usize> {
    amm:  UniswapV4Book<BOUND>,
    bids: Vec<Order>,
    asks: Vec<Order>
}

impl<const BOUND: usize> BigBoyBook<BOUND> {
    pub fn new(amm: UniswapV4Book<BOUND>, bids: Vec<Order>, asks: Vec<Order>) -> Self {
        // We're assuming right now that everything is sorted coming in
        // bids are sorted by price in increasing order from 0... (decreasing order from the END of the vector)
        // asks are sorted by price in decreasing order from 0... (increasing order from the END of the vector)
        assert!(
            bids.windows(2).all(|w| w[0].price <= w[1].price)
                && asks.windows(2).all(|w| w[0].price >= w[1].price)
        );
        BigBoyBook { amm, bids, asks }
    }

    pub fn uniform_clearing_lob(&self) -> (Option<f64>, f64) {
        let mut q = f64::ZERO;
        let mut p = None;

        // If either list is empty we can just return here
        if self.asks.is_empty() || self.bids.is_empty() { return (p, q); }
        
        // Iterate from the back because that's the order that we need
        let mut a_iter = self.asks.iter().rev();
        let mut b_iter = self.bids.iter().rev();

        let mut a = a_iter.next().unwrap();
        let mut b = b_iter.next().unwrap();

        // This is used as a dumping ground for when I need to make a "remainder" Order to fill in
        let mut remainder_order = Order { amount: f64::ZERO, price: f64::ZERO };

        while a.price <= b.price {
            let matched = a.amount.min(b.amount);
            q += matched;
            let excess = b.amount - a.amount;
            if excess == f64::ZERO {
                p = Some((a.price + b.price) / 2.0);
                let Some(new_a) = a_iter.next() else { break; };
                let Some(new_b) = b_iter.next() else { break; };
                a = new_a;
                b = new_b;
            } else if excess > f64::ZERO {
                p = Some(b.price);
                let Some(new_a) = a_iter.next() else { break; };
                a = new_a;
                b = a; // Convince the borrow checker we aren't going to modify local_bid
                remainder_order.amount = b.amount - matched;
                remainder_order.price = b.price;
                b = &remainder_order;
            } else {
                p = Some(a.price);
                let Some(new_b) = b_iter.next() else { break; };
                b = new_b;
                a = b; // Convince the borrow checker we aren't going to modify local_bid
                remainder_order.amount = a.amount - matched;
                remainder_order.price = a.price;
                a = &remainder_order;
            }

        }
        (p, q)
    }

    pub fn uniform_clearing_amm(&self) -> (Option<f64>, f64) {
        let p: Option<f64> = None;
        let q = f64::ZERO;

        let p_star = match self.uniform_clearing_lob() {
            (Some(p), _) => p,
            _ => return (p, q)
        };

        //let amm_orders = self.amm_to_order(p_star);
        

        (p, q)
    }

    pub fn fill_me(&mut self) -> (Option<f64>, f64) {
        let mut qty = f64::ZERO;
        let mut p = None;

        let mut next_bid = self.get_next_bid();
        let mut next_ask = self.get_next_ask();

        while let Some((ask, bid)) = next_ask.as_mut().zip(next_bid.as_mut()) {
            println!("\n");
            dbg!(&ask);
            dbg!(&bid);
            println!("self.amm.get_price(): {}", self.amm.get_price());

            let matched = ask.amount.min(bid.amount);
            qty += &matched;
            let excess = bid.amount - ask.amount;

            if excess > f64::ZERO {
                p = Some(bid.price);
                bid.amount -= matched;
                next_ask = self.get_next_ask();
                println!("fetching new ask: {:?}", &next_ask);
            } else if excess < f64::ZERO {
                p = Some(ask.price);
                ask.amount -= matched;
                next_bid = self.get_next_bid();
                println!("fetching new bid: {:?}", &next_bid);
            } else {
                p = Some(Self::avg_price(ask.price, bid.price));
                next_bid = self.get_next_bid();
                next_ask = self.get_next_ask();
            }
        }

        (p, qty)
    }

    fn avg_price(a: f64, b: f64) -> f64 {
        (a * b).sqrt()
    }

    // Solve: next bid & ask would be AMM *but* pending non-emtpy order is limit =>
    // gib AMM discretized limit order.
    fn get_next_bid(&mut self) -> Option<Order> {
        let (_, bid_is_amm) = self.next_bid_price();
        let (ask_price, ask_is_amm) = self.next_ask_price();

        if ask_is_amm && bid_is_amm {
            return None;
        }

        if bid_is_amm {
            let limit = self.bids.next_limit().unwrap_or(ask_price);
            let bid_amm = self.amm_to_order(limit);
            dbg!(&bid_amm);
            return Some(bid_amm);
        }

        self.bids.pop()
    }

    fn get_next_ask(&mut self) -> Option<Order> {
        let (bid_price, bid_is_amm) = self.next_bid_price();
        let (_, ask_is_amm) = self.next_ask_price();

        if ask_is_amm && bid_is_amm {
            return None;
        }

        if ask_is_amm {
            let limit = self.asks.next_limit().unwrap_or(bid_price);
            let ask_amm = self.amm_to_order(limit);
            dbg!(&ask_amm);
            return Some(ask_amm);
        }

        self.asks.pop()
    }

    fn amm_to_order(&mut self, limit: f64) -> Order {
        // TODO: Proper error handling.
        let (dx, dy) = self
            .amm
            .swap_to_limit(limit)
            .expect("Read out-of-bound tick");
        Order { amount: dx.abs(), price: -(dy / dx) }
    }

    fn next_bid_price(&self) -> (f64, bool) {
        let amm_price = self.amm.get_price();
        match self.bids.next_limit() {
            Some(limit) if limit >= amm_price => (limit, false),
            _ => (amm_price, true)
        }
    }

    fn next_ask_price(&self) -> (f64, bool) {
        let amm_price = self.amm.get_price();
        match self.asks.next_limit() {
            Some(limit) if limit <= amm_price => (limit, false),
            _ => (amm_price, true)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cfmm::PoolRange;

    #[test]
    fn test_matching() {
        println!("\nmatching");
        let ranges = [
            PoolRange::new(2000.0, 2100.0, 1000.0),
            PoolRange::new(2100.0, 2200.0, 1000.0),
            PoolRange::new(2200.0, 2300.0, 1000.0),
            PoolRange::new(2300.0, 2400.0, 1000.0),
            PoolRange::new(2400.0, 2500.0, 1000.0)
        ];

        let amm = UniswapV4Book::new(2250.0, 2, ranges);

        let mut book =
            BigBoyBook::new(amm, vec![Order::new(1.4, 2240.0)], vec![Order::new(2.4, 1800.0)]);

        let out = book.fill_me();

        println!("out: {:?}", out);

        dbg!(&book);
    }
}
