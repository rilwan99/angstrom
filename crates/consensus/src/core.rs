use std::{
    borrow::Cow,
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll, Waker}
};

use futures::{stream::FuturesUnordered, Stream, StreamExt};
use guard_types::{
    consensus::{
        Block, BlockId, BundleVote, EvidenceError, GuardSet, LeaderProposal, SignedLeaderProposal,
        Valid23Bundle
    },
    on_chain::SimmedBundle
};
use sim::Simulator;
use thiserror::Error;
use tokio::task::{JoinError, JoinHandle, JoinSet};

use crate::{
    bundle::{BundleVoteManager, BundleVoteMessage},
    evidence::EvidenceCollector,
    executor::{Executor, ExecutorMessage},
    leader::ProposalManager,
    stage::Stage
};

#[derive(Debug)]
pub enum ConsensusMessage {
    // voting related activities
    NewBundle23(Valid23Bundle),
    NewBundleVote(BundleVote),
    // finalization actions
    Proposal(LeaderProposal),
    SignedProposal(SignedLeaderProposal),
    // db related
    NewBlock(Block),
    NewBundle(SimmedBundle)
}

#[derive(Debug, Error)]
pub enum ConsensusError {
    #[error("Evidence Module had an Error: {0:#?}")]
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
    guards:             GuardSet,
    proposal_manager:   ProposalManager,
    executor:           Executor<S>,
    outbound:           VecDeque<ConsensusMessage>
}

impl<S: Simulator + 'static> ConsensusCore<S> {
    pub async fn new() -> Self {
        todo!()
    }

    pub fn new_proposal(&mut self, proposal: LeaderProposal) {
        // verify proposal
        if let Ok(proposal) = self.executor.sign_leader_proposal(proposal) {
        } else {
            error!("failed to sign the leader proposal");
        }
    }

    pub fn new_bundle(&mut self, bundle: SimmedBundle) {
        if let Some(hash) = self.bundle_data.new_simmed_bundle(bundle) {
            // new bundle, lets sign and propagate our hash
            let Ok(signed_bundle) =
                self.executor
                    .sign_bundle_vote(hash, self.stage.height, self.stage.round)
            else {
                return
            };

            self.outbound.push_back(signed_bundle.clone());

            // add vote to underlying and if we hit 2/3 we fully propagate
            if let Some(msg) = self
                .bundle_data
                .new_bundle_vote(signed_bundle, &self.guards)
            {
                self.outbound.push_back(ConsensusMessage::NewBundle23(msg));
            }
        }
    }

    pub fn new_bundle_vote(&mut self, vote: BundleVote) {
        if !self.bundle_data.contains_vote(&vote) {
            if let Some(valid23) = self.bundle_data.new_bundle_vote(vote.clone(), &self.guards) {
                self.outbound
                    .push_back(ConsensusMessage::NewBundle23(valid23));
            }
            self.outbound
                .push_back(ConsensusMessage::NewBundleVote(vote));
        }
    }

    pub fn new_bundle_23(&mut self, bundle: Valid23Bundle) {
        let mut bundle = Cow::from(bundle);

        if self.bundle_data.new_bundle23(&bundle, &self.guards) {
            self.outbound
                .push_back(ConsensusMessage::NewBundle23(bundle.into_owned()))
        }
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
