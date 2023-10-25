use alloy_primitives::{Address, U256};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{QuotingApiServer, QuotingPubSubApiServer},
    types::{QuotingSubscriptionKind, QuotingSubscriptionParam}
};

pub struct QuotingApi<OrderPool> {
    order_pool: OrderPool
}

#[async_trait::async_trait]
impl<OrderPool> QuotingApiServer for QuotingApi<OrderPool>
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
}

#[async_trait::async_trait]
impl<OrderPool> QuotingPubSubApiServer for QuotingApi<OrderPool>
where
    OrderPool: Send + Sync + 'static
{
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: QuotingSubscriptionKind,
        params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
