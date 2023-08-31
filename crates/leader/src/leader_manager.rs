use std::{
    collections::VecDeque,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, SystemTime}
};

use bundler::{BundleSigner, BundleSigningError, CowMsg, CowSolver};
use ethers_core::types::{Block, H256};
use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
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
    NewBestBundle(Arc<Batch>),
    NewValidTransactions(Arc<Vec<Eip712>>),
    GetBundleSignatures(Arc<Batch>)
}

impl From<CowMsg> for LeaderMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => LeaderMessage::NewBestBundle(b),
            CowMsg::NewTransactions(t) => LeaderMessage::NewValidTransactions(t)
        }
    }
}

pub const EIGHT_SECONDS: CriticalDurations = CriticalDurations::EightSeconds;
pub const TEN_SECONDS: CriticalDurations = CriticalDurations::TenSeconds;

pub enum CriticalDurations {
    InitPhase,
    EightSeconds,
    TenSeconds
}

impl CriticalDurations {
    pub fn get_mode(duration: Duration) -> CriticalDurations {
        match duration.as_millis() {
            0..=7999 => CriticalDurations::InitPhase,
            8000..=9999 => CriticalDurations::EightSeconds,
            _ => CriticalDurations::TenSeconds
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
pub struct Leader<M: Middleware + Unpin + 'static, S: Simulator + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// actively tells us who the selected leader is
    active_leader_config: Option<LeaderInfo>,
    /// used for signautre collections and reaching consensus
    leader_sender:        LeaderSender<M>,
    /// used to sim and then sign bundles that are requested
    /// by leader
    bundle_signer:        BundleSigner<S>,
    /// deals with our bundle state
    cow_solver:           CowSolver<S>,
    /// used to know the current blocks
    block_stream:         SubscriptionStream<'static, M::Provider, Block<H256>>,
    /// timestamp of last block
    last_block:           SystemTime,
    outbound_buffer:      VecDeque<LeaderMessage>
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> Leader<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(config: LeaderConfig<M, S>) -> anyhow::Result<Self> {
        let LeaderConfig { middleware, simulator, edsca_key, bundle_key } = config;
        let mut block_stream = middleware.subscribe_blocks().await?;

        // we do this to make sure we are in sync from the start
        block_stream.next().await;
        let last_block = SystemTime::now();

        Ok(Self {
            block_stream,
            cow_solver: CowSolver::new(simulator.clone()),
            bundle_signer: BundleSigner::new(simulator, edsca_key.clone()),
            active_leader_config: None,
            leader_sender: LeaderSender::new(Arc::new(SignerMiddleware::new(
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
            ))),
            last_block,
            outbound_buffer: VecDeque::default()
        })
    }

    pub fn new_transactions(&mut self, txes: Vec<Eip712>) {
        self.cow_solver.new_transactions(txes)
    }

    pub fn on_new_bundle(&mut self, bundle: Batch) {
        self.cow_solver.new_bundle(bundle);
    }

    pub fn on_sign_batch(&mut self, batch: &Batch) -> Result<BatchSignature, BundleSigningError> {
        if matches!(
            self.get_duration(),
            CriticalDurations::EightSeconds | CriticalDurations::TenSeconds
        ) {
            return self.bundle_signer.verify_batch_for_inclusion(&batch)
        }

        return Err(BundleSigningError::NotDelegatedSigningTime)
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

    fn get_duration(&self) -> CriticalDurations {
        CriticalDurations::get_mode(SystemTime::now().duration_since(self.last_block).unwrap())
    }

    fn start_settlement_if_leader(&mut self) {
        if self.is_leader()
            && !self.leader_sender.has_submitted()
            && !self.leader_sender.has_selected_bundle()
        {
            if let Some(bundle) = self.cow_solver.best_bundle().cloned() {
                self.leader_sender.set_selected_batch(bundle.clone());
                // we should never produce a invalid bundle signing
                let local_res = self
                    .bundle_signer
                    .verify_batch_for_inclusion(&bundle)
                    .unwrap();

                self.leader_sender.new_signature(local_res);
                self.outbound_buffer
                    .push_back(LeaderMessage::GetBundleSignatures(bundle.into()))
            }
        }
    }

    fn on_new_block(&mut self) {
        self.leader_sender.on_new_block();
        self.cow_solver.new_block();
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Vec<LeaderMessage>> {
        let mut res = self.outbound_buffer.drain(..).collect::<Vec<_>>();

        if let Poll::Ready(Some(_)) = self.block_stream.poll_next_unpin(cx) {
            self.on_new_block();
            self.last_block = SystemTime::now();
        }

        match self.get_duration() {
            CriticalDurations::EightSeconds => self.start_settlement_if_leader(),
            _ => {}
        }

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
