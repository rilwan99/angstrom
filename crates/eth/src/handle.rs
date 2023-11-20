use std::pin::Pin;

use futures_util::Stream;
use guard_types::primitive::Bundle;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;

use crate::manager::EthNetworkEvent;

pub trait Eth: Clone + Send + Sync {
    fn subscribe_network(&self) -> Pin<Box<dyn Stream<Item = EthNetworkEvent>>>;
    fn submit_bundle(&self, bundle: Bundle);
}

pub enum EthCommand {
    SubscribeEthNetworkEvents(Sender<EthNetworkEvent>),
    SubmitBundle(Bundle)
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
    fn submit_bundle(&self, bundle: Bundle) {
        let _ = self.sender.try_send(EthCommand::SubmitBundle(bundle));
    }

    fn subscribe_network(&self) -> Pin<Box<dyn Stream<Item = EthNetworkEvent>>> {
        let (tx, rx) = channel(10);
        let _ = self
            .sender
            .try_send(EthCommand::SubscribeEthNetworkEvents(tx));

        Box::pin(ReceiverStream::new(rx))
    }
}
