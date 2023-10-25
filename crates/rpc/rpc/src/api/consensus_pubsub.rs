use jsonrpsee::proc_macros::rpc;

use crate::types::subscriptions::ConsensusSubscriptionKind;

#[rpc(server, namespace = "consensus")]
#[async_trait::async_trait]
pub trait ConsensusPubSubApi {
    #[subscription(
        name = "subscribe" => "subscription",
        unsubscribe = "unsubscribe",
        item = crate::types::subscription::ConsensusSubscriptionResult
    )]
    async fn subscribe(
        &self,
        kind: ConsensusSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult;
}
