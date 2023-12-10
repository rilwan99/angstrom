use std::fmt::{Debug, Display};

use alloy_rlp::{RlpDecodable, RlpEncodable};
use reth_codecs::derive_arbitrary;
use reth_primitives::{Chain, ChainSpec, Head, NamedChain};
use reth_rpc_types::PeerId;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{version::StromVersion, StatusBuilder};

/// The status message is used in the strom protocol to ensure that the
/// connecting peer is using the same protocol version and is on the same chain.
/// More crucially, it is used to verify that the connecting peer is a valid
/// staker with sufficient balance to be a validator.

#[derive(Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Status {
    /// The current protocol version.
    pub version: u8,

    /// The chain id, as introduced in
    /// [EIP155](https://eips.ethereum.org/EIPS/eip-155#list-of-chain-ids).
    pub chain:     Chain,
    /// The peer that a node is trying to establish a connection with
    pub peer:      PeerId,
    /// The current timestamp. Used to make sure that the status message will
    /// expire
    pub timestamp: u128,
    /// the signature of this message, sign(keccack(version || chain || peer ||
    /// timestamp)),
    pub signature: guard_types::primitive::Signature
}

impl Status {
    /// Helper for returning a builder for the status message.
    pub fn builder(peer_id: PeerId) -> StatusBuilder {
        StatusBuilder::new(peer_id)
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
