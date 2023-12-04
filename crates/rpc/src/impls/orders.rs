use alloy_primitives::Bytes;
use guard_types::rpc::{
    EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
    EcRecoveredSearcherOrder
};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{api::OrderApiServer, types::OrderSubscriptionKind};

pub struct OrderApi<OrderPool> {
    pub pool: OrderPool
}

#[async_trait::async_trait]
impl<OrderPool> OrderApiServer for OrderApi<OrderPool>
where
    OrderPool: Send + Sync + 'static
{
    async fn submit_limit_order(&self, order: Bytes) -> RpcResult<bool> {
        if let Ok(order) = order.try_into() {
            let _order: EcRecoveredLimitOrder = order;
            todo!()
        } else {
            Ok(false)
        }
    }

    async fn submit_searcher_order(&self, order: Bytes) -> RpcResult<bool> {
        if let Ok(order) = order.try_into() {
            let _order: EcRecoveredSearcherOrder = order;
            todo!()
        } else {
            Ok(false)
        }
    }

    async fn submit_composable_limit_order(&self, order: Bytes) -> RpcResult<bool> {
        if let Ok(order) = order.try_into() {
            let _order: EcRecoveredComposableLimitOrder = order;
            todo!()
        } else {
            Ok(false)
        }
    }

    async fn submit_composable_searcher_order(&self, order: Bytes) -> RpcResult<bool> {
        if let Ok(order) = order.try_into() {
            let _order: EcRecoveredComposableSearcherOrder = order;
            todo!()
        } else {
            Ok(false)
        }
    }

    async fn subscribe_orders(
        &self,
        _pending: PendingSubscriptionSink,
        _kind: OrderSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
