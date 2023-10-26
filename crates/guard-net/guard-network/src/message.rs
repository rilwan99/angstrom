//! Capability messaging
//!
//! An RLPx stream is multiplexed via the prepended message-id of a framed
//! message. Capabilities are exchanged via the RLPx `Hello` message as pairs of `(id, version)`, <https://github.com/ethereum/devp2p/blob/master/rlpx.md#capability-messaging>

use std::{
    fmt,
    sync::Arc,
    task::{ready, Context, Poll}
};

use futures::FutureExt;
use guard_eth_wire::{
    capability::RawCapabilityMessage, message::RequestPair, EthMessage, GetLimitOrders,
    GetSearcherOrders, GetUsersOrders, LimitOrders, SearcherOrders, UserOrders
};
use guard_types::{
    consensus::*,
    rpc::{SignedLimitOrder, SignedSearcherOrder}
};
use reth_interfaces::p2p::error::{RequestError, RequestResult};
use reth_primitives::{
    BlockBody, Bytes, Header, PeerId, PooledTransactionsElement, ReceiptWithBloom, B256
};
use tokio::sync::{mpsc, mpsc::error::TrySendError, oneshot};

// /// Internal form of a `NewBlock` message
// #[derive(Debug, Clone)]
// pub struct NewBlockMessage {
//     /// Hash of the block
//     pub hash:  B256,
//     /// Raw received message
//     pub block: Arc<NewBlock>
// }

// === impl NewBlockMessage ===

// impl NewBlockMessage {
//     /// Returns the block number of the block
//     pub fn number(&self) -> u64 {
//         self.block.block.header.number
//     }
// }

/// All Bi-directional eth-message variants that can be sent to a session or
/// received from a session.
#[derive(Debug)]
pub enum PeerMessage {
    // Consensus related messages
    PrePropose(Arc<PreProposal>),
    Proposal(Arc<Proposal>),
    Commit(Arc<Commit>),

    // default communication
    PropagateOrder(Arc<SignedLimitOrder>),
    /// All `eth` request variants.
    EthRequest(PeerRequest),
    /// Other than eth namespace message
    #[allow(unused)]
    Other(RawCapabilityMessage)
}

/// Protocol related request messages that expect a response
#[derive(Debug)]
#[allow(clippy::enum_variant_names, missing_docs)]
pub enum PeerRequest {
    /// Request Block headers from the peer.
    ///
    /// The response should be sent through the channel.
    GetLimitOrders {
        request:  GetLimitOrders,
        response: oneshot::Sender<RequestResult<LimitOrders>>
    },
    /// Request Block headers from the peer.
    ///
    /// The response should be sent through the channel.
    GetUserOrders {
        request:  GetUsersOrders,
        response: oneshot::Sender<RequestResult<SearcherOrders>>
    },
    /// Request pooled transactions from the peer.
    ///
    /// The response should be sent through the channel.
    GetSearcherOrders {
        request:  GetSearcherOrders,
        response: oneshot::Sender<RequestResult<SearcherOrders>>
    }
}

// === impl PeerRequest ===

impl PeerRequest {
    /// Invoked if we received a response which does not match the request
    pub(crate) fn send_bad_response(self) {
        self.send_err_response(RequestError::BadResponse)
    }

    /// Send an error back to the receiver.
    pub(crate) fn send_err_response(self, err: RequestError) {
        let _ = match self {
            PeerRequest::GetUserOrders { response, .. } => response.send(Err(err)).ok(),
            PeerRequest::GetLimitOrders { response, .. } => response.send(Err(err)).ok(),
            PeerRequest::GetSearcherOrders { response, .. } => response.send(Err(err)).ok()
        };
    }

    /// Returns the [`EthMessage`] for this type
    pub fn create_request_message(&self, request_id: u64) -> EthMessage {
        match self {
            PeerRequest::GetUserOrders { request, .. } => {
                EthMessage::GetUserOrders(RequestPair { request_id, message: request.clone() })
            }
            PeerRequest::GetSearcherOrders { request, .. } => {
                EthMessage::GetSearcherOrders(RequestPair { request_id, message: request.clone() })
            }
            PeerRequest::GetLimitOrders { request, .. } => {
                EthMessage::GetLimitOrders(RequestPair { request_id, message: request.clone() })
            }
        }
    }
}

/// Corresponding variant for [`PeerRequest`].
#[derive(Debug)]
pub enum PeerResponse {
    SearcherOrder { response: oneshot::Receiver<RequestResult<SignedSearcherOrder>> },
    LimitOrder { response: oneshot::Receiver<RequestResult<SignedLimitOrder>> },
    UserOrder { response: oneshot::Receiver<RequestResult<SignedSearcherOrder>> }
}

// === impl PeerResponse ===

impl PeerResponse {
    /// Polls the type to completion.
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<PeerResponseResult> {
        macro_rules! poll_request {
            ($response:ident, $item:ident, $cx:ident) => {
                match ready!($response.poll_unpin($cx)) {
                    Ok(res) => PeerResponseResult::$item(res.map(|item| item.0)),
                    Err(err) => PeerResponseResult::$item(Err(err.into()))
                }
            };
        }

        let res = match self {
            PeerResponse::LimitOrder { response } => {
                poll_request!(response, LimitOrders, cx)
            }
            PeerResponse::SearcherOrder { response } => {
                poll_request!(response, SearcherOrders, cx)
            }
            PeerResponse::UserOrder { response } => {
                poll_request!(response, UserOrder, cx)
            }
        };
        Poll::Ready(res)
    }
}

/// All response variants for [`PeerResponse`]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum PeerResponseResult {
    LimitOrders(RequestResult<Vec<SignedLimitOrder>>),
    SearcherOrders(RequestResult<Vec<SignedSearcherOrder>>),
    UserOrder(RequestResult<Vec<SignedSearcherOrder>>)
}

// === impl PeerResponseResult ===

impl PeerResponseResult {
    /// Converts this response into an [`EthMessage`]
    pub fn try_into_message(self, id: u64) -> RequestResult<EthMessage> {
        macro_rules! to_message {
            ($response:ident, $item:ident, $request_id:ident) => {
                match $response {
                    Ok(res) => {
                        let request =
                            RequestPair { request_id: $request_id, message: $item(res) };
                        Ok(EthMessage::$item(request))
                    }
                    Err(err) => Err(err)
                }
            };
        }
        match self {
            PeerResponseResult::UserOrder(resp) => {
                to_message!(resp, UserOrders, id)
            }
            PeerResponseResult::LimitOrders(resp) => {
                to_message!(resp, LimitOrders, id)
            }
            PeerResponseResult::SearcherOrders(resp) => {
                to_message!(resp, SearcherOrders, id)
            }
        }
    }

    /// Returns the `Err` value if the result is an error.
    pub fn err(&self) -> Option<&RequestError> {
        match self {
            PeerResponseResult::UserOrder(res) => res.as_ref().err(),
            PeerResponseResult::LimitOrders(res) => res.as_ref().err(),
            PeerResponseResult::SearcherOrders(res) => res.as_ref().err()
        }
    }

    /// Returns whether this result is an error.
    #[allow(unused)]
    pub fn is_err(&self) -> bool {
        match self {
            PeerResponseResult::UserOrder(res) => res.is_err(),
            PeerResponseResult::LimitOrders(res) => res.is_err(),
            PeerResponseResult::SearcherOrders(res) => res.is_err()
        }
    }
}

/// A Cloneable connection for sending _requests_ directly to the session of a
/// peer.
#[derive(Clone)]
pub struct PeerRequestSender {
    /// id of the remote node.
    pub(crate) peer_id:       PeerId,
    /// The Sender half connected to a session.
    pub(crate) to_session_tx: mpsc::Sender<PeerRequest>
}

// === impl PeerRequestSender ===

impl PeerRequestSender {
    /// Constructs a new sender instance that's wired to a session
    pub(crate) fn new(peer_id: PeerId, to_session_tx: mpsc::Sender<PeerRequest>) -> Self {
        Self { peer_id, to_session_tx }
    }

    /// Attempts to immediately send a message on this Sender
    pub fn try_send(&self, req: PeerRequest) -> Result<(), TrySendError<PeerRequest>> {
        self.to_session_tx.try_send(req)
    }

    /// Returns the peer id of the remote peer.
    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }
}

impl fmt::Debug for PeerRequestSender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeerRequestSender")
            .field("peer_id", &self.peer_id)
            .finish_non_exhaustive()
    }
}
