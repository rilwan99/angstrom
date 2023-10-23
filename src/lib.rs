mod guard;
mod network_manager;
mod relay_sender;
mod round_robin_sync;
mod submission_server;

use common::{AtomicConsensus, IsLeader};
use ethers_signers::LocalWallet;
pub use guard::*;
pub use network_manager::*;
use sim::Simulator;
pub use submission_server::SubmissionServerConfig;

pub struct GeneralConfig<S: Simulator + Unpin + 'static> {
    pub simulator:           S,
    pub ecdsa_key:           LocalWallet,
    pub submission_key:      LocalWallet,
    pub consensus_lifecycle: AtomicConsensus,
    pub is_leader:           IsLeader
}
