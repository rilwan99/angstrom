use alloy_primitives::{Address, U256};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{QuotingApiServer, QuotingPubSubApiServer},
    types::{QuotingSubscriptionKind, QuotingSubscriptionParam}
};

pub struct QuotingApi {}

#[async_trait::async_trait]
impl QuotingApiServer for QuotingApi {
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
impl QuotingPubSubApiServer for QuotingApi {
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: QuotingSubscriptionKind,
        params: Option<QuotingSubscriptionParam>
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
