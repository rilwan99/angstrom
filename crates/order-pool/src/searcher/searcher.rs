use std::collections::{BTreeMap, HashMap};

use crate::{
    common::{OrderId, PoolId},
    traits::PooledSearcherOrder
};

pub struct VanillaSearcherPool<T: PooledSearcherOrder>(HashMap<PoolId, PendingPool<T>>);

pub struct PendingPool<T: PooledSearcherOrder> {
    orders: BTreeMap<OrderId, T>
}
