use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap}
};

use alloy_primitives::B256;
use guard_types::{
    orders::{OrderId, PooledSearcherOrder, SearcherPriorityData, ValidatedOrder},
    primitive::PoolId
};

use super::{SearcherOrderLocation, SearcherPoolError};

pub struct VanillaSearcherPool<O: PooledSearcherOrder> {
    sub_pools: Vec<PendingPool<O>>
}

impl<O: PooledSearcherOrder> VanillaSearcherPool<O>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>
{
    pub fn new(max_size: Option<usize>) -> Self {
        let sub_pools = (0..max_size.unwrap_or(15)) // Default to 15 if None
            .map(|_| PendingPool::new())
            .collect();
        VanillaSearcherPool { sub_pools }
    }
}

pub struct PendingPool<O: PooledSearcherOrder> {
    orders:       HashMap<B256, O>,
    ordered_arbs: BTreeMap<SearcherPriorityData, B256>
}

impl<O: PooledSearcherOrder> PendingPool<O>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>
{
    pub fn new() -> Self {
        Self { orders: HashMap::new(), ordered_arbs: BTreeMap::new() }
    }

    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, SearcherPriorityData>
    ) -> Result<SearcherOrderLocation, SearcherPoolError> {
        todo!()
    }

    pub fn check_for_duplicates(&self, priority_data: SearcherPriorityData) -> bool {
        !self.ordered_arbs.contains_key(&priority_data)
    }
}
