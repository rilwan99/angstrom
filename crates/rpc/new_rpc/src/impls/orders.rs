use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{OrderApiServer, OrderPubSubApiServer},
    types::OrderSubscriptionKind
};

pub struct OrderApi {}

#[async_trait::async_trait]
impl OrderApiServer for OrderApi {
    async fn submit_order(&self) -> RpcResult<bool> {
        todo!()
    }
}

#[async_trait::async_trait]
impl OrderPubSubApiServer for OrderApi {
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: OrderSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
