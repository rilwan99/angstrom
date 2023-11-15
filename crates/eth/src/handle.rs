use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub struct EthHandle {
    sender: Sender<EthCommand>
}

impl EthHandle {
    pub fn new(sender: Sender<EthCommand>) -> Self {
        Self { sender }
    }
}

pub enum EthCommand {}
