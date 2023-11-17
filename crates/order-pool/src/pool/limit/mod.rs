use std::collections::HashSet;

use guard_types::primitive::OrderType;
use reth_primitives::{Address, B256};

use self::{composable::ComposableLimitPool, limit::LimitPool, side::Side};

mod composable;
mod limit;
mod parked;
mod pending;
mod side;

pub trait LimitTx: Side {
    fn hash(&self) -> B256;
    fn get_pool(&self) -> Address;
    fn get_type(&self) -> OrderType;
    fn is_valid(&self) -> bool;
    fn get_id(&self) -> TransactionId;
}

pub struct TransactionId {
    /// Hash of the order. Needed to check for inclusion
    order_hash: B256,
    /// Nonce of the order
    nonce:      u64,
    /// when the order expires
    expiry:     u128,

}

#[derive(Debug, thiserror::Error)]
pub enum LimitPoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId)
}

pub enum LimitOrderLocation {
    Composable,
    LimitParked,
    LimitPending
}

type PoolId = Address;

struct SizeTracker {
    pub max:     usize,
    pub current: usize
}

pub struct LimitOrderPool<T: LimitTx> {
    composable_orders: ComposableLimitPool<T>,
    limit_orders:      LimitPool<T>,
    all_order_ids:     HashSet<TransactionId>,
    size:              SizeTracker
}

impl<T: LimitTx> LimitOrderPool<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            composable_orders: ComposableLimitPool::new(),
            limit_orders:      LimitPool::new(),
            all_order_ids:     HashSet::new(),
            size:              SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        Ok(())
    }

    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> Vec<T> {
        self.composable_orders.filled_orders(orders)
    }

    pub fn get_all_order(&mut self) -> Vec<T> {
        todo!()
    }
}
