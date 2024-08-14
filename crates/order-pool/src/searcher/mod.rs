use std::collections::HashMap;

use angstrom_types::{
    orders::OrderId,
    primitive::PoolId,
    sol_bindings::{grouped_orders::OrderWithStorageData, sol::TopOfBlockOrder}
};
use pending::PendingPool;

use crate::common::SizeTracker;

mod pending;

#[allow(dead_code)]
pub const SEARCHER_POOL_MAX_SIZE: usize = 15;

#[derive(Default)]
pub struct SearcherPool {
    /// Holds all non composable searcher order pools
    searcher_orders: HashMap<PoolId, PendingPool>,
    /// The size of the current transactions.
    size:            SizeTracker
}

impl SearcherPool {
    pub fn new(ids: &[PoolId], max_size: Option<usize>) -> Self {
        let searcher_orders = ids.iter().map(|id| (*id, PendingPool::new())).collect();
        Self { searcher_orders, size: SizeTracker { max: max_size, current: 0 } }
    }

    pub fn add_searcher_order(
        &mut self,
        order: OrderWithStorageData<TopOfBlockOrder>
    ) -> Result<(), SearcherPoolError> {
        let size = order.size();
        if !self.size.has_space(size) {
            return Err(SearcherPoolError::MaxSize)
        }

        self.searcher_orders
            .get_mut(&order.pool_id)
            .ok_or_else(|| SearcherPoolError::NoPool(order.pool_id))?
            .add_order(order);

        Ok(())
    }

    pub fn remove_order(&mut self, id: &OrderId) -> Option<OrderWithStorageData<TopOfBlockOrder>> {
        self.searcher_orders
            .get_mut(&id.pool_id)
            .and_then(|pool| pool.remove_order(id.hash))
    }

    pub fn get_all_orders(&self) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        self.searcher_orders
            .values()
            .flat_map(|p| p.get_all_orders())
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SearcherPoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0} ")]
    NoPool(PoolId),
    #[error(transparent)]
    Unknown(#[from] eyre::Error)
}
