use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap}
};

use alloy_primitives::B256;
use guard_types::{
    orders::{OrderId, OrderLocation, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder},
    primitive::PoolId
};

use super::{SearcherPoolError, V1_LP_POOlS, SEARCHER_POOL_MAX_SIZE};
use crate::common::SizeTracker;

pub struct VanillaSearcherPool<O: PooledSearcherOrder> {
    sub_pools: Vec<PendingPool<O>>
}

impl<O: PooledSearcherOrder> VanillaSearcherPool<O>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>
{
    pub fn new(max_size: Option<usize>) -> Self {
        let sub_pools = (0..max_size.unwrap_or(V1_LP_POOlS)) // Default to 15 if None
            .map(|_| PendingPool::new())
            .collect();
        VanillaSearcherPool { sub_pools }
    }

    #[allow(dead_code)]
    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, SearcherPriorityData>
    ) -> Result<OrderLocation, SearcherPoolError> {
        self.sub_pools
            .get_mut(order.pool_id())
            .ok_or(SearcherPoolError::NoPool(order.pool_id))
            .and_then(|pool| pool.add_order(order))
    }

    pub fn remove_order(&mut self, order_id: OrderId) -> Result<O, SearcherPoolError> {
        let pool = self
            .sub_pools
            .get_mut(order_id.pool_id)
            .ok_or(SearcherPoolError::NoPool(order_id.pool_id))?;

        pool.remove_order(order_id.hash)
            .ok_or(SearcherPoolError::OrderNotFound(order_id.hash))
    }

    #[allow(dead_code)]
    pub fn get_winning_orders(&self) -> Vec<O> {
        self.sub_pools
            .iter()
            .filter_map(|pool| pool.winning_order())
            .map(|validated_order| validated_order.order.clone())
            .collect()
    }
}

pub struct PendingPool<O: PooledSearcherOrder> {
    orders:        HashMap<B256, ValidatedOrder<O, SearcherPriorityData>>,
    ordered_arbs:  BTreeMap<SearcherPriorityData, B256>,
    _size_tracker: SizeTracker
}

impl<O: PooledSearcherOrder> PendingPool<O>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>
{
    pub fn new() -> Self {
        Self {
            orders:        HashMap::new(),
            ordered_arbs:  BTreeMap::new(),
            _size_tracker: SizeTracker::new(Some(SEARCHER_POOL_MAX_SIZE))
        }
    }

    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, SearcherPriorityData>
    ) -> Result<OrderLocation, SearcherPoolError> {
        let priority_data = order.priority_data();
        let hash = order.hash();
        self.check_for_duplicates(&priority_data)?;

        self.orders.insert(hash, order);

        self.ordered_arbs.insert(priority_data, hash);

        Ok(OrderLocation::VanillaSearcher)
    }

    pub fn remove_order(&mut self, hash: B256) -> Option<O> {
        let order = self.orders.remove(&hash)?;
        //TODO: why fetch when we could pass it as param, given that we do the initial
        // lookup?
        let priority = order.priority_data();

        self.ordered_arbs.remove(&priority);
        Some(order.order)
    }

    pub fn winning_order(&self) -> Option<&ValidatedOrder<O, SearcherPriorityData>> {
        self.ordered_arbs
            .first_key_value()
            .and_then(|(_, hash)| self.orders.get(hash))
    }

    /*pub fn add_orders(
        &mut self,
        orders: Vec<ValidatedOrder<O, SearcherPriorityData>>
    ) -> Result<Vec<OrderLocation>, SearcherPoolError> {
        orders
            .into_iter()
            .try_fold(Vec::new(), |mut locations, order| {
                locations.push(self.add_order(order)?);
                Ok(locations)
            })
    }*/

    pub fn check_for_duplicates(
        &self,
        priority_data: &SearcherPriorityData
    ) -> Result<(), SearcherPoolError> {
        self.ordered_arbs
            .contains_key(priority_data)
            .then(|| ())
            .ok_or(SearcherPoolError::DuplicateOrder)
    }
}
