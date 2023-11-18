use std::collections::{BTreeMap, HashMap};

use alloy_primitives::{Address, B256};
use composable::ComposableSearcherPool;

use self::searcher::VanillaSearcherPool;
use crate::{
    common::{BidAndAsks, OrderId, ParkedPool, PendingPool},
    PooledComposableOrder, PooledSearcherOrder
};
mod composable;
mod searcher;

pub struct SearcherPool<T: PooledSearcherOrder, C: PooledComposableOrder + PooledSearcherOrder> {
    /// Holds all non composable searcher order pools
    searcher_orders: VanillaSearcherPool<T>,
    /// Holds all composable searcher order pools
    composable_searcher_orders: ComposableSearcherPool<C>,
    /// used for easy update operations on Orders.
    all_order_ids: HashMap<OrderId, T>,
    /// used for nonce + lookup.
    user_to_id: HashMap<Address, Vec<OrderId>>,
    /// hash to pool location with identifier.
    order_hash_location: HashMap<B256, (OrderId, T)>,
    /// The size of the current transactions.
    size: SizeTracker
}
