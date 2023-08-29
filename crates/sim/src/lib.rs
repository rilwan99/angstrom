use client::RevmClient;
use ethers_core::types::transaction::{eip2718::TypedTransaction, eip712::TypedData};
use ethers_middleware::Middleware;
use sim::SimResult;
use tokio::sync::oneshot::Sender;

pub mod client;
pub mod executor;
pub mod lru_db;
pub mod reth_client;
pub mod revm;
pub mod sim;
pub mod state;

pub fn spawn_revm_sim<M: Middleware>(
    middleware: &'static M,
    max_bytes: usize,
) -> (RevmClient, JoinHandle<()>) {
    let (tx, rx) = unbounded_channel();
    let revm_client = Revm::new(rx, middleware, max_bytes);

    (RevmClient::new(tx), tokio::spawn(revm_client))
}

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator {
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id: u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: TypedTransaction);
}

/// enum of transaction type
/// CHANGE TO EIP712DOMAIN
pub enum TransactionType {
    Single(TypedData, Sender<SimResult>),
    Bundle(TypedData, Sender<SimResult>),
}
