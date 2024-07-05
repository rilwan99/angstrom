use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use alloy_primitives::FixedBytes;
use angstrom_types::consensus::{Commit, PreProposal, Proposal};
use futures::{ready, Future, FutureExt, Stream};
use reth_primitives::B512;

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

/// The current state and subsequent actions that should be taken
/// for such state in a given round. All state that this contains
/// is transient to the given ethereum block height
pub struct RoundState {
    /// The current ethereum height
    height:        u64,
    /// Total number of nodes in the Angstrom network for this round
    #[allow(dead_code)]
    node_count:    u64,
    two_thirds:    usize,
    /// Who is the current leader
    #[allow(dead_code)]
    leader:        Leader,
    /// the current action we should be taking at the moment of
    /// time for this height
    current_state: RoundAction,
    /// Broadcast preproposals seen for this round
    pre_proposals: HashMap<FixedBytes<64>, PreProposal>,
    /// Proposal seen for this round
    proposal:      Option<Proposal>,
    /// Broadcast commit messages seen for this round
    commits:       HashMap<FixedBytes<64>, Commit>
}

impl RoundState {
    pub fn new(height: u64, node_count: u64, leader: Leader) -> Self {
        // Quick and dirty way for us to find our threshold, can be cleaned up later
        let two_thirds: usize = ((2 * node_count) / 3) as usize;
        Self {
            leader,
            height,
            node_count,
            two_thirds,
            current_state: RoundAction::new(),
            pre_proposals: HashMap::new(),
            proposal: None,
            commits: HashMap::new()
        }
    }

    pub fn current_height(&self) -> u64 {
        self.height
    }

    pub fn is_leader(&self) -> bool {
        matches!(self.leader, Leader::ThisNode(_))
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

    pub fn on_proposal(&mut self, proposal: Proposal) -> Result<(), String> {
        if self.proposal.is_some() {
            return Err("Received second proposal".to_owned())
        }
        self.proposal = Some(proposal);
        Ok(())
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
                RoundAction::PreProposeLaggards(Timeout::new(Duration::from_secs(2)));
        }
        Ok(())
    }
}

impl Stream for RoundState {
    type Item = ConsensusState;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<ConsensusState>> {
        let newstate = ready!(Pin::new(&mut self.current_state).poll_timeout(
            cx,
            Duration::from_secs(6),
            Duration::from_secs(6)
        ));

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

/// Representation of a finite-state machine
pub enum RoundAction {
    /// The precommit state actions we
    OrderAccumulation(Timeout),
    /// Time window during which we expect to be receiving Prepropose messages
    PrePropose(Timeout),
    /// We have received 2/3rds of all Prepropose messages and are waiting for
    /// any extra messages that might arrive
    PreProposeLaggards(Timeout),
    /// Trigger proposal generation for the leader, otherwise go right back to
    /// OrderAccumulation (for the next block tho?)
    Propose
}

impl RoundAction {
    pub fn new() -> Self {
        Self::OrderAccumulation(Timeout::new(Duration::from_secs(6)))
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
