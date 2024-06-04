//! basic book impl so we can benchmark

use self::{order::Order, sort::SortStrategy};
use crate::cfmm::uniswap::MarketSnapshot;

pub mod volume;
pub mod order;
pub mod sort;

pub type OrderVolume = f64;
pub type OrderPrice = f64;

pub struct OrderBook<'a> {
    amm: MarketSnapshot,
    bids: Vec<Order<'a>>,
    asks: Vec<Order<'a>>
}

impl<'a> OrderBook<'a> {
    pub fn new(amm: MarketSnapshot, mut bids: Vec<Order<'a>>, mut asks: Vec<Order<'a>>, sort: Option<SortStrategy>) -> Self {
        // Use our sorting strategy to sort our bids and asks
        let strategy = sort.unwrap_or_default();
        strategy.sort_bids(&mut bids);
        strategy.sort_asks(&mut asks);
        Self { amm, bids, asks }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cfmm::uniswap::SqrtPriceX96;

    #[test]
    fn can_construct_order_book() {
        // Very basic book construction test
        let bids = vec![];
        let asks = vec![];
        let amm = MarketSnapshot::new(vec![], SqrtPriceX96::from_float_price(0.0)).unwrap();
        OrderBook::new(amm, bids, asks, None);
    }
}
