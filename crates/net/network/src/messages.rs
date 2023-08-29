use std::net::SocketAddr;

use ethers_core::types::transaction::eip712::TypedData;
use shared::{Bundle, SealedBundle};
use tokio::sync::oneshot::Sender as OneSender;

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    PropagateTransaction(TypedData),
    PropagateSealedBundle(SealedBundle),
    PropagateSignatureRequest(Bundle),
    PropagateSignedBundle(Bundle),
    PeerRequests(PeerRequests)
}

impl PeerMessages {
    pub fn is_request(&self) -> bool {
        matches!(self, PeerMessages::PeerRequests(_))
    }

}



/// Specific requests from a peer
#[derive(Debug)]
pub enum PeerRequests {
    GetTeeModule(OneSender<SocketAddr>)
}

