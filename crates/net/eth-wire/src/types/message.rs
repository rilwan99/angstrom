#![allow(missing_docs)]
use std::{fmt::Debug, net::SocketAddr, sync::Arc};

use ethers_core::types::transaction::eip712::TypedData;
use reth_primitives::bytes::{Buf, BufMut};
use reth_rlp::{length_of_length, Decodable, Encodable, Header};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use shared::{Bundle, BundleSignature, SealedBundle, TeeAddress};

use super::Status;
use crate::{errors::EthStreamError, EthVersion};

/// An `eth` protocol message, containing a message ID and payload.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProtocolMessage {
    pub message_type: EthMessageID,
    pub message: EthMessage,
}

impl ProtocolMessage {
    /// Create a new ProtocolMessage from a message type and message rlp bytes.
    pub fn decode_message(version: EthVersion, buf: &mut &[u8]) -> Result<Self, EthStreamError> {
        let message_type = EthMessageID::decode(buf)?;

        let message = match message_type {
            // EthMessageID::
            // EthMessageID::NodeData => {
            //     if version >= EthVersion::Eth67 {
            //         return Err(EthStreamError::EthInvalidMessageError(
            //             version,
            //             EthMessageID::GetNodeData
            //         ))
            //     }
            //     let request_pair = RequestPair::<NodeData>::decode(buf)?;
            //     EthMessage::NodeData(request_pair)
            // }
            // EthMessageID::GetReceipts => {
            //     let request_pair = RequestPair::<GetReceipts>::decode(buf)?;
            //     EthMessage::GetReceipts(request_pair)
            // }
            // EthMessageID::Receipts => {
            //     let request_pair = RequestPair::<Receipts>::decode(buf)?;
            //     EthMessage::Receipts(request_pair)
            // }
        };
        Ok(ProtocolMessage { message_type, message })
    }
}

/// Encodes the protocol message into bytes.
/// The message type is encoded as a single byte and prepended to the message.
impl Encodable for ProtocolMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        self.message_type.encode(out);
        self.message.encode(out);
    }

    fn length(&self) -> usize {
        self.message_type.length() + self.message.length()
    }
}

impl From<EthMessage> for ProtocolMessage {
    fn from(message: EthMessage) -> Self {
        ProtocolMessage { message_type: message.message_id(), message }
    }
}

/// Represents messages that can be sent to multiple peers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolBroadcastMessage {
    pub message_type: EthMessageID,
    pub message: EthBroadcastMessage,
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

impl From<EthBroadcastMessage> for ProtocolBroadcastMessage {
    fn from(message: EthBroadcastMessage) -> Self {
        ProtocolBroadcastMessage { message_type: message.message_id(), message }
    }
}

/// Represents a message in the eth wire protocol, versions 66, 67 and 68.
///
/// The ethereum wire protocol is a set of messages that are broadcast to the
/// network in two styles:
///  * A request message sent by a peer (such as [`GetPooledTransactions`]), and
///    an associated
///  response message (such as [`PooledTransactions`]).
///  * A message that is broadcast to the network, without a corresponding
///    request.
///
/// The newer `eth/66` is an efficiency upgrade on top of `eth/65`, introducing
/// a request id to correlate request-response message pairs. This allows for
/// request multiplexing.
///
/// The `eth/67` is based on `eth/66` but only removes two messages,
/// [`GetNodeData`] and [``NodeData].
///
/// The `eth/68` changes only NewPooledTransactionHashes to include `types` and
/// `sized`. For it, NewPooledTransactionHashes is renamed as
/// [`NewPooledTransactionHashes66`] and [`NewPooledTransactionHashes68`] is
/// defined.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EthMessage {
    // init
    Status(Status),
    GetTeeModule(RequestPair<TeeAddress>),
    // broadcast
    PropagateTransactions(Vec<Arc<TypedData>>),
    PropagateSealedBundle(Arc<SealedBundle>),
    PropagateSignatureRequest(Arc<Bundle>),
    PropagateBundleSignature(Arc<BundleSignature>),
}

impl EthMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthMessage::Status(_) => EthMessageID::Status,
            EthMessage::GetTeeModule(_) => EthMessageID::GetTeeModule,
            EthMessage::PropagateTransactions(_) => EthMessageID::PropagateTransactions,
            EthMessage::PropagateSealedBundle(_) => EthMessageID::PropagateSealedBundle,
            EthMessage::PropagateBundleSignature(_) => EthMessageID::PropagateBundleSignature,
            EthMessage::PropagateSignatureRequest(_) => EthMessageID::PropagateSignatureRequest,
        }
    }
}

impl Encodable for EthMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthMessage::Status(status) => status.encode(out),
            EthMessage::GetTeeModule(module) => module.encode(out),
            EthMessage::PropagateTransactions(txes) => txes.encode(out),
            EthMessage::PropagateSealedBundle(bundle) => bundle.encode(out),
            EthMessage::PropagateBundleSignature(sig) => sig.encode(out),
            EthMessage::PropagateSignatureRequest(sig_req) => sig_req.encode(out),
        }
    }

    fn length(&self) -> usize {
        match self {
            EthMessage::Status(status) => status.length(),
            EthMessage::GetTeeModule(module) => module.length(),
            EthMessage::PropagateTransactions(txes) => txes.length(),
            EthMessage::PropagateSealedBundle(bundle) => bundle.length(),
            EthMessage::PropagateBundleSignature(sig) => sig.length(),
            EthMessage::PropagateSignatureRequest(sig_req) => sig_req.length(),
        }
    }
}

/// Represents broadcast messages of [`EthMessage`] with the same object that
/// can be sent to multiple peers.
///
/// Messages that contain a list of hashes depend on the peer the message is
/// sent to. A peer should never receive a hash of an object (block,
/// transaction) it has already seen.
///
/// Note: This is only useful for outgoing messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EthBroadcastMessage {
    PropagateTransactions(Vec<Arc<TypedData>>),
    PropagateSealedBundle(Arc<SealedBundle>),
    PropagateSignatureRequest(Arc<Bundle>),
    PropagateBundleSignature(Arc<BundleSignature>),
}

// === impl EthBroadcastMessage ===

impl EthBroadcastMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthBroadcastMessage::PropagateTransactions(_) => EthMessageID::PropagateTransactions,
            EthBroadcastMessage::PropagateSealedBundle(_) => EthMessageID::PropagateSealedBundle,
            EthBroadcastMessage::PropagateBundleSignature(_) => {
                EthMessageID::PropagateBundleSignature
            }

            EthBroadcastMessage::PropagateSignatureRequest(_) => {
                EthMessageID::PropagateSignatureRequest
            }
        }
    }
}

impl Encodable for EthBroadcastMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthBroadcastMessage::PropagateSignatureRequest(sig) => sig.encode(out),
            EthBroadcastMessage::PropagateTransactions(tx) => tx.encode(out),
            EthBroadcastMessage::PropagateSealedBundle(bundle) => bundle.encode(out),
            EthBroadcastMessage::PropagateBundleSignature(sig) => sig.encode(out),
        }
    }

    fn length(&self) -> usize {
        match self {
            EthBroadcastMessage::PropagateSignatureRequest(sig) => sig.length(),
            EthBroadcastMessage::PropagateTransactions(tx) => tx.length(),
            EthBroadcastMessage::PropagateSealedBundle(bundle) => bundle.length(),
            EthBroadcastMessage::PropagateBundleSignature(sig) => sig.length(),
        }
    }
}

/// Represents message IDs for eth protocol messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EthMessageID {
    GetTeeModule = 0x00,
    PropagateTransactions = 0x01,
    PropagateSealedBundle = 0x02,
    PropagateSignatureRequest = 0x03,
    PropagateBundleSignature = 0x04,
    Status = 0x05,
}

impl Encodable for EthMessageID {
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(*self as u8);
    }

    fn length(&self) -> usize {
        1
    }
}

impl Decodable for EthMessageID {
    fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let id = buf.first().ok_or(reth_rlp::DecodeError::InputTooShort)?;
        let id = match id {
            0x00 => EthMessageID::GetTeeModule,
            0x01 => EthMessageID::PropagateTransactions,
            0x02 => EthMessageID::PropagateSealedBundle,
            0x03 => EthMessageID::PropagateSignatureRequest,
            0x04 => EthMessageID::PropagateBundleSignature,
            0x05 => EthMessageID::Status,
            _ => return Err(reth_rlp::DecodeError::Custom("Invalid message ID")),
        };
        buf.advance(1);
        Ok(id)
    }
}

impl TryFrom<usize> for EthMessageID {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(EthMessageID::GetTeeModule),
            0x00 => Ok(EthMessageID::GetTeeModule),
            0x01 => Ok(EthMessageID::PropagateTransactions),
            0x02 => Ok(EthMessageID::PropagateSealedBundle),
            0x03 => Ok(EthMessageID::PropagateSignatureRequest),
            0x04 => Ok(EthMessageID::PropagateBundleSignature),
            0x05 => Ok(EthMessageID::Status),
            _ => Err("Invalid message ID"),
        }
    }
}

/// This is used for all request-response style `eth` protocol messages.
/// This can represent either a request or a response, since both include a
/// message payload and request id.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestPair<T> {
    /// id for the contained request or response message
    pub request_id: u64,

    /// the request or response message payload
    pub message: T,
}

/// Allows messages with request ids to be serialized into RLP bytes.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable,
{
    fn encode(&self, out: &mut dyn reth_rlp::BufMut) {
        let header =
            Header { list: true, payload_length: self.request_id.length() + self.message.length() };

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
impl<T> Decodable for RequestPair<T>
where
    T: Decodable,
{
    fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let _header = Header::decode(buf)?;
        Ok(Self { request_id: u64::decode(buf)?, message: T::decode(buf)? })
    }
}

#[cfg(test)]
mod test {
    use hex_literal::hex;
    use reth_rlp::{Decodable, Encodable};

    use crate::{
        errors::EthStreamError, types::message::RequestPair, EthMessage, EthMessageID, GetNodeData,
        NodeData, ProtocolMessage,
    };

    fn encode<T: Encodable>(value: T) -> Vec<u8> {
        let mut buf = vec![];
        value.encode(&mut buf);
        buf
    }

    #[test]
    fn test_removed_message_at_eth67() {
        let get_node_data =
            EthMessage::GetNodeData(RequestPair { request_id: 1337, message: GetNodeData(vec![]) });
        let buf = encode(ProtocolMessage {
            message_type: EthMessageID::GetNodeData,
            message: get_node_data,
        });
        let msg = ProtocolMessage::decode_message(crate::EthVersion::Eth67, &mut &buf[..]);
        assert!(matches!(msg, Err(EthStreamError::EthInvalidMessageError(..))));

        let node_data =
            EthMessage::NodeData(RequestPair { request_id: 1337, message: NodeData(vec![]) });
        let buf =
            encode(ProtocolMessage { message_type: EthMessageID::NodeData, message: node_data });
        let msg = ProtocolMessage::decode_message(crate::EthVersion::Eth67, &mut &buf[..]);
        assert!(matches!(msg, Err(EthStreamError::EthInvalidMessageError(..))));
    }

    #[test]
    fn request_pair_encode() {
        let request_pair = RequestPair { request_id: 1337, message: vec![5u8] };

        // c5: start of list (c0) + len(full_list) (length is <55 bytes)
        // 82: 0x80 + len(1337)
        // 05 39: 1337 (request_id)
        // === full_list ===
        // c1: start of list (c0) + len(list) (length is <55 bytes)
        // 05: 5 (message)
        let expected = hex!("c5820539c105");
        let got = encode(request_pair);
        assert_eq!(expected[..], got, "expected: {expected:X?}, got: {got:X?}",);
    }

    #[test]
    fn request_pair_decode() {
        let raw_pair = &hex!("c5820539c105")[..];

        let expected = RequestPair { request_id: 1337, message: vec![5u8] };

        let got = RequestPair::<Vec<u8>>::decode(&mut &*raw_pair).unwrap();
        assert_eq!(expected.length(), raw_pair.len());
        assert_eq!(expected, got);
    }
}
