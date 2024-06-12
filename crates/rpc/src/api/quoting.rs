use alloy_primitives::{Address, U256};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::types::subscriptions::{QuotingSubscriptionKind, QuotingSubscriptionParam};

#[cfg_attr(not(feature = "client"), rpc(server, namespace = "quoting"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "quoting"))]
#[async_trait::async_trait]
pub trait QuotingApi {
    #[method(name = "get_quote")]
    async fn quote_transaction(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        amount_out: U256
    ) -> RpcResult<U256>;

    #[subscription(
        name = "subscribe_BBO", 
        unsubscribe = "unsubscribe_quotes",
        item = crate::types::subscriptions::QuotingSubscriptionResult
    )]
    async fn subscribe_quotes(
        &self,
        kind: QuotingSubscriptionKind,
        params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult;
}
