#![allow(missing_docs)]
use std::{fmt::Debug, sync::Arc};

use ethers_core::k256::elliptic_curve::rand_core::le;
use guard_types::{
    consensus::{
        Block, Bundle23Votes, BundleVote, LeaderProposal, SignedLeaderProposal, Valid23Bundle
    },
    on_chain::{SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement}
};
use reth_primitives::bytes::{Buf, BufMut};
use reth_rlp::{length_of_length, Decodable, Encodable, Header};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Status;
use crate::{errors::EthStreamError, EthVersion};

/// An `eth` protocol message, containing a message ID and payload.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProtocolMessage {
    pub message_type: EthMessageID,
    pub message:      EthMessage
}

impl ProtocolMessage {
    /// Create a new ProtocolMessage from a message type and message rlp bytes.
    pub fn decode_message(_version: EthVersion, buf: &mut &[u8]) -> Result<Self, EthStreamError> {
        let message_type = EthMessageID::decode(buf)?;

        let message = match message_type {
            EthMessageID::Status => EthMessage::Status(Status::decode(buf)?),
            EthMessageID::PropagateUserTransactions => {
                EthMessage::PropagateUserTransactions(Vec::<SimmedUserSettlement>::decode(buf)?)
            }
            EthMessageID::PropagateBundle => {
                EthMessage::PropagateBundle(SimmedBundle::decode(buf)?)
            }
            EthMessageID::PropagateSearcherTransactions => {
                EthMessage::PropagateSearcherTransactions(Vec::<SimmedLvrSettlement>::decode(buf)?)
            }
            EthMessageID::NewBlock => EthMessage::NewBlock(Block::decode(buf)?),
            EthMessageID::BundleVote => EthMessage::BundleVote(BundleVote::decode(buf)?),
            EthMessageID::Bundle23Vote => EthMessage::Bundle23Vote(Bundle23Votes::decode(buf)?),
            EthMessageID::LeaderProposal => {
                EthMessage::LeaderProposal(LeaderProposal::decode(buf)?)
            }
            EthMessageID::SignedLeaderProposal => {
                EthMessage::SignedLeaderProposal(SignedLeaderProposal::decode(buf)?)
            }
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
    pub message:      EthBroadcastMessage
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
    // broadcast
    PropagateBundle(SimmedBundle),
    PropagateUserTransactions(Vec<SimmedUserSettlement>),
    PropagateSearcherTransactions(Vec<SimmedLvrSettlement>),
    BundleVote(BundleVote),
    Bundle23Vote(Bundle23Votes),
    LeaderProposal(LeaderProposal),
    SignedLeaderProposal(SignedLeaderProposal),
    NewBlock(Block)
}

impl EthMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthMessage::Status(_) => EthMessageID::Status,
            EthMessage::SignedLeaderProposal(_) => EthMessageID::SignedLeaderProposal,
            EthMessage::LeaderProposal(_) => EthMessageID::LeaderProposal,
            EthMessage::Bundle23Vote(_) => EthMessageID::Bundle23Vote,
            EthMessage::BundleVote(_) => EthMessageID::BundleVote,
            EthMessage::NewBlock(_) => EthMessageID::NewBlock,
            EthMessage::PropagateBundle(_) => EthMessageID::PropagateBundle,
            EthMessage::PropagateUserTransactions(_) => EthMessageID::PropagateUserTransactions,
            EthMessage::PropagateSearcherTransactions(_) => {
                EthMessageID::PropagateSearcherTransactions
            }
        }
    }
}

impl Encodable for EthMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthMessage::Status(status) => status.encode(out),
            EthMessage::PropagateUserTransactions(txes) => txes.encode(out),
            EthMessage::PropagateBundle(bundle) => bundle.encode(out),
            EthMessage::PropagateSearcherTransactions(txes) => txes.encode(out),
            EthMessage::NewBlock(block) => block.encode(out),
            EthMessage::PropagateBundle(bundle) => bundle.encode(out),
            EthMessage::BundleVote(vote) => vote.encode(out),
            EthMessage::Bundle23Vote(vote) => vote.encode(out),
            EthMessage::LeaderProposal(prop) => prop.encode(out),
            EthMessage::SignedLeaderProposal(prop) => prop.encode(out)
        }
    }

    fn length(&self) -> usize {
        match self {
            EthMessage::Status(status) => status.length(),
            EthMessage::PropagateUserTransactions(txes) => txes.length(),
            EthMessage::PropagateBundle(bundle) => bundle.length(),
            EthMessage::PropagateSearcherTransactions(txes) => txes.length(),
            EthMessage::NewBlock(block) => block.length(),
            EthMessage::PropagateBundle(bundle) => bundle.length(),
            EthMessage::BundleVote(vote) => vote.length(),
            EthMessage::Bundle23Vote(vote) => vote.length(),
            EthMessage::LeaderProposal(prop) => prop.length(),
            EthMessage::SignedLeaderProposal(prop) => prop.length()
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
    PropagateBundle(Arc<SimmedBundle>),
    PropagateUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    PropagateSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>),
    BundleVote(Arc<BundleVote>),
    Bundle23Vote(Arc<Bundle23Votes>),
    LeaderProposal(Arc<LeaderProposal>),
    SignedLeaderProposal(Arc<SignedLeaderProposal>),
    NewBlock(Arc<Block>)
}

// === impl EthBroadcastMessage ===

impl EthBroadcastMessage {
    /// Returns the message's ID.
    pub fn message_id(&self) -> EthMessageID {
        match self {
            EthBroadcastMessage::SignedLeaderProposal(_) => EthMessageID::SignedLeaderProposal,
            EthBroadcastMessage::LeaderProposal(_) => EthMessageID::LeaderProposal,
            EthBroadcastMessage::Bundle23Vote(_) => EthMessageID::Bundle23Vote,
            EthBroadcastMessage::BundleVote(_) => EthMessageID::BundleVote,
            EthBroadcastMessage::NewBlock(_) => EthMessageID::NewBlock,
            EthBroadcastMessage::PropagateBundle(_) => EthMessageID::PropagateBundle,
            EthBroadcastMessage::PropagateUserTransactions(_) => {
                EthMessageID::PropagateUserTransactions
            }
            EthBroadcastMessage::PropagateSearcherTransactions(_) => {
                EthMessageID::PropagateSearcherTransactions
            }
        }
    }
}

impl Encodable for EthBroadcastMessage {
    fn encode(&self, out: &mut dyn BufMut) {
        match self {
            EthBroadcastMessage::PropagateUserTransactions(txes) => txes.encode(out),
            EthBroadcastMessage::PropagateBundle(bundle) => bundle.encode(out),
            EthBroadcastMessage::PropagateSearcherTransactions(txes) => txes.encode(out),
            EthBroadcastMessage::NewBlock(block) => block.encode(out),
            EthBroadcastMessage::PropagateBundle(bundle) => bundle.encode(out),
            EthBroadcastMessage::BundleVote(vote) => vote.encode(out),
            EthBroadcastMessage::Bundle23Vote(vote) => vote.encode(out),
            EthBroadcastMessage::LeaderProposal(prop) => prop.encode(out),
            EthBroadcastMessage::SignedLeaderProposal(prop) => prop.encode(out)
        }
    }

    fn length(&self) -> usize {
        match self {
            EthBroadcastMessage::PropagateUserTransactions(txes) => txes.length(),
            EthBroadcastMessage::PropagateBundle(bundle) => bundle.length(),
            EthBroadcastMessage::PropagateSearcherTransactions(txes) => txes.length(),
            EthBroadcastMessage::NewBlock(block) => block.length(),
            EthBroadcastMessage::PropagateBundle(bundle) => bundle.length(),
            EthBroadcastMessage::BundleVote(vote) => vote.length(),
            EthBroadcastMessage::Bundle23Vote(vote) => vote.length(),
            EthBroadcastMessage::LeaderProposal(prop) => prop.length(),
            EthBroadcastMessage::SignedLeaderProposal(prop) => prop.length()
        }
    }
}

/// Represents message IDs for eth protocol messages.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EthMessageID {
    Status               = 0,
    BundleVote           = 1,
    PropagateBundle      = 2,
    Bundle23Vote         = 3,
    LeaderProposal       = 4,
    SignedLeaderProposal = 5,
    NewBlock             = 6,
    PropagateUserTransactions = 7,
    PropagateSearcherTransactions = 8
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
            0 => EthMessageID::Status,
            1 => EthMessageID::BundleVote,
            2 => EthMessageID::PropagateBundle,
            3 => EthMessageID::Bundle23Vote,
            4 => EthMessageID::LeaderProposal,
            5 => EthMessageID::SignedLeaderProposal,
            6 => EthMessageID::NewBlock,
            7 => EthMessageID::PropagateUserTransactions,
            8 => EthMessageID::PropagateSearcherTransactions,
            _ => return Err(reth_rlp::DecodeError::Custom("Invalid message ID"))
        };
        buf.advance(1);
        Ok(id)
    }
}

impl TryFrom<usize> for EthMessageID {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EthMessageID::Status),
            1 => Ok(EthMessageID::BundleVote),
            2 => Ok(EthMessageID::PropagateBundle),
            3 => Ok(EthMessageID::Bundle23Vote),
            4 => Ok(EthMessageID::LeaderProposal),
            5 => Ok(EthMessageID::SignedLeaderProposal),
            6 => Ok(EthMessageID::NewBlock),
            7 => Ok(EthMessageID::PropagateUserTransactions),
            8 => Ok(EthMessageID::PropagateSearcherTransactions),
            _ => Err("Invalid message ID")
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
    pub message: T
}

/// Allows messages with request ids to be serialized into RLP bytes.
impl<T> Encodable for RequestPair<T>
where
    T: Encodable
{
    fn encode(&self, out: &mut dyn reth_rlp::BufMut) {
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
impl<T> Decodable for RequestPair<T>
where
    T: Decodable
{
    fn decode(buf: &mut &[u8]) -> Result<Self, reth_rlp::DecodeError> {
        let _header = Header::decode(buf)?;
        Ok(Self { request_id: u64::decode(buf)?, message: T::decode(buf)? })
    }
}

#[cfg(test)]
mod test {
    // use hex_literal::hex;
    // use reth_rlp::{Decodable, Encodable};
    //
    // use crate::{
    //     errors::EthStreamError, types::message::RequestPair, EthMessage,
    // EthMessageID,     ProtocolMessage
    // };
    //
    // fn encode<T: Encodable>(value: T) -> Vec<u8> {
    //     let mut buf = vec![];
    //     value.encode(&mut buf);
    //     buf
    // }
    //
    // #[test]
    // fn test_removed_message_at_eth67() {
    //     let get_node_data = EthMessage::GetNodeData(RequestPair {
    //         request_id: 1337,
    //         message:    GetNodeData(vec![])
    //     });
    //     let buf = encode(ProtocolMessage {
    //         message_type: EthMessageID::GetNodeData,
    //         message:      get_node_data
    //     });
    //     let msg = ProtocolMessage::decode_message(crate::EthVersion::Eth67,
    // &mut &buf[..]);     assert!(matches!(msg,
    // Err(EthStreamError::EthInvalidMessageError(..))));
    //
    //     let node_data =
    //         EthMessage::NodeData(RequestPair { request_id: 1337, message:
    // NodeData(vec![]) });     let buf = encode(ProtocolMessage {
    //         message_type: EthMessageID::NodeData,
    //         message:      node_data
    //     });
    //     let msg = ProtocolMessage::decode_message(crate::EthVersion::Eth67,
    // &mut &buf[..]);     assert!(matches!(msg,
    // Err(EthStreamError::EthInvalidMessageError(..)))); }
    //
    // #[test]
    // fn request_pair_encode() {
    //     let request_pair = RequestPair { request_id: 1337, message: vec![5u8]
    // };
    //
    //     // c5: start of list (c0) + len(full_list) (length is <55 bytes)
    //     // 82: 0x80 + len(1337)
    //     // 05 39: 1337 (request_id)
    //     // === full_list ===
    //     // c1: start of list (c0) + len(list) (length is <55 bytes)
    //     // 05: 5 (message)
    //     let expected = hex!("c5820539c105");
    //     let got = encode(request_pair);
    //     assert_eq!(expected[..], got, "expected: {expected:X?}, got:
    // {got:X?}",); }
    //
    // #[test]
    // fn request_pair_decode() {
    //     let raw_pair = &hex!("c5820539c105")[..];
    //
    //     let expected = RequestPair { request_id: 1337, message: vec![5u8] };
    //
    //     let got = RequestPair::<Vec<u8>>::decode(&mut &*raw_pair).unwrap();
    //     assert_eq!(expected.length(), raw_pair.len());
    //     assert_eq!(expected, got);
    // }
}
