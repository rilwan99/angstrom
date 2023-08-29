use client::RevmClient;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_middleware::Middleware;
use revm::Revm;
use sim::{SimError, SimResult};
use tokio::{
    sync::{mpsc::unbounded_channel, oneshot::Sender},
    task::JoinHandle
};

pub mod client;
pub mod executor;
pub mod lru_db;
pub mod middleware;
pub mod revm;
pub mod sim;
pub mod state;

pub fn spawn_revm_sim<M: Middleware>(
    middleware: &'static M,
    max_bytes: usize
) -> (RevmClient, JoinHandle<()>) {
    let (tx, rx) = unbounded_channel();
    let revm_client = Revm::new(rx, middleware, max_bytes);

    (RevmClient::new(tx), tokio::spawn(revm_client))
}

/// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator: Clone {
    //fn run_sim(&self, transaction: EIP712Domain, tx: Receiver<SimResult>, id:
    // u64) -> Result<SimResult>;
    async fn run_sim(&self, transaction: TransactionSim) -> Result<SimResult, SimError>;
}

pub enum TransactionSim {
    Transaction(TypedTransaction, Sender<SimResult>),
    Bundle(Vec<TypedTransaction>, Sender<SimResult>)
}
