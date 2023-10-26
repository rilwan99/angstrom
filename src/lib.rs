#![feature(result_option_inspect)]

mod bundle_builder;
mod eth_manager;
mod guard;
mod relay_sender;
mod round_robin_sync;

use common::{AtomicConsensus, IsLeader};
pub use eth_manager::*;
use ethers_signers::LocalWallet;
pub use guard::*;
use order_pool::*;
use sim::Simulator;

pub struct GeneralConfig<S: Simulator + Unpin + 'static> {
    pub simulator:           S,
    pub ecdsa_key:           LocalWallet,
    pub submission_key:      LocalWallet,
    pub consensus_lifecycle: AtomicConsensus,
    pub is_leader:           IsLeader
}
