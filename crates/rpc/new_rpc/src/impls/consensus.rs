use common::ConsensusState;
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink};

use crate::{
    api::{ConsensusApiServer, ConsensusPubSubApiServer},
    types::ConsensusSubscriptionKind
};

pub struct ConsensusApi {}

#[async_trait::async_trait]
impl ConsensusApiServer for ConsensusApi {
    async fn consensus_state(&self) -> RpcResult<ConsensusState> {
        todo!()
    }
}

#[async_trait::async_trait]
impl ConsensusPubSubApiServer for ConsensusApi {
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: ConsensusSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        todo!()
    }
}
