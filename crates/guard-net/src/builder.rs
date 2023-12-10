//! Builder structs for messages.

use std::time::{SystemTime, UNIX_EPOCH};

use reth_primitives::{alloy_primitives::FixedBytes, keccak256, BufMut, BytesMut, Chain, PeerId};
use secp256k1::{Message, SecretKey};

use crate::{types::status::StatusState, Status};

/// Builder for [`Status`] messages.
#[derive(Debug)]
pub struct StatusBuilder {
    state: StatusState
}

impl StatusBuilder {
    pub fn new(peer: PeerId) -> StatusBuilder {
        Self { state: StatusState::new(peer) }
    }

    /// Consumes the type and creates the actual [`Status`] message, Signing the
    /// payload
    pub fn build(mut self, key: SecretKey) -> Status {
        // set state timestamp to now;
        self.state.timestamp_now();

        let message = self.state.to_message();
        let sig = reth_primitives::sign_message(FixedBytes(key.secret_bytes()), message).unwrap();

        Status { state: self.state, signature: guard_types::primitive::Signature(sig) }
    }

    /// Sets the protocol version.
    pub fn version(mut self, version: u8) -> Self {
        self.state.version = version;
        self
    }

    /// Sets the chain id.
    pub fn chain(mut self, chain: Chain) -> Self {
        self.state.chain = chain;
        self
    }
}

impl From<StatusState> for StatusBuilder {
    fn from(value: StatusState) -> Self {
        Self { state: value }
    }
}
