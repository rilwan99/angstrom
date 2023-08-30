use std::{path::Path, sync::Arc};

use client::RevmClient;
use ethers_core::types::transaction::eip712::TypedData;
use reth_db::mdbx::WriteMap;
use sim::SimResult;
use tokio::{
    sync::{mpsc::unbounded_channel, oneshot::Sender},
    task::JoinHandle
};

use crate::{revm::Revm, sim::SimError};

pub mod client;
pub mod executor;
pub mod lru_db;
pub mod revm;
pub mod sim;
pub mod state;

pub fn spawn_revm_sim(db: Arc<reth_db::mdbx::Env<WriteMap>>, max_bytes: usize) -> RevmClient {
    let (tx, rx) = unbounded_channel();
    let revm_client = Revm::new(rx, db, max_bytes);
    std::thread::spawn(move || {
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
    Bundle(TypedData, Sender<SimResult>)
}
