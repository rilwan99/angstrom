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

pub struct VanillaSearcherPool<T: PooledSearcherOrder>(HashMap<PoolId, PendingPool<T>>);

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
