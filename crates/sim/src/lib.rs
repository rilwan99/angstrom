use ethers_core::types::transaction::{eip2718::TypedTransaction, eip712::TypedData};
use sim::SimResult;
use tokio::sync::oneshot::Sender;

pub mod client;
pub mod executor;
pub mod lru_db;
pub mod reth_client;
pub mod revm;
pub mod sim;
pub mod state;

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator {
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id:
    // u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: TypedTransaction);
}

/// enum of transaction type
/// CHANGE TO EIP712DOMAIN
pub enum TransactionType {
    Single(TypedData, Sender<SimResult>),
    Bundle(TypedData, Sender<SimResult>)
}
