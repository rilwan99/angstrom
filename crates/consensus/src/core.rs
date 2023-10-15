use std::{
    borrow::Cow,
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll, Waker}
};

use futures::{stream::FuturesUnordered, Stream, StreamExt};
// use guard_provider::ProviderFactory;
use guard_types::{
    consensus::{
        Block, BundleVote, EvidenceError, GuardInfo, GuardSet, LeaderProposal,
        SignedLeaderProposal, Valid23Bundle
    },
    database::{BlockId, State},
    on_chain::SimmedBundle
};
use reth_db::{mdbx::Env, DatabaseEnv};
use sim::Simulator;
use thiserror::Error;
use tokio::task::{JoinError, JoinHandle, JoinSet};
use tracing::{error, warn};

use crate::{
    evidence::EvidenceCollector,
    executor::{Executor, ExecutorMessage},
    guard_stages::GuardStages,
    round::RoundState,
    state::ChainMaintainer
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
/// This includes tracking slashable events, other guards commits and votes
/// and submitting to consensus.
/// # Design Principles
/// The main interfacing idea for the ConsensusCore is that this module
/// only operates on truths. What this means is this module avoids doing
/// any comparison, building or evaluation in order to keep it as simple as
/// possible (Of course we cannot rid all of this, however there is always a
/// focus to minimize this). all values that are handed to this module are true.
/// for example, this means that the consensus module doesn't know of any other
/// bundles that this guard has built except for the most profitable one. Nor
/// does it know what the proper pricing for a given storage slot is. We
/// abstract all of this out in order to keep this module as clean as possible
/// as proper functionality is critical here to ensure that Angstrom works
/// properly.
pub struct ConsensusCore<S: Simulator + 'static> {
    /// collects + formulates evidence of byzantine guards
    evidence_collector: EvidenceCollector,
    /// keeps track of the current round state
    round_state:        RoundState,
    /// the current overlook of the network stage
    state:              State,
    /// keeps track of what stage of consensus all guards are on
    guard_stages:       GuardStages,
    /// used to execute underlying state.
    /// TODO: can prob remove this.
    executor:           Executor<S>,
    /// u8 is a placeholder till we unblackbox db
    /// Deals with our local chain state
    chain_maintainer:   ChainMaintainer<u8>,
    /// messages to share with others
    outbound:           VecDeque<ConsensusMessage>
}

impl<S: Simulator + 'static> ConsensusCore<S> {
    pub async fn new() -> Self {
        todo!()
    }

    // TODO: this should be when they officially join
    pub fn new_guard(&mut self, guard: GuardInfo) {
        self.state.next_guards.new_guard(guard);
    }

    pub fn new_block(&mut self, block: Block) {
        if self.state.verify_block(&block) {
            todo!()
        }
    }

    pub fn new_proposal_vote(&mut self, vote: SignedLeaderProposal) {
        if !self.round_state.stage().is_past_proposal_vote_cutoff()
            && self
                .round_state
                .proposal_manager()
                .new_proposal_vote(vote.clone(), &self.state.guards)
        {
            self.outbound
                .push_back(ConsensusMessage::SignedProposal(vote))
        }
    }

    pub fn new_proposal(&mut self, proposal: LeaderProposal) {
        if self.round_state.stage().is_past_proposal_cutoff() {
            warn!(?proposal, "received proposal to late");
            return
        }

        if self.round_state.proposal_manager().has_proposal() {
            return
        }

        let Some(current_leader) = self.state.guards.get_current_leader() else { return };

        // validate signatures on proposal
        if !proposal.validate_signature(current_leader.pub_key) {
            warn!(?proposal, "failed to validate signatures");
            return
        }

        let Ok(proposal_vote) = self.executor.sign_leader_proposal(&proposal) else {
            error!("failed to sign the leader proposal");
            return
        };

        // verify the bundle is the best
        if !self.round_state.bundle().is_best_bundle(&proposal.bundle) {
            error!(?proposal, "the proposed bundle doesn't match our best bundle");
        }

        self.round_state
            .proposal_manager()
            .new_proposal(proposal.clone());
        self.round_state
            .proposal_manager()
            .new_proposal_vote(proposal_vote.clone(), &self.state.guards);

        self.outbound.extend(
            vec![
                ConsensusMessage::Proposal(proposal),
                ConsensusMessage::SignedProposal(proposal_vote),
            ]
            .into_iter()
        );
    }

    pub fn new_bundle(&mut self, bundle: SimmedBundle) {
        if self.round_state.stage().is_past_bundle_signing_cutoff() {
            return
        }

        if let Some(hash) = self.round_state.bundle().new_simmed_bundle(bundle) {
            // new bundle, lets sign and propagate our hash
            let Ok(signed_bundle) = self.executor.sign_bundle_vote(
                hash,
                self.round_state.stage().height(),
                self.round_state.stage().round()
            ) else {
                return
            };

            self.outbound
                .push_back(ConsensusMessage::NewBundleVote(signed_bundle.clone()));

            // add vote to underlying and if we hit 2/3 we fully propagate
            if let Some(msg) = self
                .round_state
                .bundle()
                .new_bundle_vote(signed_bundle, &self.state.guards)
            {
                self.outbound.push_back(ConsensusMessage::NewBundle23(msg));
            }
        }
    }

    pub fn new_bundle_vote(&mut self, vote: BundleVote) {
        if self.round_state.stage().is_past_bundle_signing_cutoff() {
            return
        }

        if !self.round_state.bundle().contains_vote(&vote) {
            if let Some(valid23) = self
                .round_state
                .bundle()
                .new_bundle_vote(vote.clone(), &self.state.guards)
            {
                self.outbound
                    .push_back(ConsensusMessage::NewBundle23(valid23));
            }
            self.outbound
                .push_back(ConsensusMessage::NewBundleVote(vote));
        }
    }

    pub fn new_bundle_23(&mut self, bundle: Valid23Bundle) {
        if self.round_state.stage().is_past_bundle23_prop_cutoff() {
            return
        }

        if self
            .round_state
            .bundle()
            .new_bundle23(bundle.clone(), &self.state.guards)
        {
            self.outbound
                .push_back(ConsensusMessage::NewBundle23(bundle.clone()))
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
        if let Some(new_round_step) = self.round_state.stage().update_current_stage() {
            todo!()
        }

        let stuff = self.executor.poll(cx);

        todo!()
    }
}
