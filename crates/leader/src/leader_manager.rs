use std::task::{Context, Poll};

use bundler::{BundleSigner, CowMsg, CowSolver, SimulatedTransaction};
use ethers_core::types::transaction::eip712::TypedData;
use ethers_providers::Middleware;
use reth_primitives::{Address, U64};
use shared::{Bundle, SealedBundle};
use sim::Simulator;

use crate::leader_sender::LeaderSender;

#[derive(Debug, Clone)]
pub enum LeaderMessage {
    NewBestBundle(SealedBundle),
    NewValidTransactions(Vec<SimulatedTransaction>),
    SignedBundle(Bundle)
}

impl From<CowMsg> for LeaderMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => LeaderMessage::NewBestBundle(b),
            CowMsg::NewValidTransactions(t) => LeaderMessage::NewValidTransactions(t)
        }
    }
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
pub struct Leader<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    /// actively tells us who the selected leader is
    active_leader_config: Option<LeaderConfig>,
    /// used when selected to be leader. mostly for just submitting
    leader_sender:        LeaderSender<M>,
    /// used to sim and then sign bundles that are requested
    /// by leader
    bundle_signer:        BundleSigner<S>,
    /// deals with our bundle state
    cow_solver:           CowSolver<S>,
    /// used to make basic requests
    full_node_req:        &'static M
}

impl<M: Middleware + Unpin, S: Simulator> Leader<M, S> {
    pub fn new_transaction(&mut self, txes: Vec<TypedData>) {
        todo!()
    }

    pub fn current_leader(&self) -> Option<&LeaderConfig> {
        self.active_leader_config.as_ref()
    }

    pub fn process_msg(&mut self) {}

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Vec<LeaderMessage>> {
        let mut res = Vec::with_capacity(10);

        if !res.is_empty() {
            return Poll::Ready(res)
        }

        Poll::Pending
    }
}
