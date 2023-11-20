use alloy_primitives::{Address, Bytes, TxHash, U256};

use super::{OrderId, OrderOrigin, OrderPriorityData, PooledComposableOrder, PooledOrder};
use crate::{
    primitive::{ComposableOrder, Order, PoolKey},
    rpc::{EcRecoveredComposableSearcherOrder, EcRecoveredSearcherOrder}
};

pub trait PooledSearcherOrder: PooledOrder {
    /// The liquidity pool this order trades in
    fn pool(&self) -> u8;
    /// donate value
    fn donate(&self) -> (u128, u128);

    fn volume(&self) -> u128;

    fn gas(&self) -> u128;

    fn donated(&self) -> u128;
}

pub trait SearcherOrderValidation {
    fn data(&self) -> u8;
}

pub trait ComposableSearcherOrderValidation {
    fn data(&self) -> u8;
}

impl PooledOrder for EcRecoveredSearcherOrder {
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

impl PooledSearcherOrder for EcRecoveredSearcherOrder {
    fn gas(&self) -> u128 {
        todo!()
    }

    fn pool(&self) -> u8 {
        todo!()
    }

    fn donate(&self) -> (u128, u128) {
        todo!()
    }

    fn volume(&self) -> u128 {
        todo!()
    }

    fn donated(&self) -> u128 {
        todo!()
    }
}

impl PooledOrder for EcRecoveredComposableSearcherOrder {
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

impl PooledSearcherOrder for EcRecoveredComposableSearcherOrder {
    fn gas(&self) -> u128 {
        todo!()
    }

    fn pool(&self) -> u8 {
        todo!()
    }

    fn donate(&self) -> (u128, u128) {
        todo!()
    }

    fn volume(&self) -> u128 {
        todo!()
    }

    fn donated(&self) -> u128 {
        todo!()
    }
}

impl PooledComposableOrder for EcRecoveredComposableSearcherOrder {
    fn pre_hook(&self) -> Option<Bytes> {
        todo!()
    }

    fn post_hook(&self) -> Option<Bytes> {
        todo!()
    }
}
