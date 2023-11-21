use std::collections::{BTreeMap, HashMap};

use guard_types::{
    orders::{
        OrderId, OrderLocation, PooledComposableOrder, PooledSearcherOrder, SearcherPriorityData,
        ValidatedOrder
    },
    primitive::PoolId
};

use super::SearcherPoolError;
pub struct ComposableSearcherPool<O: PooledComposableOrder + PooledSearcherOrder>(
    Vec<PendingPool<O>>
);

impl<T: PooledComposableOrder + PooledSearcherOrder> ComposableSearcherPool<T> {
    pub fn new() -> Self {
        todo!()
    }
}

pub struct PendingPool<O: PooledSearcherOrder + PooledComposableOrder> {
    ordered_arbs: BTreeMap<SearcherPriorityData, O>
}

impl<O: PooledSearcherOrder + PooledComposableOrder> PendingPool<O>
where
    O: PooledSearcherOrder<ValidationData = ValidatedOrder<O, SearcherPriorityData>>
{
    pub fn new() -> Self {
        Self { ordered_arbs: BTreeMap::new() }
    }

    pub fn add_order(
        &mut self,
        order: ValidatedOrder<O, SearcherPriorityData>
    ) -> Result<OrderLocation, SearcherPoolError> {
        todo!()
    }

    pub fn check_for_duplicates(&self, priority_data: SearcherPriorityData) -> bool {
        !self.ordered_arbs.contains_key(&priority_data)
    }
}
