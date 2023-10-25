use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{OrderApiServer, OrderPubSubApiServer},
    types::OrderSubscriptionKind
};

pub struct OrderApi<OrderPool> {
    order: OrderPool
}

#[async_trait::async_trait]
impl<OrderPool> OrderApiServer for OrderApi<OrderPool>
where
    OrderPool: Send + Sync + 'static
{
    async fn submit_order(&self) -> RpcResult<bool> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<OrderPool> OrderPubSubApiServer for OrderApi<OrderPool>
where
    OrderPool: Send + Sync + 'static
{
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: OrderSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
