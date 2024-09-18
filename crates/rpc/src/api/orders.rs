use angstrom_types::sol_bindings::rpc_orders::{
    ExactFlashOrder, ExactStandingOrder, PartialFlashOrder, PartialStandingOrder, TopOfBlockOrder
};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::types::OrderSubscriptionKind;

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "angstrom"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "angstrom"))]
#[async_trait::async_trait]
pub trait OrderApi {
    /// Users send the rlp encoded signature and order bytes
    #[method(name = "sendPartialStandingOrder")]
    async fn send_partial_standing_order(&self, order: PartialStandingOrder) -> RpcResult<bool>;

    #[method(name = "sendExactStandingOrder")]
    async fn send_exact_standing_order(&self, order: ExactStandingOrder) -> RpcResult<bool>;

    #[method(name = "sendSearcherOrder")]
    async fn send_searcher_order(&self, order: TopOfBlockOrder) -> RpcResult<bool>;

    #[method(name = "sendPartialFlashOrder")]
    async fn send_partial_flash_order(&self, order: PartialFlashOrder) -> RpcResult<bool>;

    #[method(name = "sendExactFlashOrder")]
    async fn send_exact_flash_order(&self, order: ExactFlashOrder) -> RpcResult<bool>;

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
