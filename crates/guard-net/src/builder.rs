//! Builder structs for messages.

use std::time::{SystemTime, UNIX_EPOCH};

use reth_primitives::{alloy_primitives::FixedBytes, keccak256, BufMut, BytesMut, Chain, PeerId};
use secp256k1::{Message, SecretKey};

use crate::Status;

/// Builder for [`Status`] messages.
#[derive(Debug)]
pub struct StatusBuilder {
    /// The current protocol version.
    version: u8,

    /// The chain id, as introduced in
    /// [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    chain:     Chain,
    /// The peer that a node is trying to establish a connection with
    peer:      PeerId,
    /// The current timestamp. Used to make sure that the status message will
    /// expire
    timestamp: u128
}

impl StatusBuilder {
    pub fn new(peer: PeerId) -> StatusBuilder {
        Self {
            peer,
            version: Default::default(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u128,
            chain: Chain::default()
        }
    }

    /// Consumes the type and creates the actual [`Status`] message, Signing the
    /// payload
    pub fn build(self, key: SecretKey) -> Status {
        let mut buf = BytesMut::with_capacity(113);
        buf.put_u8(self.version);
        buf.put_u64(u64::from(self.chain));
        buf.put(self.peer.0.as_ref());
        buf.put_u128(self.timestamp);

        let payload = keccak256(buf);
        let sig = reth_primitives::sign_message(FixedBytes(key.secret_bytes()), payload).unwrap();

        Status {
            timestamp: self.timestamp,
            peer:      self.peer,
            chain:     self.chain,
            version:   self.version,
            signature: guard_types::primitive::Signature(sig)
        }
    }

    /// Sets the protocol version.
    pub fn version(mut self, version: u8) -> Self {
        self.version = version;
        self
    }

    /// Sets the chain id.
    pub fn chain(mut self, chain: Chain) -> Self {
        self.chain = chain;
        self
    }
}
