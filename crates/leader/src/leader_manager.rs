use std::{
    sync::Arc,
    task::{Context, Poll}
};

use bundler::{BundleSigner, BundleSigningError, CowMsg, CowSolver};
use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::Middleware;
use ethers_signers::LocalWallet;
use futures::stream::StreamExt;
use reth_primitives::{Address, U64};
use shared::{Batch, BatchSignature, Eip712};
use sim::Simulator;
use tracing::info;
use url::Url;

use crate::leader_sender::LeaderSender;

const SIMULATION_RELAY: &str = "https://relay.flashbots.net";

static BUILDER_URLS: &[&str] = &[
    "https://builder0x69.io",
    "https://rpc.beaverbuild.org",
    "https://relay.flashbots.net",
    "https://rsync-builder.xyz",
    "https://rpc.titanbuilder.xyz",
    "https://api.blocknative.com/v1/auction",
    "https://mev.api.blxrbdn.com",
    "https://eth-builder.com",
    "https://builder.gmbit.co/rpc",
    "https://buildai.net",
    "https://rpc.payload.de",
    "https://rpc.lightspeedbuilder.info",
    "https://rpc.nfactorial.xyz"
];

pub struct LeaderConfig<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    pub middleware: &'static M,
    pub simulator:  S,
    pub edsca_key:  LocalWallet,
    pub bundle_key: LocalWallet
}

#[derive(Debug, Clone)]
pub enum LeaderMessage {
    NewBestBundle(Batch),
    NewValidTransactions(Arc<Vec<Eip712>>)
}

impl From<CowMsg> for LeaderMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => LeaderMessage::NewBestBundle(b),
            CowMsg::NewTransactions(t) => LeaderMessage::NewValidTransactions(t)
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
    /// used for signature collection
    leader_sender:        LeaderSender<M>,
    /// used to sim and then sign bundles that are requested
    /// by leader
    bundle_signer:        BundleSigner<S>,
    /// deals with our bundle state
    cow_solver:           CowSolver<S>,
    /// used to make basic requests
    full_node_req:        &'static M
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Leader<M, S> {
    pub fn new(config: LeaderConfig<M, S>) -> anyhow::Result<Self> {
        let LeaderConfig { middleware, simulator, edsca_key, bundle_key } = config;
        Ok(Self {
            full_node_req:        middleware,
            cow_solver:           CowSolver::new(simulator.clone()),
            bundle_signer:        BundleSigner::new(simulator, edsca_key.clone()),
            active_leader_config: None,
            leader_sender:        LeaderSender::new(Arc::new(SignerMiddleware::new(
                BroadcasterMiddleware::new(
                    middleware,
                    BUILDER_URLS
                        .into_iter()
                        .map(|u| Url::parse(u).unwrap())
                        .collect(),
                    Url::parse(SIMULATION_RELAY)?,
                    bundle_key
                ),
                edsca_key
            )))
        })
    }

    pub fn new_transactions(&mut self, txes: Vec<Eip712>) {
        self.cow_solver.new_transactions(txes)
    }

    pub fn on_sign_batch(&mut self, batch: Batch) -> Result<BatchSignature, BundleSigningError> {
        self.bundle_signer.verify_batch_for_inclusion(batch)
    }

    pub fn is_leader(&self) -> bool {
        if let Some(leader) = self.active_leader_config.as_ref() {
            return self.bundle_signer.get_key() == leader.selected_leader
        }
        return false
    }

    pub fn current_leader(&self) -> Option<&LeaderInfo> {
        self.active_leader_config.as_ref()
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Vec<LeaderMessage>> {
        let mut res = Vec::with_capacity(10);

        if let Poll::Ready(_) = self.leader_sender.poll(cx) {
            info!("leader submitted bundle");
        }

        while let Poll::Ready(Some(cow_solver_msg)) = self.cow_solver.poll_next_unpin(cx) {
            res.push(cow_solver_msg.into());
        }

        if !res.is_empty() {
            return Poll::Ready(res)
        }

        Poll::Pending
    }
}
