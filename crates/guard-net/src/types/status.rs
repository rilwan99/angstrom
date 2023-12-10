use std::fmt::{Debug, Display};

use alloy_rlp::{RlpDecodable, RlpEncodable};
use guard_types::primitive::Signature;
use reth_codecs::derive_arbitrary;
use reth_primitives::{
    alloy_primitives::FixedBytes, keccak256, Address, BufMut, BytesMut, Chain, ChainSpec, Head,
    NamedChain
};
use reth_rpc_types::PeerId;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    ffi::CPtr,
    Message, SECP256K1
};
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
    pub signature: Signature
}

impl Status {
    /// Helper for returning a builder for the status message.
    pub fn builder(peer_id: PeerId) -> StatusBuilder {
        StatusBuilder::new(peer_id)
    }

    /// returns true if the signature is valid
    pub fn verify(self) -> Result<PeerId, secp256k1::Error> {
        let mut buf = BytesMut::with_capacity(113);
        buf.put_u8(self.version);
        buf.put_u64(u64::from(self.chain));
        buf.put(self.peer.0.as_ref());
        buf.put_u128(self.timestamp);

        let message = keccak256(buf);
        self.signature.recover_signer_full_public_key(message)
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
