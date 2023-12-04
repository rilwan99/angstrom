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
        _token_in: Address,
        _token_out: Address,
        _amount_in: U256,
        _amount_out: U256
    ) -> RpcResult<U256> {
        todo!()
    }

    async fn subscribe_quotes(
        &self,
        _pending: PendingSubscriptionSink,
        _kind: QuotingSubscriptionKind,
        _params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
