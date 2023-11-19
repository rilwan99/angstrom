use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap}
};

use alloy_primitives::B256;
use guard_types::orders::{OrderId, PooledSearcherOrder};

use crate::common::PoolId;

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
    pub fn new() -> Self {
        Self { orders: HashMap::new(), ordered_arbs: BTreeMap::new() }
    }

    pub fn new_order(&mut self, order: T) {
        unreachable!();
        let order_id = order.order_id();
        let arb_data = ArbPriorityData {
            donated: order.donated(),
            volume:  order.volume(),
            gas:     order.gas()
        };
        self.ordered_arbs.insert(arb_data, order_id.hash);
        self.orders.insert(order_id.hash, order);
    }

    pub fn check_for_duplicates(&self, priority_data: ArbPriorityData) -> bool {
        !self.ordered_arbs.contains_key(&priority_data)
    }
}
