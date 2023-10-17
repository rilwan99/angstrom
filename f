use std::{
    ops::{Deref, DerefMut},
    pin::{pin, Pin},
    task::{Context, Poll},
    time::Duration
};

use common::{AtomicConsensus, IsLeader};
use futures::{Future, Stream};

use self::{
    bundle::BundleVoteManager, commit::CommitState, pre_propose::PreProposeState,
    propose::ProposeState, submit::SubmitState
};

pub mod bundle;
pub mod commit;
pub mod leader;
pub mod pre_propose;
pub mod propose;
pub mod submit;

trait StateTransition {
    fn should_transition(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<RoundAction>;
}

#[repr(transparent)]
pub struct Timeout(Pin<Box<dyn Future<Output = ()>>>);

impl Deref for Timeout {
    type Target = Pin<Box<dyn Future<Output = ()>>>;

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

/// The current state and subsequent actions that should be taken
/// for such state in a given round. All state that this contains
/// is transient to the given ethereum block height
pub struct RoundState {
    /// The current ethereum height
    current_height: u64,
    /// If this guard is leader for the given height
    is_leader:      IsLeader,
    /// global consensus state indicator
    consensus:      AtomicConsensus,
    /// the current action we should be taking at the moment of
    /// time for this height
    current_state:  RoundAction /* all votes for the given height
                                 * vote_collector: BundleVoteManager */
}

impl RoundState {
    pub fn new(current_height: u64, is_leader: bool) -> Self {
        Self { is_leader, current_height, current_state: RoundAction::new(is_leader) }
    }
}

/// Representation of a finite-state machine
pub enum RoundAction {
    /// The precommit state actions we
    PrePropose(PreProposeState),
    Propose(ProposeState),
    Commit(CommitState),
    Submit(SubmitState)
}

impl RoundAction {
    pub fn new(is_leader: bool) -> Self {
        // placeholder timeout
        Self::PrePropose(PreProposeState::new(timeout::new(Duration::from_secs(10))))
    }
}

impl Stream for RoundAction {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let transition = match *self {
            RoundAction::PrePropose(p) => pin!(p).should_transition(cx),
            _ => panic!()
        };

        todo!()
    }
}
