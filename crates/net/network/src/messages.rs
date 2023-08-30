use std::net::SocketAddr;

use ethers_core::types::transaction::eip712::TypedData;
use guard_eth_wire::EthMessage;
use shared::{Bundle, SealedBundle, TeeAddress};
use tokio::sync::oneshot::Sender as OneSender;

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    PropagateTransactions(Vec<Arc<TypedData>>),
    PropagateSealedBundle(Arc<SealedBundle>),
    PropagateSignatureRequest(Arc<Bundle>),
    PropagateBundleSignature(Signature),

    /// This is only for receiving and will never be propagated
    /// so we don't have to worry about this when we batch propagate
    /// to the network
    PeerRequests(PeerRequests)
}

/// Specific requests from a peer
#[derive(Debug)]
pub enum PeerRequests {
    GetTeeModule(OneSender<TeeAddress>)
}

impl PeerRequests {
    pub fn create_request_message(&self, request_id: u64) -> EthMessage {
        match self {
            PeerRequests::GetTeeModule(message) => {
                EthMessage::GetTeeModule(guard_eth_wire::message::RequestPair {
                    request_id,
                    message
                })
            }
        }
    }
}

/// All response variants for [`PeerResponse`]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum PeerResponseResult {
    TeeModule(SocketAddr)
}

/// Dummy implementation, this will never be cloned
impl Clone for PeerRequests {
    fn clone(&self) -> Self {
        unreachable!()
    }
}
