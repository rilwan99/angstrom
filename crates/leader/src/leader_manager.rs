use std::task::{Context, Poll};

use ethers_providers::Middleware;
use reth_primitives::{Address, U64};
use sim::Simulator;

use crate::{
    bundle_signer::BundleSigner,
    leader_core::{LeaderCore, LeaderCoreActions}
};

pub enum LeaderAction {
    /// todo, flatten
    Core(LeaderCoreActions)
}

/// This is going to be changing.. just a placeholder
#[derive(Debug)]
pub struct LeaderConfig {
    /// the current selected leader
    pub selected_leader: Address,
    /// block number to check for stale state
    pub block_number:    U64
}

/// handles tasks around dealing with a leader
pub struct Leader<M: Middleware + Unpin + 'static, S: Simulator> {
    /// actively tells us who the selected leader is
    active_leader_config: Option<LeaderConfig>,
    /// used when selected to be leader.
    leader_core:          LeaderCore<M, S>,
    /// used to sim and then sign bundles that are requested
    /// by leader
    bundle_signer:        BundleSigner<S>,
    /// used to make basic requests
    full_node_req:        &'static M
}

impl<M: Middleware + Unpin, S: Simulator> Leader<M, S> {
    pub fn current_leader(&self) -> Option<&LeaderConfig> {
        self.active_leader_config.as_ref()
    }

    pub fn process_msg(&mut self) {}

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Vec<LeaderAction>> {
        let mut res = Vec::with_capacity(50);

        // pull all leader state
        while let Poll::Ready(msg) = self.leader_core.poll(cx) {
            res.push(LeaderAction::Core(msg));
        }

        if !res.is_empty() {
            return Poll::Ready(res)
        }

        Poll::Pending
    }
}
