use std::{
    sync::Arc,
    task::{ready, Context, Poll}
};

use futures::FutureExt;
use guard_eth_wire::{message::RequestPair, EthMessage};
use reth_interfaces::p2p::error::RequestResult;
use shared::{
    BundleSignature, SafeTx, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement, TeeAddress
};
use tokio::sync::{oneshot, oneshot::Sender as OneSender};

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    /// new simmed user txes
    PropagateUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    /// new simmed searcher txes
    PropagateSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>),
    /// propagates a new bundle
    PropagateBundle(Arc<SimmedBundle>),
    /// leader request to get signatures on a new bundle
    PropagateSignatureRequest(Arc<SafeTx>),
    /// propgating the signature for the send out bundle
    PropagateBundleSignature(Arc<BundleSignature>),

    /// This is only for receiving and will never be propagated
    /// so we don't have to worry about this when we batch propagate
    /// to the network
    PeerRequests(PeerRequests)
}

/// Specific requests from a peer
#[derive(Debug)]
pub enum PeerRequests {
    /// for connecting for sealed second bid
    GetTeeModule(GetTeeModule, OneSender<TeeAddress>)
}

#[derive(Debug)]
pub struct GetTeeModule(pub TeeAddress);

impl PeerRequests {
    pub fn create_request_message(&self, request_id: u64) -> EthMessage {
        match self {
            PeerRequests::GetTeeModule(msg, _) => {
                EthMessage::GetTeeModule(guard_eth_wire::message::RequestPair {
                    request_id,
                    message: msg.0.clone()
                })
            }
        }
    }
}

pub enum PeerResponse {
    GetTeeModule(oneshot::Receiver<RequestResult<TeeAddress>>)
}

impl PeerResponse {
    /// Polls the type to completion.
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<PeerResponseResult> {
        macro_rules! poll_request {
            ($response:ident, $item:ident, $cx:ident) => {
                match ready!($response.poll_unpin($cx)) {
                    Ok(res) => PeerResponseResult::$item(res),
                    Err(err) => PeerResponseResult::$item(Err(err.into()))
                }
            };
        }

        let res = match self {
            PeerResponse::GetTeeModule(response) => {
                poll_request!(response, GetTeeModule, cx)
            }
        };

        Poll::Ready(res)
    }
}

/// All response variants for [`PeerResponse`]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum PeerResponseResult {
    GetTeeModule(RequestResult<TeeAddress>)
}
impl PeerResponseResult {
    pub fn try_into_message(self, id: u64) -> RequestResult<EthMessage> {
        match self {
            PeerResponseResult::GetTeeModule(addr) => match addr {
                Ok(msg) => {
                    Ok(EthMessage::GetTeeModule(RequestPair { request_id: id, message: msg }))
                }
                Err(err) => Err(err)
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
