pub mod bundle;
pub mod common;
pub mod order;

use tokio::sync::mpsc::UnboundedSender;
/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<SimEvent>
}

impl RevmClient {
    pub fn new(transaction_tx: UnboundedSender<SimEvent>) -> Self {
        Self { transaction_tx }
    }
}
