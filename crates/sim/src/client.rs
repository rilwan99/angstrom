use ethers_core::types::transaction::eip2718::TypedTransaction;
use tokio::sync::{
    mpsc::UnboundedSender,
    oneshot::{channel, Sender}
};

use crate::sim::SimError;
use crate::{sim::SimResult, Simulator, TransactionType};

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<TransactionType>
}

impl RevmClient {
    pub fn new(transaction_tx: UnboundedSender<TransactionType>) -> Self {
        Self { transaction_tx }
    }
}

#[async_trait::async_trait]
impl Simulator for RevmClient {
    async fn run_sim(&self, transaction: TransactionType) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx.send(transaction);

        rx.await.unwrap()
    }
}
