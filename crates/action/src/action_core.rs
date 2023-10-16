use std::{
    collections::VecDeque,
    sync::Arc,
    task::{Context, Poll},
    time::SystemTime
};

use ethers_core::types::{Block, H256};
use ethers_providers::{Middleware, PubsubClient, SubscriptionStream};
use ethers_signers::LocalWallet;
use futures::stream::StreamExt;
use guard_types::on_chain::{SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement};
use sim::Simulator;

use crate::{CowMsg, CowSolver};

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

pub struct ActionConfig<M: Middleware + Unpin + 'static, S: Simulator + 'static> {
    pub middleware: &'static M,
    pub simulator:  S,
    pub edsca_key:  LocalWallet,
    pub bundle_key: LocalWallet
}

#[derive(Debug, Clone)]
pub enum ActionMessage {
    NewBestBundle(Arc<SimmedBundle>),
    NewValidUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    NewValidSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>)
}

impl From<CowMsg> for ActionMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => ActionMessage::NewBestBundle(b),
            CowMsg::NewUserTransactions(t) => ActionMessage::NewValidUserTransactions(t),
            CowMsg::NewSearcherTransactions(t) => ActionMessage::NewValidSearcherTransactions(t)
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
pub struct ActionCore<M: Middleware + Unpin + 'static, S: Simulator + 'static>
where
    <M as Middleware>::Provider: PubsubClient
{
    /// deals with our bundle state
    cow_solver:      CowSolver<S>,
    /// used to know the current blocks
    block_stream:    SubscriptionStream<'static, M::Provider, Block<H256>>,
    /// timestamp of last block
    last_block:      SystemTime,
    /// queue of messages
    outbound_buffer: VecDeque<ActionMessage>
}

impl<M: Middleware + Unpin, S: Simulator + Unpin> ActionCore<M, S>
where
    <M as Middleware>::Provider: PubsubClient
{
    pub async fn new(config: ActionConfig<M, S>) -> anyhow::Result<Self> {
        let ActionConfig { middleware, simulator, .. } = config;
        let mut block_stream = middleware.subscribe_blocks().await?;

        // we do this to make sure we are in sync from the start
        block_stream.next().await;
        let last_block = SystemTime::now();

        Ok(Self {
            block_stream,
            cow_solver: CowSolver::new(simulator.clone(), vec![]),
            last_block,
            outbound_buffer: VecDeque::default()
        })
    }

    pub fn get_cow(&mut self) -> &mut CowSolver<S> {
        &mut self.cow_solver
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Vec<ActionMessage>> {
        let mut res = self.outbound_buffer.drain(..).collect::<Vec<_>>();

        if let Poll::Ready(Some(_)) = self.block_stream.poll_next_unpin(cx) {
            self.last_block = SystemTime::now();
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
