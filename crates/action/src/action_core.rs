use std::{
    sync::Arc,
    task::{Context, Poll},
    time::SystemTime
};

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

pub struct ActionConfig<S: Simulator + 'static> {
    pub simulator:  S,
    pub edsca_key:  LocalWallet,
    pub bundle_key: LocalWallet
}

#[derive(Debug, Clone)]
pub enum ActionMessage {
    NewBestBundle(Arc<SimmedBundle>),
    NewValidUserTransaction(Arc<SimmedUserSettlement>),
    NewValidSearcherTransaction(Arc<SimmedLvrSettlement>)
}

impl From<CowMsg> for ActionMessage {
    fn from(value: CowMsg) -> Self {
        match value {
            CowMsg::NewBestBundle(b) => ActionMessage::NewBestBundle(b),
            CowMsg::NewUserTransaction(t) => ActionMessage::NewValidUserTransaction(t),
            CowMsg::NewSearcherTransaction(t) => ActionMessage::NewValidSearcherTransaction(t)
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
    cow_solver: CowSolver<S>,
    /// used to know the current blocks
    last_block: SystemTime
}

impl<S: Simulator + Unpin> ActionCore<S> {
    pub async fn new(config: ActionConfig<S>) -> anyhow::Result<Self> {
        let ActionConfig { simulator, .. } = config;

        let last_block = SystemTime::now();

        Ok(Self { cow_solver: CowSolver::new(simulator.clone(), vec![]), last_block })
    }

    pub fn get_cow(&mut self) -> &mut CowSolver<S> {
        &mut self.cow_solver
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Option<ActionMessage>> {
        self.cow_solver
            .poll_next_unpin(cx)
            .map(|op| op.map(Into::into))
    }
}
