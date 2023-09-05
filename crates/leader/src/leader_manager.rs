use std::{
    collections::VecDeque,
    sync::Arc,
    task::{Context, Poll},
    time::{Duration, SystemTime}
};

use bundler::{BundleError, BundleSigner, CowMsg, CowSolver};
use ethers_core::types::{Block, H256};
use ethers_flashbots::BroadcasterMiddleware;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use ethers_signers::LocalWallet;
use futures::stream::StreamExt;
use reth_primitives::{Address, U64};
use shared::{BundleSignature, SafeTx, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement};
use sim::Simulator;
use tracing::{error, info};
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
    NewBestBundle(Arc<SimmedBundle>),
    NewValidUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    NewValidSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>),
    GetBundleSignatures(Arc<SafeTx>),
    PropagateSignature(Arc<BundleSignature>)
}

impl From<CowMsg> for LeaderMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => LeaderMessage::NewBestBundle(b),
            CowMsg::NewUserTransactions(t) => LeaderMessage::NewValidUserTransactions(t),
            CowMsg::NewSearcherTransactions(t) => LeaderMessage::NewValidSearcherTransactions(t)
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
pub struct Leader<M: Middleware + Unpin + 'static, S: Simulator + Unpin + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// actively tells us who the selected leader is
    active_leader_config: Option<LeaderInfo>,
    /// used for signature collections and reaching consensus
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
    /// queue of messages
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

    pub fn get_cow(&mut self) -> &mut CowSolver<S> {
        &mut self.cow_solver
    }

    pub fn on_new_sigs(&mut self, sig: BundleSignature) {
        self.leader_sender.new_signature(sig);
    }

    pub fn on_sign_bundle(&mut self, bundle: Arc<SafeTx>) -> Result<(), BundleError> {
        if matches!(
            self.get_duration(),
            CriticalDurations::EightSeconds | CriticalDurations::TenSeconds
        ) {
            return self.bundle_signer.verify_bundle_for_inclusion(bundle)
        }

        return Err(BundleError::NotDelegatedSigningTime)
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
                self.leader_sender.set_selected_bundle(bundle.clone());
                // we should never produce a invalid bundle signing
                // if let Err(sim_err) =
                // self.bundle_signer.verify_bundle_for_inclusion(&bundle) {
                //     error!(?sim_err, "failed to verify bundle for inclusion")
                // }
                //
                // self.outbound_buffer
                //     .push_back(LeaderMessage::GetBundleSignatures(bundle.
                // into()))
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
            CriticalDurations::EightSeconds | CriticalDurations::TenSeconds => {
                self.start_settlement_if_leader()
            }
            _ => {}
        }

        if let Poll::Ready(signed_bundle) = self.bundle_signer.poll(cx) {
            match signed_bundle {
                Ok(signature) => {
                    if self.is_leader() {
                        self.leader_sender.new_signature(signature.clone());
                    } else {
                        res.push(LeaderMessage::PropagateSignature(signature.into()))
                    }
                }
                Err(e) => error!(?e, "attempt to sign bundle failed")
            }
        }

        if let Poll::Ready(_) = self.leader_sender.poll(cx) {
            info!("leader submitted bundle");
        }

        if let Poll::Ready(Some(cow_solver_msg)) = self.cow_solver.poll_next_unpin(cx) {
            res.extend(cow_solver_msg.into_iter().map(Into::into));
        }

        if !res.is_empty() {
            return Poll::Ready(res)
        }

        Poll::Pending
    }
}

#[cfg(feature = "test_harness")]
pub mod test_harness {
    use reth_primitives::{PeerId, H160};

    use super::*;

    impl<M: Middleware + Unpin, S: Simulator + Unpin> Leader<M, S>
    where
        <M as Middleware>::Provider: PubsubClient
    {
        pub fn make_leader(&mut self, id: PeerId, block: U64) {
            // gotta take last 20 bytes as thats there address

            let sized: &[u8; 20] = unsafe { &*(&id[44..] as *const _ as *mut _) };
            self.active_leader_config =
                Some(LeaderInfo { selected_leader: H160::from(sized), block_number: block });
        }
    }
}
