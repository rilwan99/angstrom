use std::sync::mpsc::Sender;

use crate::Simulator;

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    // some sort of handle to threadpool executor wrapped
    // in async
    sender: UnboundedSender<TransactionType>
}

impl Simulator for RevmClient {}

pub struct Revm {
    // threadpool: Execu
}


pub enum TransactionType {
    Single(Eip712Domain, Sender<SimResult>),
    Bundle(Eip712Domain, Sender<SimResult>),
}