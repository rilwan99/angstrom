use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap}
};

use alloy_primitives::B256;

use crate::{
    common::{OrderId, PoolId},
    traits::PooledSearcherOrder
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArbPriorityData {
    pub donated: u128,
    pub volume:  u128,
    pub gas:     u128
}
/// Reverse ordering for arb priority data to sort donated value in descending
/// order
impl PartialOrd for ArbPriorityData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            other
                .donated
                .cmp(&self.donated)
                .then_with(|| other.volume.cmp(&self.volume))
        )
    }
}

impl Ord for ArbPriorityData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
pub struct VanillaSearcherPool<T: PooledSearcherOrder>(HashMap<PoolId, PendingPool<T>>);

pub struct PendingPool<T: PooledSearcherOrder> {
    orders:       HashMap<B256, T>,
    ordered_arbs: BTreeMap<ArbPriorityData, B256>
}

impl<T: PooledSearcherOrder> PendingPool<T> {
    pub fn insert_order(&mut self, priority_data: ArbPriorityData, order_id: B256) -> bool {
        if self.ordered_arbs.contains_key(&priority_data) {
            // Key already exists, reject the insertion
            false
        } else {
            // Key does not exist, insert the new order
            self.ordered_arbs.insert(priority_data, order_id);
            true
        }
    }
}
