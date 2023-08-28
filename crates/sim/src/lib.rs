use ethers_core::types::transaction::eip2718::TypedTransaction;
use sim::SimResult;
use tokio::sync::oneshot::Sender;

pub mod client;
pub mod revm;
pub mod sim;
pub mod executor;
pub mod state;
pub mod lru_db;

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator {    
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id: u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: TypedTransaction);

}


/// enum of transaction type
pub enum TransactionType {
    Single(TypedTransaction, Sender<SimResult>),
    Bundle(TypedTransaction, Sender<SimResult>),
}




