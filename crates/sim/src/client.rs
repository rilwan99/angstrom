use ethers_core::types::transaction::eip712::EIP712Domain;
use revm::db::DatabaseRef;
use tokio::sync::mpsc::UnboundedSender;
use crate::{executor::ThreadPool, Simulator, TransactionType};
use ethers_middleware::Middleware;

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<TransactionType>,
}

impl RevmClient {
    pub fn new(transaction_tx: UnboundedSender<TransactionType>) -> Self {
        Self { transaction_tx }
    }
}

#[async_trait::async_trait]
impl Simulator for RevmClient {
    async fn run_sim(&self, transaction: EIP712Domain) {
        todo!()
    }
}