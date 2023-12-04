//! Builder structs for messages.

use reth_primitives::{Chain, ForkId, B256, U256};

use crate::Status;

/// Builder for [`Status`] messages.
#[derive(Debug, Default)]
pub struct StatusBuilder {
    status: Status
}

impl StatusBuilder {
    /// Consumes the type and creates the actual [`Status`] message.
    pub fn build(self) -> Status {
        self.status
    }

    /// Sets the protocol version.
    pub fn version(mut self, version: u8) -> Self {
        self.status.version = version;
        self
    }

    /// Sets the chain id.
    pub fn chain(mut self, chain: Chain) -> Self {
        self.status.chain = chain;
        self
    }
}
