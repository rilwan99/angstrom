use ethers_core::types::transaction::eip712::EIP712Domain;
use sim::SimResult;
use tokio::sync::oneshot::{Sender, Receiver};
use eyre::Result;

pub mod revm;
pub mod sim;
pub mod executor;

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator: Clone {
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id: u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: EIP712Domain, tx: Sender<SimResult>);
}
