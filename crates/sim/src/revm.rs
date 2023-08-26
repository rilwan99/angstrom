use ethers_core::types::transaction::eip712::EIP712Domain;
use tokio::sync::{mpsc::UnboundedSender, oneshot::Sender};
use crate::{Simulator, sim::SimResult};
use eyre::Result;

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<TransactionType>
}

pub struct Revm {
}

#[async_trait::async_trait]
impl Simulator for RevmClient {
    async fn run_sim(&self, transaction: EIP712Domain, tx: Sender<SimResult>) -> () {
        todo!()
    }
}


pub enum TransactionType {
    Single(EIP712Domain, Sender<SimResult>),
    Bundle(EIP712Domain, Sender<SimResult>),
}