use std::collections::HashMap;

use angstrom_types::{
    orders::{OrderId, OrderLocation, PooledSearcherOrder, SearcherPriorityData},
    primitive::PoolId
};

use super::{pending::PendingPool, SearcherPoolError};
use crate::common::ValidOrder;

pub struct VanillaSearcherPool<O: PooledSearcherOrder> {
    sub_pools: HashMap<PoolId, PendingPool<O>>
}

impl<O: PooledSearcherOrder> VanillaSearcherPool<O>
where
    O: PooledSearcherOrder<ValidationData = SearcherPriorityData>
{
    pub fn new(ids: &[PoolId]) -> Self {
        let sub_pools = ids.iter().map(|id| (*id, PendingPool::new())).collect();

        VanillaSearcherPool { sub_pools }
    }

    pub fn add_order(
        &mut self,
        order: ValidOrder<O>
    ) -> Result<OrderLocation, SearcherPoolError<O>> {
        self.sub_pools
            .get_mut(&order.pool_id())
            .ok_or(SearcherPoolError::NoPool(order.pool_id))
            .and_then(|pool| pool.add_order(order))
    }

    pub fn remove_order(
        &mut self,
        order_id: &OrderId
    ) -> Result<ValidOrder<O>, SearcherPoolError<O>> {
        let pool = self
            .sub_pools
            .get_mut(&order_id.pool_id)
            .ok_or(SearcherPoolError::NoPool(order_id.pool_id))?;

        pool.remove_order(order_id.hash)
            .ok_or(SearcherPoolError::OrderNotFound(order_id.hash))
    }

    pub fn get_winning_orders(&self) -> Vec<ValidOrder<O>> {
        self.sub_pools
            .values()
            .filter_map(|pool| pool.winning_order())
            .cloned()
            .collect()
    }
}
