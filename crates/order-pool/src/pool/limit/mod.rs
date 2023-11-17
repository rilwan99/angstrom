use std::collections::{BTreeMap, HashMap, HashSet};

use guard_types::primitive::OrderType;
use reth_primitives::{alloy_primitives::Address, B256};

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
    fn is_expired(&self) -> bool;
    fn get_id(&self) -> TransactionId;
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct TransactionId {
    pub user_addr:  Address,
    /// Hash of the order. Needed to check for inclusion
    pub order_hash: B256,
    /// Nonce of the order
    pub nonce:      u64,
    /// when the order expires
    pub expiry:     u128
}

#[derive(Debug, thiserror::Error)]
pub enum LimitPoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId),
    #[error("already have a ordered with {0:?}")]
    DuplicateNonce(TransactionId)
}

pub enum LimitOrderLocation {
    Composable,
    LimitParked,
    LimitPending
}

type PoolId = Address;

struct SizeTracker {
    pub max:     Option<usize>,
    pub current: usize
}

pub struct LimitOrderPool<T: LimitTx> {
    composable_orders: ComposableLimitPool<T>,
    limit_orders:      LimitPool<T>,
    /// used for easy update operations on Orders.
    all_order_ids:     HashMap<TransactionId, LimitOrderLocation>,
    /// used for nonce lookup.
    user_to_id:        HashMap<Address, TransactionId>,
    size:              SizeTracker
}

impl<T: LimitTx> LimitOrderPool<T> {
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            composable_orders: ComposableLimitPool::new(),
            limit_orders:      LimitPool::new(),
            all_order_ids:     HashMap::new(),
            user_to_id:        HashMap::new(),
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
