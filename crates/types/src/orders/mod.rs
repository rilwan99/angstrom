mod limit;
mod order_id;
mod order_metadata;
mod origin;
mod searcher;
use std::fmt;

use alloy_primitives::{Address, Bytes, TxHash, U128, U256};
pub use limit::*;
pub use order_id::*;
pub use order_metadata::*;
pub use origin::*;
pub use searcher::*;

pub trait PooledOrder: fmt::Debug + Send + Sync + Clone + Unpin + 'static {
    type ValidationData: Send + Sync + Clone + Unpin + 'static;

    /// Hash of the order
    fn hash(&self) -> TxHash;

    fn order_id(&self) -> OrderId;

    /// The order signer
    fn from(&self) -> Address;

    /// Transaction nonce
    fn nonce(&self) -> U256;

    /// Amount of tokens to sell
    fn amount_in(&self) -> u128;

    /// Min amount of tokens to buy
    fn amount_out_min(&self) -> u128;

    /// Limit Price
    fn limit_price(&self) -> u128;

    fn order_priority_data(&self) -> OrderPriorityData;

    /// Order deadline
    fn deadline(&self) -> U256;

    /// Returns a measurement of the heap usage of this type and all its
    /// internals.
    fn size(&self) -> usize;

    /// Returns the length of the rlp encoded transaction object
    ///
    /// Note: Implementations should cache this value.
    fn encoded_length(&self) -> usize;

    /// Returns chain_id
    fn chain_id(&self) -> Option<u64>;

    /// Returns if the order should be pending or parked
    fn is_valid(&self) -> bool;

    /// Returns the direction of the pool defined by ordering
    fn is_bid(&self) -> bool;
}

pub trait PooledComposableOrder: PooledOrder {
    fn pre_hook(&self) -> Option<Bytes>;

    fn post_hook(&self) -> Option<Bytes>;
}
