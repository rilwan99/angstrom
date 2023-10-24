use jsonrpsee::proc_macros::rpc;

use crate::types::subscriptions::{QuotingSubscriptionKind, QuotingSubscriptionParam};

#[rpc(server, namespace = "quoting")]
#[async_trait::async_trait]
pub trait QuotingPubSubApi {
    #[subscription(
        name = "subscribe" => "subscription",
        unsubscribe = "unsubscribe",
        item = crate::types::subscription::QuotingSubscriptionResult
    )]
    async fn subscribe(
        &self,
        kind: QuotingSubscriptionKind,
        params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult;
}
