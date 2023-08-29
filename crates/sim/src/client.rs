use ethers_core::types::transaction::eip2718::TypedTransaction;
use tokio::sync::mpsc::UnboundedSender;

use crate::{Simulator, TransactionType};

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
    async fn run_sim(&self, transaction: TypedTransaction) {
        todo!()
    }
}
