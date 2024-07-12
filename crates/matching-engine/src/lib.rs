use std::collections::HashMap;

use angstrom_types::{
    primitive::PoolId,
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};
use book::OrderBook;
use cfmm::uniswap::MarketSnapshot;

pub mod book;
pub mod cfmm;
pub mod matcher;
pub mod simulation;
pub mod strategy;

pub fn build_books(
    source_orders: Vec<OrderWithStorageData<GroupedVanillaOrder>>
) -> HashMap<PoolId, OrderBook> {
    // let output = HashMap::new();
    for o in source_orders.into_iter() {}
    HashMap::new()
}

pub fn build_book(
    id: PoolId,
    amm: Option<MarketSnapshot>,
    orders: Vec<OrderWithStorageData<GroupedVanillaOrder>>
) -> OrderBook {
    let (bids, asks) = orders.into_iter().partition(|o| o.is_bid);

    OrderBook::new(amm, bids, asks, Some(book::sort::SortStrategy::ByPriceByVolume))
}
