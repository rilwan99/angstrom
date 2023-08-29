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
    /// This is only for receving and will never be propogated
    /// so we don't have to worry about this when we batch propogate
    /// to the network
    PeerRequests(PeerRequests)
}


/// Specific requests from a peer
#[derive(Debug)]
pub enum PeerRequests {
    GetTeeModule(OneSender<SocketAddr>)
}
