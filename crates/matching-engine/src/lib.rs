use std::collections::HashMap;

use angstrom_types::{
    orders::PoolOrder,
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

pub fn build_books<'a>(
    source_orders: Vec<OrderWithStorageData<GroupedVanillaOrder>>
) -> HashMap<PoolId, OrderBook<'a>> {
    HashMap::new()
}

pub fn build_book<'a, O: PoolOrder>(
    id: PoolId,
    amm: Option<MarketSnapshot>,
    orders: BidsAndAsks<O>
) -> OrderBook<'a> {
    let mut book =
        OrderBook::new(amm, vec![], vec![], Some(book::sort::SortStrategy::ByPriceByVolume));
    book
}
