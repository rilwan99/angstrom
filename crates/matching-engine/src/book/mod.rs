//! basic book impl so we can benchmark
use malachite::{
    num::basic::traits::{Two, Zero},
    Rational
};

use crate::cfmm::UniswapV4Book;

pub trait BookOrder: Ord {
    fn get_amount(&self) -> Rational;
    fn get_limit_price(&self) -> Rational;
    fn set_amount(&mut self, am: Rational);
    fn set_price(&mut self, price: Rational);
}

pub trait SearcherOrder {
    fn get_amount(&self) -> Rational;
    fn swap_direction(&self) -> bool;
}

// TODO: just placeholder, will be more complex, just for prelim benchmarking
pub struct BigBoyBook<const BOUND: usize, S: SearcherOrder, O: BookOrder> {
    underlying_book: UniswapV4Book<BOUND>,
    searcher_order:  S,
    bids:            Vec<O>,
    asks:            Vec<O>
}

type PriceQty = (Rational, Rational);

impl<const BOUND: usize, S: SearcherOrder, O: BookOrder> BigBoyBook<BOUND, S, O> {
    pub fn fill_me(mut self) -> Option<PriceQty> {
        let mut qty = Rational::ZERO;
        let mut p = Rational::ZERO;

        let mut a_idx = 0usize;
        let mut b_idx = 0usize;

        while self.asks.get(a_idx)?.get_limit_price() <= self.bids.get(b_idx)?.get_limit_price() {
            let a = &mut self.asks[a_idx];
            let b = &mut self.bids[b_idx];

            let matched = std::cmp::min(a.get_amount(), b.get_amount());
            qty += &matched;
            let excess = b.get_amount() - a.get_amount();

            if excess == Rational::ZERO {
                p = (a.get_limit_price() + b.get_limit_price()) / Rational::TWO;
                a_idx += 1;
                b_idx += 1;
            } else if excess > Rational::ZERO {
                p = b.get_limit_price();
                b.set_price(p.clone());
                b.set_amount(b.get_amount() - matched);
                a_idx += 1;
            } else {
                p = a.get_limit_price();
                a.set_price(p.clone());
                a.set_amount(a.get_amount() - matched);
                b_idx += 1;
            }
        }

        let rem =



        Some((p, qty))
    }
}
