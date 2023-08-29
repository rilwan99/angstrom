use std::task::{Context, Poll};

use bundler::{BundleSigner, CowMsg, CowSolver, SimulatedTransaction};
use ethers_core::types::transaction::eip712::TypedData;
use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Middleware;
use ethers_signers::LocalWallet;
use reth_primitives::{Address, U64};
use shared::{Bundle, SealedBundle};
use sim::Simulator;
use url::Url;

use crate::leader_sender::LeaderSender;

const SIMULATION_RELAY: &str = "https://relay.flashbots.net";

pub struct LeaderConfig<'a, M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    middleware: &'static M,
    simulator:  S,
    edsca_key:  LocalWallet,
    bundle_key: LocalWallet,
    // we can covert this
    relays:     &'a [&'a str]
}

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
pub struct LeaderInfo {
    /// the current selected leader
    pub selected_leader: Address,
    /// block number to check for stale state
    pub block_number:    U64
}

/// handles tasks around dealing with a leader
pub struct Leader<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    /// actively tells us who the selected leader is
    active_leader_config: Option<LeaderInfo>,
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
    pub fn new(config: LeaderConfig<M, S>) -> anyhow::Result<Self> {
        let LeaderConfig { middleware, simulator, edsca_key, bundle_key, relays } = config;
        Ok(Self {
            full_node_req:        middleware,
            cow_solver:           CowSolver::new(simulator.clone()),
            bundle_signer:        BundleSigner::new(simulator, edsca_key.clone()),
            active_leader_config: None,
            leader_sender:        LeaderSender(SignerMiddleware::new(
                BroadcasterMiddleware::new(
                    middleware,
                    relays.iter().map(|url| Url::parse(url).unwrap()).collect(),
                    Url::parse(SIMULATION_RELAY)?,
                    bundle_key
                ),
                edsca_key
            ))
        })
    }

    pub fn new_transaction(&mut self, txes: Vec<TypedData>) {
        todo!()
    }

    pub fn current_leader(&self) -> Option<&LeaderInfo> {
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
