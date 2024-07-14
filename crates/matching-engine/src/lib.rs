use std::collections::{HashMap, HashSet};

use angstrom_types::{
    consensus::{PreProposal, Proposal},
    primitive::PoolId,
    sol_bindings::grouped_orders::{GroupedVanillaOrder, OrderWithStorageData}
};
use book::OrderBook;
use cfmm::uniswap::MarketSnapshot;
use futures_util::future::BoxFuture;

pub mod book;
pub mod cfmm;
pub mod manager;
pub mod matcher;
pub mod simulation;
pub mod strategy;

pub use manager::MatchingManager;

pub trait MatchingEngineHandle: Send + Sync + Clone + Unpin + 'static {
    fn build_proposal(&self, preproposals: Vec<PreProposal>)
        -> BoxFuture<Result<Proposal, String>>;
}

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
    orders: HashSet<OrderWithStorageData<GroupedVanillaOrder>>
) -> OrderBook {
    let (bids, asks) = orders.into_iter().partition(|o| o.is_bid);

    OrderBook::new(id, amm, bids, asks, Some(book::sort::SortStrategy::ByPriceByVolume))
}
