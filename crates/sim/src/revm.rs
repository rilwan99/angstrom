use std::sync::mpsc::Sender;
use ethers_core::types::transaction::eip712::EIP712Domain;
use tokio::sync::mpsc::UnboundedSender;
use crate::{Simulator, sim::SimResult};

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    // some sort of handle to threadpool executor wrapped
    // in async
    sender: UnboundedSender<TransactionType>
}

impl Simulator for RevmClient {}

pub struct Revm {
}


pub enum TransactionType {
    Single(EIP712Domain, Sender<SimResult>),
    Bundle(EIP712Domain, Sender<SimResult>),
}