use std::{
    sync::Arc,
    task::{Context, Poll}
};

use common::{AtomicConsensus, IsLeader};
use ethers_signers::LocalWallet;
use futures::stream::StreamExt;
use guard_types::on_chain::{BestBundles, SubmittedOrder, VanillaBundle};
use sim::Simulator;

use crate::{BundleSolver, BundleSolverMsg};

pub struct ActionConfig<S: Simulator + 'static> {
    pub simulator:           S,
    pub ecdsa_key:           LocalWallet,
    pub submission_key:      LocalWallet,
    pub consensus_lifecycle: AtomicConsensus,
    pub is_leader:           IsLeader
}

#[derive(Debug, Clone)]
pub enum ActionMessage {
    NewBestVanilla(Arc<VanillaBundle>),
    NewOrder(Arc<SubmittedOrder>),
    NewBestBundles(BestBundles)
}

impl From<BundleSolverMsg> for ActionMessage {
    fn from(value: BundleSolverMsg) -> Self {
        match value {
            BundleSolverMsg::NewBestVanilla(b) => ActionMessage::NewBestVanilla(b),
            BundleSolverMsg::NewOrder(t) => ActionMessage::NewOrder(t)
        }
    }
}

/// The Action Modules design is the counterpart to the consensus design. That
/// being that we handle all unknowns, building and comparisons here. This
/// mostly refers to building new bundles, comparing other bundles as-well as
/// dealing with supplying our consensus module with Events every time we
/// calculate something that is strictly more optimal than what our current
/// Consensus is looking at. Most external functions such as adding
/// quotability, or storage slot pricing for composable bundle occurs in this
/// module.
pub struct ActionCore<S: Simulator + 'static> {
    /// deals with our bundle state
    bundle_solver: BundleSolver<S>,
    /// current consensus lifecycle
    lifecycle:     AtomicConsensus,
    /// if we are leader
    is_leader:     IsLeader
}

impl<S: Simulator + Unpin> ActionCore<S> {
    pub async fn new(config: ActionConfig<S>) -> anyhow::Result<Self> {
        let ActionConfig { simulator, consensus_lifecycle, is_leader, .. } = config;

        Ok(Self {
            bundle_solver: BundleSolver::new(simulator.clone(), vec![]),
            // placeholders
            lifecycle: consensus_lifecycle,
            is_leader
        })
    }

    pub fn bundle_solver(&mut self) -> &mut BundleSolver<S> {
        &mut self.bundle_solver
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Option<ActionMessage>> {
        self.bundle_solver
            .poll_next_unpin(cx)
            .map(|op| op.map(Into::into))
    }
}
