use std::collections::{BTreeMap, HashMap};

use guard_types::{
    orders::{OrderId, PooledComposableOrder, PooledSearcherOrder},
    primitive::PoolId
};

pub struct ComposableSearcherPool<T: PooledComposableOrder + PooledSearcherOrder>(
    Vec<PendingPool<T>>
);

pub struct PendingPool<T: PooledSearcherOrder + PooledComposableOrder> {
    orders: BTreeMap<OrderId, T>
}
