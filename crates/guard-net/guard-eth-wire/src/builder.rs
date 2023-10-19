//! Builder structs for [`Status`](crate::types::Status) and
//! [`HelloMessage`](crate::HelloMessage) messages.

use guard_discv4::DEFAULT_DISCOVERY_PORT;
use reth_primitives::{Chain, ForkId, PeerId, Signature, H256, U256};
use secp256k1::ecdsa::RecoverableSignature;

use crate::{
    capability::Capability, hello::HelloMessage, p2pstream::ProtocolVersion, EthVersion, Status
};

/// Builder for [`Status`](crate::types::Status) messages.
///
/// # Example
/// ```
/// use reth_eth_wire::EthVersion;
/// use reth_primitives::{Chain, U256, H256, MAINNET_GENESIS, MAINNET, Hardfork};
/// use reth_eth_wire::types::Status;
///
/// // this is just an example status message!
/// let status = Status::builder()
///     .version(EthVersion::Eth66.into())
///     .chain(Chain::Named(ethers_core::types::Chain::Mainnet))
///     .total_difficulty(U256::from(100))
///     .blockhash(H256::from(MAINNET_GENESIS))
///     .genesis(H256::from(MAINNET_GENESIS))
///     .forkid(Hardfork::Paris.fork_id(&MAINNET).unwrap())
///     .build();
///
/// assert_eq!(
///     status,
///     Status {
///         version: EthVersion::Eth66.into(),
///         chain: Chain::Named(ethers_core::types::Chain::Mainnet),
///         total_difficulty: U256::from(100),
///         blockhash: H256::from(MAINNET_GENESIS),
///         genesis: H256::from(MAINNET_GENESIS),
///         forkid: Hardfork::Paris.fork_id(&MAINNET).unwrap(),
///     }
/// );
/// ```
#[derive(Debug, Default)]
pub struct StatusBuilder {
    status: Status
}

impl StatusBuilder {
    /// Consumes the type and creates the actual
    /// [`Status`](crate::types::Status) message.
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

    /// Sets the total difficulty.
    pub fn total_difficulty(mut self, total_difficulty: U256) -> Self {
        self.status.total_difficulty = total_difficulty;
        self
    }

    /// Sets the block hash.
    pub fn blockhash(mut self, blockhash: H256) -> Self {
        self.status.blockhash = blockhash;
        self
    }

    /// Sets the genesis hash.
    pub fn genesis(mut self, genesis: H256) -> Self {
        self.status.genesis = genesis;
        self
    }

    /// Sets the fork id.
    pub fn forkid(mut self, forkid: ForkId) -> Self {
        self.status.forkid = forkid;
        self
    }
}

/// Builder for [`HelloMessage`](crate::HelloMessage) messages.
pub struct HelloBuilder {
    hello: HelloMessage
}

impl HelloBuilder {
    /// Creates a new [`HelloBuilder`](crate::builder::HelloBuilder) with
    /// default [`HelloMessage`] values, and a with a signed 'Hello' message
    /// to verify the public key
    pub fn new(signature: RecoverableSignature, signed_hello: H256, id: PeerId) -> Self {
        let sig = signature.serialize_compact();
        Self {
            hello: HelloMessage {
                protocol_version: ProtocolVersion::V5,
                // TODO: proper client versioning
                client_version: "Ethereum/1.0.0".to_string(),
                capabilities: vec![EthVersion::Eth68.into()],
                port: DEFAULT_DISCOVERY_PORT,
                signed_hello,
                signature: sig.1.to_vec(),
                sig_recovery_id: sig.0.to_i32().to_be_bytes().to_vec(),
                id
            }
        }
    }

    /// Consumes the type and creates the actual [`HelloMessage`] message.
    pub fn build(self) -> HelloMessage {
        self.hello
    }

    /// Sets the protocol version.
    pub fn protocol_version(mut self, protocol_version: ProtocolVersion) -> Self {
        self.hello.protocol_version = protocol_version;
        self
    }

    /// Sets the client version.
    pub fn client_version(mut self, client_version: String) -> Self {
        self.hello.client_version = client_version;
        self
    }

    /// Sets the capabilities.
    pub fn capabilities(mut self, capabilities: Vec<Capability>) -> Self {
        self.hello.capabilities = capabilities;
        self
    }

    /// Sets the port.
    pub fn port(mut self, port: u16) -> Self {
        self.hello.port = port;
        self
    }

    /// Sets the signed hello message.
    pub fn signed_hello(mut self, signed_hello: H256) -> Self {
        self.hello.signed_hello = signed_hello;
        self
    }

    /// Sets the signed hello message.
    pub fn signature(mut self, id: PeerId) -> Self {
        self.hello.id = id;
        self
    }
}