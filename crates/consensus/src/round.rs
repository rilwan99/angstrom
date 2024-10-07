use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use alloy_primitives::{FixedBytes, B512};
use angstrom_types::consensus::{Commit, PreProposal};
use futures::{ready, Future, FutureExt, Stream};
use tokio::task::AbortHandle;
use tracing::warn;

use crate::ConsensusState;

#[allow(dead_code)]
pub enum Leader {
    ThisNode(B512),
    OtherNode(B512)
}

impl Leader {
    #[allow(dead_code)]
    pub fn address(&self) -> FixedBytes<64> {
        match self {
            Self::ThisNode(a) => *a,
            Self::OtherNode(a) => *a
        }
    }

    #[allow(dead_code)]
    pub fn is_me(&self) -> bool {
        matches!(self, Self::ThisNode(_))
    }
}

impl Default for Leader {
    fn default() -> Self {
        Self::ThisNode(FixedBytes::default())
    }
}

#[derive(Clone, Default)]
pub struct RoundStateTimings {
    pub accumulation: Duration,
    pub preproposal:  Duration,
    pub laggards:     Duration
}

impl RoundStateTimings {
    pub fn new(acc_secs: u64, pp_secs: u64, lag_secs: u64) -> Self {
        Self {
            accumulation: Duration::from_secs(acc_secs),
            preproposal:  Duration::from_secs(pp_secs),
            laggards:     Duration::from_secs(lag_secs)
        }
    }
}

/// The current state and subsequent actions that should be taken
/// for such state in a given round. All state that this contains
/// is transient to the given ethereum block height
pub struct RoundState {
    /// The current ethereum height
    height:         u64,
    /// Total number of nodes in the Angstrom network for this round
    #[allow(dead_code)]
    node_count:     u64,
    two_thirds:     usize,
    /// Who is the current leader
    #[allow(dead_code)]
    leader:         Leader,
    /// Round timing configuration
    timings:        RoundStateTimings,
    /// the current action we should be taking at the moment of
    /// time for this height
    current_state:  RoundAction,
    /// Broadcast preproposals seen for this round
    pre_proposals:  HashMap<FixedBytes<64>, PreProposal>,
    /// Proposal build task for this round - to be aborted if we start a new
    /// round
    proposal_build: Option<AbortHandle>,
    /// Broadcast commit messages seen for this round
    commits:        HashMap<FixedBytes<64>, Commit>
}

impl Drop for RoundState {
    fn drop(&mut self) {
        // Abort any outstanding proposal build if we have one
        if let Some(ref h) = self.proposal_build {
            h.abort();
        }
    }
}

impl RoundState {
    pub fn new(
        height: u64,
        node_count: u64,
        leader: Leader,
        timings: Option<RoundStateTimings>
    ) -> Self {
        // Quick and dirty way for us to find our threshold, can be cleaned up later
        let two_thirds: usize = ((2 * node_count) / 3) as usize;
        let timings = timings.unwrap_or_default();
        let current_state = RoundAction::new(&timings);
        Self {
            leader,
            timings,
            height,
            node_count,
            two_thirds,
            current_state,
            pre_proposals: HashMap::new(),
            proposal_build: None,
            commits: HashMap::new()
        }
    }

    pub fn current_height(&self) -> u64 {
        self.height
    }

    pub fn is_leader(&self) -> bool {
        matches!(self.leader, Leader::ThisNode(_))
    }

    pub fn timings(&self) -> RoundStateTimings {
        self.timings.clone()
    }

    pub fn on_commit(&mut self, peer: FixedBytes<64>, commit: Commit) -> Result<(), String> {
        match self.commits.entry(peer) {
            Entry::Occupied(_) => Err("Received a second commit from host".to_owned()),
            Entry::Vacant(v) => {
                v.insert(commit);
                Ok(())
            }
        }
    }

    /// Used when we start building a proposal to register the proposal build
    /// task and cancel any outstanding build task for this round (which
    /// should never happen, but hey...)
    pub fn on_proposal(&mut self, build_handle: AbortHandle) {
        if let Some(h) = self.proposal_build.replace(build_handle) {
            warn!("Second proposal build launched for eth height {}", self.height);
            h.abort()
        }
    }

    pub fn on_pre_propose(
        &mut self,
        peer: FixedBytes<64>,
        pre_propose: PreProposal
    ) -> Result<(), String> {
        // TODO:  Validate this prepropose
        if !matches!(
            self.current_state,
            RoundAction::PrePropose(_) | RoundAction::PreProposeLaggards(_)
        ) {
            return Err("PreProposal received early".to_owned())
        }
        match self.pre_proposals.entry(peer) {
            Entry::Occupied(_) => return Err("Received a second prepropose from host".to_owned()),
            Entry::Vacant(v) => {
                v.insert(pre_propose);
            }
        }
        // Check if we have enough preproposals to trigger our state change
        if self.pre_proposals.len() >= self.two_thirds {
            self.current_state =
                RoundAction::PreProposeLaggards(Timeout::new(self.timings.laggards));
        }
        Ok(())
    }

    pub fn get_preproposals(&self) -> Vec<PreProposal> {
        self.pre_proposals.values().cloned().collect()
    }
}

impl Stream for RoundState {
    type Item = ConsensusState;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<ConsensusState>> {
        let acc_timeout = self.timings.accumulation;
        let pre_timeout = self.timings.preproposal;

        let newstate =
            ready!(Pin::new(&mut self.current_state).poll_timeout(cx, acc_timeout, pre_timeout));

        let new_global_state = newstate.to_global();
        self.current_state = newstate;
        Poll::Ready(Some(new_global_state))

        // let is_leader = self.is_leader.clone();
        // if let Poll::Ready((new_action, msg)) = Pin::new(&mut
        // self.current_state)     .should_transition(cx,
        // GlobalStateContext { is_leader })     .map(|(round_action,
        // new_state, message)| {         self.consensus.
        // update_state(new_state);         (round_action, message)
        //     })
        // {
        //     self.current_state = new_action;
        //     return_if!(msg => { is_some() } map(Poll::Ready));
        // }

        // Poll::Pending
    }
}

#[repr(transparent)]
pub struct Timeout(Pin<Box<dyn Future<Output = ()> + Send>>);

impl Deref for Timeout {
    type Target = Pin<Box<dyn Future<Output = ()> + Send>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timeout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Timeout {
    pub fn new(timeout: Duration) -> Self {
        Timeout(Box::pin(async move { tokio::time::sleep(timeout).await }))
    }
}

impl std::fmt::Debug for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timeout")
    }
}

/// Enum describing the current state of our consensus round
#[derive(Debug)]
pub enum RoundAction {
    /// In OrderAccumulation state, orders for the current block are being
    /// collected.  The timeout represents the amount of time before we
    /// transition to PrePropose state.
    OrderAccumulation(Timeout),
    /// In PrePropose state, we assemble our accumulated orders and broadcast a
    /// PreProposal.  All nodes then collect PreProposal messages.  The timeout
    /// represents the amount of time that the leader node will wait to receive
    /// PreProposal messages from 2/3rds of the nodes in the network
    PrePropose(Timeout),
    /// Once the leader node has received PreProposals for 2/3rds of the nodes,
    /// we will wait a little bit to see if additional PrePropose messages make
    /// their way to us.  The timeout represents the amount of time the leader
    /// will wait to receive these extra messages.
    PreProposeLaggards(Timeout),
    /// Trigger proposal generation for the leader, otherwise go right back to
    /// OrderAccumulation (for the next block tho?)
    Propose
}

impl RoundAction {
    pub fn new(config: &RoundStateTimings) -> Self {
        Self::OrderAccumulation(Timeout::new(config.accumulation))
    }

    pub fn to_global(&self) -> ConsensusState {
        match self {
            Self::OrderAccumulation(_) => ConsensusState::OrderAccumulation,
            Self::PrePropose(_) => ConsensusState::PreProposal,
            Self::PreProposeLaggards(_) => ConsensusState::PreProposalLaggards,
            Self::Propose => ConsensusState::Proposal
        }
    }

    pub fn poll_timeout(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        acc_timeout: Duration,
        pre_timeout: Duration
    ) -> Poll<RoundAction> {
        match &mut *self {
            Self::OrderAccumulation(t) => t
                .poll_unpin(cx)
                .map(|_| Self::PrePropose(Timeout::new(pre_timeout))),
            Self::PrePropose(t) => t.poll_unpin(cx).map(|_| Self::Propose),
            Self::PreProposeLaggards(t) => t.poll_unpin(cx).map(|_| Self::Propose),
            Self::Propose => Poll::Ready(Self::OrderAccumulation(Timeout::new(acc_timeout)))
        }
    }
}

// impl StateTransition for RoundAction {
//     fn should_transition(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         global_state: GlobalStateContext
//     ) -> Poll<(RoundAction, ConsensusState, Option<RoundStateMessage>)> {
//         match &mut *self {
//             RoundAction::OrderAccumulation(p) =>
// Pin::new(p).should_transition(cx, global_state),
// RoundAction::PrePropose(p) => Pin::new(p).should_transition(cx,
// global_state),             RoundAction::Propose(p) =>
// Pin::new(p).should_transition(cx, global_state)
// RoundAction::Commit(p) => Pin::new(p).should_transition(cx, global_state),
//             RoundAction::Submit(p) => Pin::new(p).should_transition(cx,
// global_state),             RoundAction::Completed(p) =>
// Pin::new(p).should_transition(cx, global_state)         }
//     }
// }

#[cfg(test)]
mod tests {
    use alloy_primitives::FixedBytes;
    use testing_tools::type_generator::consensus::preproposal::PreproposalBuilder;
    use tokio_stream::StreamExt;

    use super::*;

    #[test]
    fn can_be_constructed() {
        RoundState::new(100, 100, super::Leader::ThisNode(FixedBytes::random()), None);
    }

    #[tokio::test]
    async fn states_progress_properly() {
        let mut state =
            RoundState::new(100, 100, super::Leader::ThisNode(FixedBytes::random()), None);
        assert!(
            matches!(state.current_state, RoundAction::OrderAccumulation(_)),
            "Didn't start in OrderAccumulation state"
        );

        // Step to PreProposal state
        let new_state = state.next().await;
        assert!(
            matches!(new_state, Some(ConsensusState::PreProposal)),
            "Didn't emit global PreProposal state"
        );
        assert!(
            matches!(state.current_state, RoundAction::PrePropose(_)),
            "Didn't set to internal PreProposal state"
        );

        // Step to Proposal state - skip PreProposalLaggards
        let new_state = state.next().await;
        assert!(
            matches!(new_state, Some(ConsensusState::Proposal)),
            "Didn't emit global Proposal state"
        );
        assert!(
            matches!(state.current_state, RoundAction::Propose),
            "Didn't set to internal Proposal state"
        );

        // Back to OrderAccumulation
        let new_state = state.next().await;
        assert!(
            matches!(new_state, Some(ConsensusState::OrderAccumulation)),
            "Didn't emit global OrderAccumulation state"
        );
        assert!(
            matches!(state.current_state, RoundAction::OrderAccumulation(_)),
            "Didn't set to internal OrderAccumulation state"
        );
    }

    #[test]
    fn hits_preproposal_laggard_state() {
        // Make our round state
        let mut state =
            RoundState::new(100, 3, super::Leader::ThisNode(FixedBytes::random()), None);
        // Set it to PrePropose state
        state.current_state = RoundAction::PrePropose(Timeout::new(Duration::default()));
        // One preproposal is not enough
        let pp1 = PreproposalBuilder::new()
            .order_count(20)
            .for_random_pools(1)
            .for_block(100)
            .build();
        state.on_pre_propose(FixedBytes::random(), pp1).unwrap();
        assert!(
            matches!(state.current_state, RoundAction::PrePropose(_)),
            "Not still in PrePropose state"
        );
        // But then two should cause us to have 2/3rds of the network so we should be
        // good
        let pp2 = PreproposalBuilder::new()
            .order_count(20)
            .for_random_pools(1)
            .for_block(100)
            .build();
        state.on_pre_propose(FixedBytes::random(), pp2).unwrap();
        assert!(
            matches!(state.current_state, RoundAction::PreProposeLaggards(_)),
            "Didn't jump to Laggards state"
        );
    }
}
