use std::fmt::{Debug, Display};

use alloy_rlp::{RlpDecodable, RlpEncodable};
use reth_codecs::derive_arbitrary;
use reth_primitives::{Chain, ChainSpec, Head, NamedChain};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{version::StromVersion, StatusBuilder};

/// The status message is used in the eth protocol handshake to ensure that
/// peers are on the same network and are following the same fork.
///
/// When performing a handshake, the total difficulty is not guaranteed to
/// correspond to the block hash. This information should be treated as
/// untrusted.
#[derive_arbitrary(rlp)]
#[derive(Copy, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Status {
    /// The current protocol version.
    pub version: u8,

    /// The chain id, as introduced in
    /// [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    pub chain: Chain
}

impl Status {
    /// Helper for returning a builder for the status message.
    pub fn builder() -> StatusBuilder {
        Default::default()
    }

    /// Create a [`StatusBuilder`] from the given [`ChainSpec`] and head block.
    pub fn spec_builder(spec: &ChainSpec, _head: &Head) -> StatusBuilder {
        Self::builder().chain(spec.chain)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status {{ version: {}, chain: {}}}", self.version, self.chain,)
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "Status {{\n\tversion: {:?},\n\tchain: {:?}}}", self.version, self.chain,)
        } else {
            write!(f, "Status {{ version: {:?}, chain: {:?}}}", self.version, self.chain,)
        }
    }
}

// <https://etherscan.io/block/0>
impl Default for Status {
    fn default() -> Self {
        Status { version: StromVersion::Strom0 as u8, chain: Chain::Named(NamedChain::Mainnet) }
    }
}
