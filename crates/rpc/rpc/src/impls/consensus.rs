use common::ConsensusState;
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{ConsensusApiServer, ConsensusPubSubApiServer},
    types::ConsensusSubscriptionKind
};

pub struct ConsensusApi<C> {
    consensus: C
}

#[async_trait::async_trait]
impl<C> ConsensusApiServer for ConsensusApi<C>
where
    C: Send + Sync + 'static
{
    async fn consensus_state(&self) -> RpcResult<ConsensusState> {
        todo!()
    }
}

#[async_trait::async_trait]
impl<C> ConsensusPubSubApiServer for ConsensusApi<C>
where
    C: Send + Sync + 'static
{
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: ConsensusSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
