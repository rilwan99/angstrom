#![allow(missing_docs)]
use std::{fmt::Debug, sync::Arc};

use alloy_rlp::{length_of_length, Decodable, Encodable, Header};
use guard_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::{GetPooledOrders, Orders, PooledOrder}
};
use reth_eth_wire::{capability::Capability, protocol::Protocol};
use reth_interfaces::p2p::error::RequestError;
use reth_primitives::bytes::{Buf, BufMut};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::errors::StromStreamError;
/// Result alias for result of a request.
pub type RequestResult<T> = Result<T, RequestError>;
use super::version::StromVersion;
use crate::Status;

/// [`MAX_MESSAGE_SIZE`] is the maximum cap on the size of a protocol message.
// https://github.com/ethereum/go-ethereum/blob/30602163d5d8321fbc68afdcbbaf2362b2641bde/eth/protocols/eth/protocol.go#L50
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// An `eth` protocol message, containing a message ID and payload.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StromProtocolMessage {
    pub message_type: StromMessageID,
    pub message:      StromMessage
}

impl StromProtocolMessage {
    /// Create a new ProtocolMessage from a message type and message rlp bytes.
    pub fn decode_message(
        _version: StromVersion,
        buf: &mut &[u8]
    ) -> Result<Self, StromStreamError> {
        let message_type = StromMessageID::decode(buf)?;

        let message = match message_type {
            StromMessageID::Status => StromMessage::Status(Status::decode(buf)?),
            StromMessageID::PrePropose => StromMessage::PrePropose(PreProposal::decode(buf)?),
            StromMessageID::Propose => StromMessage::Propose(Proposal::decode(buf)?),
            StromMessageID::Commit => StromMessage::Commit(Commit::decode(buf)?),
            StromMessageID::PropagatePooledOrders => {
                StromMessage::PropagatePooledOrders(Orders::decode(buf)?)
            }
            StromMessageID::GetPooledOrders => {
                StromMessage::GetPooledOrders(RequestPair::decode(buf)?)
            }
            StromMessageID::PooledOrders => StromMessage::PooledOrders(RequestPair::decode(buf)?)
        };
        Ok(StromProtocolMessage { message_type, message })
    }

    /// Returns the capability for the `ping` protocol.
    pub fn capability() -> Capability {
        Capability::new_static("strom", 1)
    }

    /// Returns the protocol for the `test` protocol.
    pub fn protocol() -> Protocol {
        Protocol::new(Self::capability(), 7)
    }
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl Encodable for StromProtocolMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }

    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
}

/// Represents messages that can be sent to multiple peers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolBroadcastMessage {
    pub message_type: StromMessageID,
    pub message:      StromBroadcastMessage
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl Encodable for ProtocolBroadcastMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }

    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
}

impl From<StromBroadcastMessage> for ProtocolBroadcastMessage {
    fn from(message: StromBroadcastMessage) -> Self {
        ProtocolBroadcastMessage { message_type: message.message_id(), message }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StromMessage {
    /// init
    Status(Status),

    /// Consensus
    PrePropose(PreProposal),
    Propose(Proposal),
    Commit(Commit),

    /// Propagation messages that broadcast new orders to all peers
    PropagatePooledOrders(Orders),

    // Order Request / Response pairs
    GetPooledOrders(RequestPair<GetPooledOrders>),
    PooledOrders(RequestPair<Orders>)
}

macro_rules! encodable_enum {
    ($enum_name:ident, $($var:ident),+) => {
        impl Encodable for $enum_name {
            fn encode(&self, out: &mut dyn BufMut) {
                match self {
                    $( $enum_name::$var(t) => t.encode(out), )*
                }
            }

            fn length(&self) -> usize {
                match self {
                    $( $enum_name::$var(t) => t.length(), )*
                }
            }
        }
    };
}

encodable_enum!(
    StromMessage,
    Status,
    PrePropose,
    Propose,
    Commit,
    PropagatePooledOrders,
    GetPooledOrders,
    PooledOrders
);

impl StromMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> StromMessageID {
        match self {
            StromMessage::Status(_) => StromMessageID::Status,
            StromMessage::PrePropose(_) => StromMessageID::PrePropose,
            StromMessage::Propose(_) => StromMessageID::Propose,
            StromMessage::Commit(_) => StromMessageID::Commit,

            StromMessage::PropagatePooledOrders(_) => StromMessageID::PropagatePooledOrders,
            StromMessage::GetPooledOrders(_) => StromMessageID::GetPooledOrders,
            StromMessage::PooledOrders(_) => StromMessageID::PooledOrders
        }
    }
}

/// Represents broadcast messages of [`StromMessage`] with the same object that
/// can be sent to multiple peers.
///
/// Messages that contain a list of hashes depend on the peer the message is
/// sent to. A peer should never receive a hash of an object (block,
/// transaction) it has already seen.
///
/// Note: This is only useful for outgoing messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StromBroadcastMessage {
    // Consensus Broadcast
    PrePropose(Arc<PreProposal>),
    Propose(Arc<Proposal>),
    Commit(Arc<Commit>),
    // Order Broadcast
    PropagatePooledOrders(Arc<Vec<PooledOrder>>),
    GetPooledOrders(Arc<RequestPair<Orders>>)
}

// === impl StromBroadcastMessage ===
impl StromBroadcastMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> StromMessageID {
        match self {
            StromBroadcastMessage::PrePropose(_) => StromMessageID::PrePropose,
            StromBroadcastMessage::Propose(_) => StromMessageID::Propose,
            StromBroadcastMessage::Commit(_) => StromMessageID::Commit,
            StromBroadcastMessage::PropagatePooledOrders(_) => {
                StromMessageID::PropagatePooledOrders
            }
            StromBroadcastMessage::GetPooledOrders(_) => StromMessageID::GetPooledOrders
        }
    }
}

encodable_enum!(
    StromBroadcastMessage,
    PrePropose,
    Propose,
    Commit,
    PropagatePooledOrders,
    GetPooledOrders
);

/// Represents message IDs for eth protocol messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StromMessageID {
    Status          = 0,
    /// Consensus
    PrePropose      = 1,
    Propose         = 2,
    Commit          = 3,
    /// Propagation messages that broadcast new orders to all peers
    PropagatePooledOrders = 4,

    /// Order Request / Response pairs
    GetPooledOrders = 5,
    PooledOrders    = 6
}

impl Encodable for StromMessageID {
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(*self as u8);
    }

    fn length(&self) -> usize {
        1
    }
}

impl Decodable for StromMessageID {
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let id = buf.first().ok_or(alloy_rlp::Error::InputTooShort)?;
        let id = match id {
            0 => StromMessageID::Status,
            1 => StromMessageID::PrePropose,
            2 => StromMessageID::Propose,
            3 => StromMessageID::Commit,
            4 => StromMessageID::PropagatePooledOrders,
            5 => StromMessageID::GetPooledOrders,
            6 => StromMessageID::PooledOrders,
            _ => return Err(alloy_rlp::Error::Custom("Invalid message ID"))
        };
        buf.advance(1);
        Ok(id)
    }
}

impl TryFrom<usize> for StromMessageID {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StromMessageID::Status),
            1 => Ok(StromMessageID::PrePropose),
            2 => Ok(StromMessageID::Propose),
            3 => Ok(StromMessageID::Commit),
            4 => Ok(StromMessageID::PropagatePooledOrders),
            5 => Ok(StromMessageID::GetPooledOrders),
            6 => Ok(StromMessageID::PooledOrders),
            _ => Err("Invalid message ID")
        }
    }
}

/// This is used for all request-response style `eth` protocol messages.
/// This can represent either a request or a response, since both include a
/// message payload and request id.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestPair<O> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: O
}

/// Allows messages with request ids to be serialized into RLP bytes.
impl<O> Encodable for RequestPair<O>
where
    O: Encodable
{
    fn encode(&self, out: &mut dyn alloy_rlp::BufMut) {
        let header = Header {
            list:           true,
            payload_length: self.request_id.length() + self.message.length()
        };

        header.encode(out);
        self.request_id.encode(out);
        self.message.encode(out);
    }

    fn length(&self) -> usize {
        let mut length = 0;
        length += self.request_id.length();
        length += self.message.length();
        length += length_of_length(length);
        length
    }
}

/// Allows messages with request ids to be deserialized into RLP bytes.
impl<O> Decodable for RequestPair<O>
where
    O: Decodable
{
    fn decode(buf: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let _header = Header::decode(buf)?;
        Ok(Self { request_id: u64::decode(buf)?, message: O::decode(buf)? })
    }
}

/*
/// Protocol related request messages that expect a response
#[derive(Debug)]
#[allow(clippy::enum_variant_names, missing_docs)]
pub enum PeerRequest {
    GetAllOrders { request: GetOrders, response: oneshot::Sender<RequestResult<Orders>> }
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
            Self::GetAllOrders { response, .. } => response.send(Err(err)).ok()
        };
    }

    /// Returns the [`EthMessage`] for this type
    pub fn create_request_message(&self, request_id: u64) -> StromMessage {
        match self {
            PeerRequest::GetAllOrders { request, .. } => {
                StromMessage::GetOrders(RequestPair { request_id, message: request.clone() })
            }
        }
    }

    /// Consumes the type and returns the inner [`GetPooledTransactions`]
    /// variant.
    pub fn into_get_all_orders(self) -> Option<GetOrders> {
        match self {
            PeerRequest::GetPooledTransactions { request, .. } => Some(request),
            _ => None
        }
    }
}

/// Corresponding variant for [`PeerRequest`].
#[derive(Debug)]
pub enum PeerResponse {
    BlockHeaders { response: oneshot::Receiver<RequestResult<Orders>> }
}

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
            PeerResponse::BlockHeaders { response } => {
                poll_request!(response, BlockHeaders, cx)
            }
            PeerResponse::BlockBodies { response } => {
                poll_request!(response, BlockBodies, cx)
            }
            PeerResponse::PooledTransactions { response } => {
                poll_request!(response, PooledTransactions, cx)
            }
            PeerResponse::NodeData { response } => {
                poll_request!(response, NodeData, cx)
            }
            PeerResponse::Receipts { response } => {
                poll_request!(response, Receipts, cx)
            }
        };
        Poll::Ready(res)
    }
} */
