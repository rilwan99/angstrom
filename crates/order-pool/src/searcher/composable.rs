use std::collections::{BTreeMap, HashMap};

use guard_types::orders::{OrderId, PooledComposableOrder, PooledSearcherOrder};

use crate::common::PoolId;

pub struct ComposableSearcherPool<T: PooledComposableOrder + PooledSearcherOrder>(
    HashMap<PoolId, PendingPool<T>>
);

pub struct PendingPool<T: PooledSearcherOrder + PooledComposableOrder> {
    orders: BTreeMap<OrderId, T>
}
