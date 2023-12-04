use std::pin::Pin;

use futures_util::Stream;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;

use crate::manager::EthEvent;

pub trait Eth: Clone + Send + Sync {
    fn subscribe_network(&self) -> Pin<Box<dyn Stream<Item = EthEvent>>>;
}

pub enum EthCommand {
    SubscribeEthNetworkEvents(Sender<EthEvent>)
}

#[derive(Debug, Clone)]
pub struct EthHandle {
    sender: Sender<EthCommand>
}

impl EthHandle {
    pub fn new(sender: Sender<EthCommand>) -> Self {
        Self { sender }
    }
}

impl Eth for EthHandle {
    fn subscribe_network(&self) -> Pin<Box<dyn Stream<Item = EthEvent>>> {
        let (tx, rx) = channel(10);
        let _ = self
            .sender
            .try_send(EthCommand::SubscribeEthNetworkEvents(tx));

        Box::pin(ReceiverStream::new(rx))
    }
}
