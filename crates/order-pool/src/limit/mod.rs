use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Debug
};

use guard_types::primitive::OrderType;
use reth_primitives::{alloy_primitives::Address, B256, U256};

use self::{composable::ComposableLimitPool, limit::LimitPool, side::Side};

mod composable;
mod limit;
mod parked;
mod pending;
mod side;

pub trait LimitTx: Side + Clone + Debug + Send + Sync + 'static {
    fn hash(&self) -> B256;
    fn get_pool(&self) -> Address;
    fn get_type(&self) -> OrderType;
    fn is_valid(&self) -> bool;
    fn is_expired(&self) -> bool;
    fn is_composable(&self) -> bool;
    fn get_id(&self) -> TransactionId;
    fn price(&self) -> OrderPrice;
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderPrice {
    price: U256
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TransactionId {
    pub user_addr:  Address,
    /// Pool id
    pub pool_id:    PoolId,
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
    #[error("Already have a ordered with {0:?}")]
    DuplicateNonce(TransactionId),
    #[error("Duplicate order")]
    DuplicateOrder
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
    /// TODO: this trait bound will change
    composable_orders:   ComposableLimitPool<T>,
    limit_orders:        LimitPool<T>,
    /// used for easy update operations on Orders.
    all_order_ids:       HashMap<TransactionId, LimitOrderLocation>,
    /// used for nonce lookup.
    user_to_id:          HashMap<Address, Vec<TransactionId>>,
    /// hash to pool location with identifier.
    order_hash_location: HashMap<B256, (TransactionId, LimitOrderLocation)>,
    /// The size of the current transactions.
    size:                SizeTracker
}

impl<T: LimitTx> LimitOrderPool<T> {
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            composable_orders:   ComposableLimitPool::new(),
            limit_orders:        LimitPool::new(),
            all_order_ids:       HashMap::new(),
            user_to_id:          HashMap::new(),
            order_hash_location: HashMap::new(),
            size:                SizeTracker { max: max_size, current: 0 }
        }
    }

    pub fn new_order(&mut self, order: T) -> Result<(), LimitPoolError> {
        let id = order.get_id();

        // is new order
        if self.all_order_ids.contains_key(&id) {
            return Err(LimitPoolError::DuplicateOrder)
        }

        // check for duplicate nonce
        if self
            .user_to_id
            .get(&id.user_addr)
            .map(|inner| inner.iter().any(|other_id| other_id.nonce == id.nonce))
            .unwrap_or(false)
        {
            return Err(LimitPoolError::DuplicateNonce(id))
        }

        // TODO: based on composability, insert into pools and then add to the tracking
        // list

        Ok(())
    }

    /// Removes all filled orders from the pools
    pub fn filled_orders(&mut self, orders: &Vec<B256>) -> Vec<T> {
        // remove from lower level + hash locations;
        orders
            .iter()
            .filter_map(|order_hash| {
                // remove from order hash loc
                let (id, location) = self.order_hash_location.remove(order_hash)?;
                match location {
                    LimitOrderLocation::Composable => self.composable_orders.remove_order(&id),
                    LimitOrderLocation::LimitPending => None,
                    _ => {
                        unreachable!()
                    }
                }
            })
            .map(|order| {
                let id = order.get_id();
                // remove from all orders
                let _ = self.all_order_ids.remove(&id);
                // remove from user;
                self.user_to_id
                    .get_mut(&id.user_addr)
                    .map(|inner| inner.retain(|order| order != &id));

                order
            })
            .collect()
    }

    /// Removes all orders for a given user when there state changes for
    /// re-validation
    pub fn changed_user_state(&mut self, users: &Vec<Address>) -> Vec<T> {
        users
            .iter()
            // remove user
            .filter_map(|user| self.user_to_id.remove(user))
            .flatten()
            .filter_map(|user_order| {
                // remove all orders
                let loc = self.all_order_ids.remove(&user_order)?;
                // remove hash
                let _ = self.order_hash_location.remove(&user_order.order_hash);
                match loc {
                    LimitOrderLocation::Composable => {
                        self.composable_orders.remove_order(&user_order)
                    }
                    LimitOrderLocation::LimitPending => todo!(),
                    LimitOrderLocation::LimitParked => todo!()
                }
            })
            .collect()
    }

    pub fn get_all_order(&mut self) -> (Vec<T>, Vec<T>) {
        todo!()
    }

    pub fn get_overlap_with_buffer(&mut self, tick_buffer: u8) -> (Vec<T>, Vec<T>) {
        todo!()
    }

    // TODO: add ability to fetch composable and non-composable orders
}
