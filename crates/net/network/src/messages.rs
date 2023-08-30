use std::{net::SocketAddr, sync::Arc};
use reth_interfaces::p2p::error::RequestResult;
use ethers_core::types::transaction::eip712::TypedData;
use guard_eth_wire::EthMessage;
use shared::{Bundle, BundleSignature, Eip712, SealedBundle, TeeAddress};
guard_eth_wire::message::RequestPair;
use tokio::sync::oneshot::Sender as OneSender;

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    PropagateTransactions(Arc<Vec<Eip712>>),
    PropagateSealedBundle(Arc<SealedBundle>),
    PropagateSignatureRequest(Arc<Bundle>),
    PropagateBundleSignature(Arc<BundleSignature>),

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
            PeerRequests::GetTeeModule(_) => {
                EthMessage::GetTeeModule(guard_eth_wire::message::RequestPair {
                    request_id,
                    message: ()
                })
            }
        }
    }
}

/// All response variants for [`PeerResponse`]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum PeerResponseResult {
    TeeModule(RequestResult<TeeAddress>)
}
impl PeerResponseResult {
    pub fn try_into_message(self, id: u64) -> RequestResult<EthMessage> {
        match self {
            PeerResponseResult::TeeModule(addr) =>  {
                match addr {
                    Ok(msg) => Ok(EthMessage::TeeModule(RequestPair { request_id: id, message: msg})),
                    Err(err) => Err(err)

                }
            }
        }
    }
}



/// Dummy implementation, this will never be cloned
impl Clone for PeerRequests {
    fn clone(&self) -> Self {
        unreachable!()
    }
}
