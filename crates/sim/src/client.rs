use tokio::sync::mpsc::UnboundedSender;

use crate::{errors::SimError, Simulator, TransactionType};

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
    fn run_sim(&self, transaction: TransactionType) -> Result<(), SimError> {
        self.transaction_tx.send(transaction)?;
        Ok(())
    }
}
