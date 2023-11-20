use std::fmt::Debug;

use alloy_primitives::{Address, Bytes, TxHash, U256};

use super::{
    super::{OrderId, OrderOrigin, PooledComposableOrder, PooledOrder},
    ValidatedOrder
};
use crate::{
    primitive::{ComposableOrder, Order, PoolId},
    rpc::{
        EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
        EcRecoveredSearcherOrder, SignedComposableLimitOrder
    }
};

pub trait PooledLimitOrder: PooledOrder {
    /// The liquidity pool this order trades in
    fn pool_and_direction(&self) -> (PoolId, bool);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderPriorityData {
    pub price:  u128,
    pub volume: u128,
    pub gas:    u128
}

impl PartialOrd for OrderPriorityData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.price.cmp(&other.price).then_with(|| {
            self.volume
                .cmp(&other.volume)
                .then_with(|| self.gas.cmp(&other.gas))
        }))
    }
}

impl Ord for OrderPriorityData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<O> ValidatedOrder<O, OrderPriorityData>
where
    O: PooledOrder
{
    pub fn pool_id(&self) -> usize {
        self.pool_id
    }

    pub fn is_bid(&self) -> bool {
        self.is_bid
    }

    pub fn priority_data(&self) -> OrderPriorityData {
        self.data.clone()
    }
}

impl PooledOrder for EcRecoveredLimitOrder {
    type ValidationData = ValidatedOrder<Self, OrderPriorityData>;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn is_bid(&self) -> bool {
        todo!()
    }

    fn hash(&self) -> TxHash {
        self.signed_order.hash
    }

    fn from(&self) -> Address {
        self.signer
    }

    fn nonce(&self) -> U256 {
        self.order.nonce
    }

    fn amount_in(&self) -> u128 {
        self.signed_order.order.amountIn
    }

    fn amount_out_min(&self) -> u128 {
        self.signed_order.order.amountOutMin
    }

    fn limit_price(&self) -> u128 {
        self.amount_out_min() / self.amount_in()
    }

    fn deadline(&self) -> U256 {
        self.signed_order.order.deadline
    }

    fn size(&self) -> usize {
        unreachable!()
    }

    fn encoded_length(&self) -> usize {
        unreachable!()
    }

    fn chain_id(&self) -> Option<u64> {
        unreachable!()
    }

    fn order_id(&self) -> OrderId {
        todo!()
    }
}

impl PooledLimitOrder for EcRecoveredLimitOrder {
    fn pool_and_direction(&self) -> (PoolId, bool) {
        //(self.signed_order.order.pool, self.signed_order.order.direction)
        todo!()
    }
}

impl PooledOrder for EcRecoveredComposableLimitOrder {
    type ValidationData = ValidatedOrder<Self, OrderPriorityData>;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn is_bid(&self) -> bool {
        todo!()
    }

    fn hash(&self) -> TxHash {
        self.signed_order.hash
    }

    fn from(&self) -> Address {
        self.signer
    }

    fn nonce(&self) -> U256 {
        self.order.nonce
    }

    fn amount_in(&self) -> u128 {
        self.signed_order.order.amountIn
    }

    fn amount_out_min(&self) -> u128 {
        self.signed_order.order.amountOutMin
    }

    fn limit_price(&self) -> u128 {
        self.amount_out_min() / self.amount_in()
    }

    fn deadline(&self) -> U256 {
        self.signed_order.order.deadline
    }

    fn size(&self) -> usize {
        unreachable!()
    }

    fn encoded_length(&self) -> usize {
        unreachable!()
    }

    fn chain_id(&self) -> Option<u64> {
        unreachable!()
    }

    fn order_id(&self) -> OrderId {
        todo!()
    }
}

impl PooledLimitOrder for EcRecoveredComposableLimitOrder {
    fn pool_and_direction(&self) -> (usize, bool) {
        //(self.signed_order.order.pool, self.signed_order.order.direction)
        todo!()
    }
}

impl PooledComposableOrder for EcRecoveredComposableLimitOrder {
    fn pre_hook(&self) -> Option<Bytes> {
        todo!()
    }

    fn post_hook(&self) -> Option<Bytes> {
        todo!()
    }
}
