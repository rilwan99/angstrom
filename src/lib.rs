#![feature(result_option_inspect)]

mod guard;

use common::{AtomicConsensus, IsLeader};
use ethers_signers::LocalWallet;
pub use guard::*;
use validation::Simulator;

pub struct GeneralConfig<S: Simulator + Unpin + 'static> {
    pub simulator:           S,
    pub ecdsa_key:           LocalWallet,
    pub submission_key:      LocalWallet,
    pub consensus_lifecycle: AtomicConsensus,
    pub is_leader:           IsLeader
}
