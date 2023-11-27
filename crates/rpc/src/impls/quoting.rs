use alloy_primitives::{Address, U256};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::QuotingApiServer,
    types::{QuotingSubscriptionKind, QuotingSubscriptionParam}
};

pub struct QuotesApi<OrderPool> {
    pub pool: OrderPool
}

#[async_trait::async_trait]
impl<OrderPool> QuotingApiServer for QuotesApi<OrderPool>
where
    OrderPool: Send + Sync + 'static
{
    async fn quote_transaction(
        &self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        amount_out: U256
    ) -> RpcResult<U256> {
        todo!()
    }

    async fn subscribe_quotes(
        &self,
        pending: PendingSubscriptionSink,
        kind: QuotingSubscriptionKind,
        params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
