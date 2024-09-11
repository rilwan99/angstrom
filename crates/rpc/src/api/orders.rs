use angstrom_types::rpc::{
    FlashOrderRequest, SignedOrder, StandingOrderRequest, TopOfBlockOrderRequest
};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::types::OrderSubscriptionKind;

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "angstrom"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "angstrom"))]
#[async_trait::async_trait]
pub trait OrderApi {
    /// Users send the rlp encoded signature and order bytes
    #[method(name = "sendStandingOrder")]
    async fn send_standing_order(
        &self,
        order: SignedOrder<StandingOrderRequest>
    ) -> RpcResult<bool>;

    #[method(name = "sendSearcherOrder")]
    async fn send_searcher_order(
        &self,
        order: SignedOrder<TopOfBlockOrderRequest>
    ) -> RpcResult<bool>;

    #[method(name = "sendFlashOrder")]
    async fn send_flash_order(&self, order: SignedOrder<FlashOrderRequest>) -> RpcResult<bool>;

    #[subscription(
        name = "subscribeOrders",
        unsubscribe = "unsubscribeOrders",
        item = crate::types::subscriptions::OrderSubscriptionResult
    )]
    async fn subscribe_orders(
        &self,
        kind: OrderSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult;
}
