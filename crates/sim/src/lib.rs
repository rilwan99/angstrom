use std::sync::Arc;

use client::RevmClient;
use errors::SimResult;
use reth_db::mdbx::WriteMap;
use shared::Eip712;
use tokio::sync::{mpsc::unbounded_channel, oneshot::Sender};

use crate::{errors::SimError, revm::Revm};

pub mod client;
pub(crate) mod errors;
pub mod executor;
pub mod lru_db;
pub mod revm;
pub mod state;

pub fn spawn_revm_sim(
    db: Arc<reth_db::mdbx::Env<WriteMap>>,
    max_bytes: usize,
) -> Result<RevmClient, SimError> {
    let (tx, rx) = unbounded_channel();
    let revm_client = Revm::new(rx, db, max_bytes)?;
    std::thread::spawn(move || {
        let handle = revm_client.get_threadpool_handle();
        handle.block_on(revm_client);
    });

    Ok(RevmClient::new(tx))
}
// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator: Clone {
    fn run_sim(&self, transaction: TransactionType) -> Result<(), SimError>;
}

/// enum of transaction type
pub enum TransactionType {
    Single(Eip712, Sender<SimResult>),
    Bundle(Eip712, Sender<SimResult>),
}
