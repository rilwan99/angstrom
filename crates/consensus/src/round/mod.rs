use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use alloy_primitives::FixedBytes;
use angstrom_types::consensus::{Commit, PreProposal, Proposal};
use futures::{Future, FutureExt};
use reth_primitives::B512;

pub enum Leader {
    ThisNode(B512),
    OtherNode(B512)
}

impl Leader {
    pub fn address(&self) -> FixedBytes<64> {
        match self {
            Self::ThisNode(a) => *a,
            Self::OtherNode(a) => *a
        }
    }

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
    /// Who is the current leader
    leader:        Leader,
    /// the current action we should be taking at the moment of
    /// time for this height
    current_state: RoundAction
}

impl RoundState {
    #[allow(dead_code)]
    pub fn new(height: u64, leader: Leader) -> Self {
        Self { leader, height, current_state: RoundAction::new() }
    }

    pub fn new_height(&mut self, block_height: u64, leader: Leader) {
        assert!(block_height > self.height);

        self.height = block_height;
        self.leader = leader;
        self.current_state = RoundAction::new();
    }

    pub fn current_height(&self) -> u64 {
        self.height
    }

    #[allow(dead_code)]
    pub fn on_commit(&mut self, _commit: Commit) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn on_proposal(&mut self, _proposal: Proposal) {
        todo!()
    }

    #[allow(dead_code)]
    pub fn on_pre_propose(&mut self, _pre_propose: PreProposal) {
        todo!()
    }
}

// impl Stream for RoundState {
//     type Item = RoundStateMessage;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) ->
// Poll<Option<Self::Item>> {         let is_leader = self.is_leader.clone();
//         if let Poll::Ready((new_action, msg)) = Pin::new(&mut
// self.current_state)             .should_transition(cx, GlobalStateContext {
// is_leader })             .map(|(round_action, new_state, message)| {
//                 self.consensus.update_state(new_state);
//                 (round_action, message)
//             })
//         {
//             self.current_state = new_action;
//             return_if!(msg => { is_some() } map(Poll::Ready));
//         }

//         Poll::Pending
//     }
// }

pub enum RoundStateMessage {
    /// All angstroms broadcast their complete order view across the network
    PrePropose(),
    /// the leader for this round will send out the vanilla bundle and
    /// lower-bound commit for the round
    Proposal(),
    /// the commit or nil vote the the lower-bound + vanilla proposal
    Commit(),
    /// if leader. then the finalized bundle that is sent to builders
    RelaySubmission()
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
}

impl RoundAction {
    /// Get the timeout of our current round action
    pub fn timeout(&self) -> Option<&Timeout> {
        match self {
            Self::OrderAccumulation(t) => Some(t),
            Self::PrePropose(t) => Some(t),
            Self::PreProposeLaggards(t) => Some(t),
            Self::Propose => None
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
