use alloy_primitives::{Address, Bytes, TxHash, U256};

use super::{OrderId, OrderPriorityData, PooledComposableOrder, PooledOrder};
use crate::{
    primitive::{ComposableOrder, Order, PoolKey},
    rpc::{EcRecoveredComposableLimitOrder, EcRecoveredLimitOrder}
};

pub trait PooledLimitOrder: PooledOrder {
    /// The liquidity pool this order trades in
    fn pool_and_direction(&self) -> (u8, bool);
}

pub trait LimitOrderValidation {
    fn priority_data(&self) -> OrderPriorityData;
}

impl LimitOrderValidation for OrderPriorityData {
    fn priority_data(&self) -> OrderPriorityData {
        self.clone()
    }
}

pub trait ComposableLimitOrderValidation {
    fn data(&self) -> u8;
}

impl PooledOrder for EcRecoveredLimitOrder {
    type ValidationData = OrderPriorityData;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn order_priority_data(&self) -> OrderPriorityData {
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
    fn pool_and_direction(&self) -> (u8, bool) {
        //(self.signed_order.order.pool, self.signed_order.order.direction)
        todo!()
    }
}

impl PooledOrder for EcRecoveredComposableLimitOrder {
    type ValidationData = ();

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn order_priority_data(&self) -> OrderPriorityData {
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
    fn pool_and_direction(&self) -> (u8, bool) {
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
