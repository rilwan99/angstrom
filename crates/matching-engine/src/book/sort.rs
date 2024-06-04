/// There are lots of different ways we can sort the orders we get in, so let's make this modular
use super::order::Order;

pub enum SortStrategy {
    Unsorted,
    ByPriceByVolume,
}

impl Default for SortStrategy {
    fn default() -> Self {
        Self::Unsorted
    }
}

impl SortStrategy {
    pub fn sort_bids(&self, bids: &mut Vec<Order>) {
        match self {
            Self::ByPriceByVolume => {
                // Sort by price and then by volume - highest price first, highest volume first for same price
                bids.sort_by(|a, b| b.price().partial_cmp(&a.price()).unwrap_or(std::cmp::Ordering::Less)
                    .then(b.quantity(0.0).partial_cmp(&a.quantity(0.0)).unwrap_or(std::cmp::Ordering::Less)));
            },
            _ => ()
        }
    }

    pub fn sort_asks(&self, asks: &mut Vec<Order>) {
        match self {
            Self::ByPriceByVolume => {
                // Sort by price and then by volume - lowest price first, highest volume first for same price
                asks.sort_by(|a, b| a.price().partial_cmp(&b.price()).unwrap_or(std::cmp::Ordering::Less)
                    .then(b.quantity(0.0).partial_cmp(&a.quantity(0.0)).unwrap_or(std::cmp::Ordering::Less)));
            },
            _ => ()
        }
    } 
}