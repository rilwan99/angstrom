use jsonrpsee::proc_macros::rpc;

use crate::types::subscriptions::OrderSubscriptionKind;

#[rpc(server, namespace = "order")]
#[async_trait::async_trait]
pub trait OrderPubSubApi {
    #[subscription(
        name = "subscribe" => "subscription",
        unsubscribe = "unsubscribe",
        item = crate::types::subscription::OrderSubscriptionResult
    )]
    async fn subscribe(&self, kind: OrderSubscriptionKind) -> jsonrpsee::core::SubscriptionResult;
}
