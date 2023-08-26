use std::{
    ops::{Deref, DerefMut},
    task::{Context, Poll}
};

use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Middleware;
use ethers_signers::LocalWallet;
use sim::Simulator;

use crate::cow_solver::CowSolver;

type StakedWallet = LocalWallet;
type BundleKey = LocalWallet;

pub struct LeaderCoreConfig<M: Middleware>(
    pub SignerMiddleware<BroadcasterMiddleware<M, BundleKey>, StakedWallet>
);

impl<M: Middleware> Deref for LeaderCoreConfig<M> {
    type Target = SignerMiddleware<BroadcasterMiddleware<M, BundleKey>, StakedWallet>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Middleware> DerefMut for LeaderCoreConfig<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// requests that the leader can request
pub enum LeaderCoreActions {
    SignBundle(),
    GetEip712Tx(),
    GetCexDex()
}

/// leader core purely deals with building the best bundle and submitting it to
/// the specified relays
pub struct LeaderCore<M: Middleware, S: Simulator> {
    config: LeaderCoreConfig<M>,
    solver: CowSolver,

    // bundle state
    top_of_bundles: Vec<()>,
    body_txes:      Vec<()>,

    sim: S
}

impl<M: Middleware, S: Simulator> LeaderCore<M, S> {
    pub fn get_sim(&self) -> &S {
        &self.sim
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<LeaderCoreActions> {
        Poll::Pending
    }
}
