use std::{
    pin::Pin,
    task::{Context, Poll}
};

use futures::Stream;
use guard_types::consensus::{Block, BlockId, EvidenceError, Proposal};
use sim::Simulator;
use thiserror::Error;

use crate::{
    bundle::{BundleVoteManager, ValidBundle},
    evidence::EvidenceCollector,
    executor::Executor,
    stage::Stage
};

#[derive(Debug)]
pub enum ConsensusMessage {}

#[derive(Debug, Error)]
pub enum ConsensusError {
    #[error("Evidence Module had an Error: {0#?}")]
    EvidenceError(#[from] EvidenceError)
}

/// The ConsensusCore module handles everything related to consensus.
/// This includes but not limited to.
///
/// 1) Collecting Evidence against misbehavior
/// 2) Verifying Block and bundle data
/// 3) Leader Selection
/// 4) Verifying Historical State
/// 5) Signing Votes, Commitments & Proposals
pub struct ConsensusCore<S: Simulator + 'static> {
    evidence_collector: EvidenceCollector,
    stage:              Stage,
    bundle_data:        BundleVoteManager,
    executor:           Executor<S>
}

impl<S: Simulator + 'static> ConsensusCore<S> {
    pub async fn new() -> Self {
        todo!()
    }

    fn on_executor(
        &mut self,
        executor_msg: Option<ExecutorMessage>
    ) -> Option<Result<ConsensusMessage, ConsensusError>> {
        let message = executor_msg?;
        None
    }
}

impl<S: Simulator + 'static> Stream for ConsensusCore<S> {
    type Item = Result<ConsensusMessage, ConsensusError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stuff = self.executor.poll(cx);

        todo!()
    }
}
