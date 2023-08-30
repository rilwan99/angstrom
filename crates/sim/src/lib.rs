use client::RevmClient;
use ethers_core::types::transaction::{eip2718::TypedTransaction, eip712::TypedData};
use ethers_middleware::Middleware;
use sim::SimResult;
use tokio::{
    sync::{mpsc::unbounded_channel, oneshot::Sender},
    task::JoinHandle,
};

use crate::{revm::Revm, sim::SimError};

pub mod client;
pub mod executor;
pub mod lru_db;
pub mod revm;
pub mod sim;
pub mod state;

pub fn spawn_revm_sim<M: Middleware>(middleware: &'static M, max_bytes: usize) -> RevmClient {
    let (tx, rx) = unbounded_channel();
    std::thread::spawn(move || {
        let revm_client = Revm::new(rx, middleware, max_bytes);
        let handle = revm_client.get_threadpool_handle();
        handle.block_on(revm_client);
    });

    RevmClient::new(tx)
}
// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator: Clone {
    async fn run_sim(&self, transaction: TransactionType) -> Result<SimResult, SimError>;
}

/// enum of transaction type
/// CHANGE TO EIP712DOMAIN
pub enum TransactionType {
    Single(TypedData, Sender<SimResult>),
    Bundle(TypedData, Sender<SimResult>),
}
