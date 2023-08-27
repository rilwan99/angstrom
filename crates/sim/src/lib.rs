use ethers_core::types::transaction::eip712::EIP712Domain;
use sim::SimResult;
use tokio::sync::oneshot::Sender;

pub mod client;
pub mod revm;
pub mod sim;
pub mod executor;

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator {    
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id: u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: EIP712Domain);

}


/// enum of transaction type
pub enum TransactionType {
    Single(EIP712Domain, Sender<SimResult>),
    Bundle(EIP712Domain, Sender<SimResult>),
}