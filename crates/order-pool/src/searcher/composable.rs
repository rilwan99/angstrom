use std::collections::{BTreeMap, HashMap};

use crate::{
    common::{OrderId, PoolId},
    traits::{PooledComposableOrder, PooledSearcherOrder}
};

pub struct ComposableSearcherPool<T: PooledComposableOrder + PooledSearcherOrder>(
    HashMap<PoolId, PendingPool<T>>
);

pub struct PendingPool<T: PooledSearcherOrder + PooledComposableOrder> {
    orders: BTreeMap<OrderId, T>
}
