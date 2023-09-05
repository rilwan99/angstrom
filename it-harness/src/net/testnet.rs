//! A network implementation for testing purposes.

use std::{
    fmt,
    future::Future,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    pin::Pin,
    task::{Context, Poll}
};

use ethers_core::{
    rand,
    rand::{rngs::OsRng, Rng}
};
use futures::{Stream, StreamExt};
use guard_eth_wire::{
    capability::Capability, errors::EthStreamError, DisconnectReason, HelloBuilder,
    DEFAULT_HELLO_VERIFICATION_MESSAGE
};
use guard_network::{
    config::SecretKey, error::NetworkError, peers::PeersHandle, NetworkConfig,
    NetworkConfigBuilder, Swarm, SwarmEvent
};
use pin_project::pin_project;
use reth_primitives::{
    bytes::{BufMut, BytesMut},
    hex_literal::hex,
    keccak256, sign_message, Address, Bytes, PeerId, H512, U256
};
use secp256k1::{
    ecdsa,
    ecdsa::{RecoverableSignature, RecoveryId},
    Message, PublicKey, Secp256k1, SECP256K1
};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        oneshot
    },
    task::JoinHandle
};

/// A test network consisting of multiple peers.
pub struct Testnet {
    /// All running peers in the network.
    pub peers: Vec<Peer>,
    pub tx:    UnboundedSender<(Option<SwarmEvent>, H512)>
}

// === impl Testnet ===

impl Testnet {
    /// Same as [`Self::try_create_with`] but panics on error
    pub async fn create_with(
        num_peers: usize,
        tx: UnboundedSender<(Option<SwarmEvent>, H512)>
    ) -> Self {
        Self::try_create_with(num_peers, tx).await.unwrap()
    }

    /// Creates a new [`Testnet`] with the given number of peers and the
    /// provider.
    pub async fn try_create_with(
        num_peers: usize,
        tx: UnboundedSender<(Option<SwarmEvent>, H512)>
    ) -> Result<Self, NetworkError> {
        //let st_rx = UnboundedReceiverStream::new(rx);
        let mut this = Self { peers: Vec::with_capacity(num_peers), tx };
        for i in 0..num_peers {
            let config = PeerConfig::new().await;
            this.add_peer_with_config(config).await?;
        }
        Ok(this)
    }

    /// Return a mutable slice of all peers.
    pub fn peers_mut(&mut self) -> &mut [Peer] {
        &mut self.peers
    }

    /// Return a slice of all peers.
    pub fn peers(&self) -> &[Peer] {
        &self.peers
    }

    /// Return a mutable iterator over all peers.
    pub fn peers_iter_mut(&mut self) -> impl Iterator<Item = &mut Peer> + '_ {
        self.peers.iter_mut()
    }

    /// Return an iterator over all peers.
    pub fn peers_iter(&self) -> impl Iterator<Item = &Peer> + '_ {
        self.peers.iter()
    }

    /// Returns all handles in the networks
    pub fn handles(&self) -> impl Iterator<Item = PeersHandle> + '_ {
        self.peers.iter().map(|p| p.handle())
    }

    /// Returns all local address in the networks
    pub fn addresses(&self) -> impl Iterator<Item = SocketAddr> + '_ {
        self.peers.iter().map(|p| p.local_addr())
    }

    /// Returns all peer ids in the networks
    pub fn peer_ids(&self) -> impl Iterator<Item = PeerId> + '_ {
        self.peers.iter().map(|p| p.id())
    }

    /// Extend the list of peers with new peers that are configured with each of
    /// the given [`PeerConfig`]s.
    pub async fn extend_peer_with_config(
        &mut self,
        configs: impl IntoIterator<Item = PeerConfig>
    ) -> Result<(), NetworkError> {
        let peers = configs
            .into_iter()
            .map(|c| async move { c.launch().await })
            .collect::<Vec<_>>();
        for peer in peers {
            self.peers.push(peer.await?);
        }
        Ok(())
    }

    /// Add a peer to the [`Testnet`] with the given [`PeerConfig`].
    pub async fn add_peer_with_config(&mut self, config: PeerConfig) -> Result<(), NetworkError> {
        let peer = config.launch().await.unwrap();
        self.peers.push(peer);
        Ok(())
    }

    /// Apply a closure on each peer
    pub fn for_each<F>(&self, f: F)
    where
        F: Fn(&Peer)
    {
        self.peers.iter().for_each(f)
    }

    /// Apply a closure on each peer
    pub fn for_each_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut Peer)
    {
        self.peers.iter_mut().for_each(f)
    }
}

impl Testnet {
    /// Spawns the testnet to a separate task
    pub fn spawn(self) -> TestnetHandle {
        let (tx, rx) = oneshot::channel::<oneshot::Sender<Self>>();
        let mut net = self;
        let handle = tokio::task::spawn(async move {
            let mut tx = None;
            loop {
                tokio::select! {
                    _ = &mut net => { break }
                    inc = rx => {
                        tx = inc.ok();
                        break
                    }
                }
            }
            if let Some(tx) = tx {
                let _ = tx.send(net);
            }
        });

        TestnetHandle { _handle: handle, terminate: tx }
    }
}

impl fmt::Debug for Testnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Testnet {{}}").finish_non_exhaustive()
    }
}

impl Future for Testnet {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        for peer in this.peers.iter_mut() {
            if let Poll::Ready(val) = peer.poll_next_unpin(cx) {
                this.tx.send((val, peer.id())).unwrap();
            }
        }
        Poll::Pending
    }
}

/// A handle to a [`Testnet`] that can be shared.
pub struct TestnetHandle {
    _handle:   JoinHandle<()>,
    terminate: oneshot::Sender<oneshot::Sender<Testnet>>
}

// === impl TestnetHandle ===

impl TestnetHandle {
    /// Terminates the task and returns the [`Testnet`] back.
    pub async fn terminate(self) -> Testnet {
        let (tx, rx) = oneshot::channel();
        self.terminate.send(tx).unwrap();
        rx.await.unwrap()
    }
}

#[pin_project]
pub struct Peer {
    client:     Swarm,
    pub_key:    PeerId,
    secret_key: SecretKey
}

// === impl Peer ===

impl Peer {
    /// Returns the number of connected peers.
    pub fn num_peers(&self) -> usize {
        self.client.num_connected_peers()
    }

    /// Returns the number of connected peers.
    pub fn id(&self) -> PeerId {
        self.pub_key
    }

    /// The address that listens for incoming connections.
    pub fn local_addr(&self) -> SocketAddr {
        self.client.local_addr()
    }

    /// Adds a peer connection
    pub fn add_peer(&self, peer_id: PeerId, addr: SocketAddr) {
        self.client.peers_handle().add_peer(peer_id, addr);
    }

    /// returns the handles
    pub fn handle(&self) -> PeersHandle {
        self.client.peers_handle()
    }
}

impl Stream for Peer {
    type Item = SwarmEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Poll::Ready(val) = self.client.poll_next_unpin(cx) {
                return Poll::Ready(val)
            }
            //println!("NONE");
            return Poll::Pending
        }
    }
}

/// A helper config for setting up the reth networking stack.
pub struct PeerConfig {
    pub net_config: NetworkConfig,
    pub_key:        PeerId,
    secret_key:     SecretKey
}

// === impl PeerConfig ===

impl PeerConfig {
    /// Launches the network and returns the [Peer] that manages it
    pub async fn launch(self) -> Result<Peer, NetworkError> {
        let PeerConfig { secret_key, pub_key, net_config } = self;

        let (_, rx) = unbounded_channel();
        let net = Swarm::new(net_config, rx).await.unwrap();

        let peer = Peer { client: net, pub_key, secret_key };
        Ok(peer)
    }

    /// Initialize the network with a random secret key, allowing the devp2p and
    /// discovery to bind to any available IP and port.
    pub async fn new() -> Self {
        let (secret_key, pub_key) = secp256k1::generate_keypair(&mut rand::thread_rng());

        let hash = keccak256(&pub_key.serialize_uncompressed()[1..]);
        let addr = Address::from_slice(&hash[12..]);
        //println!("ADDRESS INIT {:#x}", addr);

        let pub_key: [u8; 64] = pub_key.serialize_uncompressed()[1..].try_into().unwrap();
        //println!("PUB KEY INIT {:#x}", H512::from_slice(&pub_key));
        let net_config = Self::network_config_builder(secret_key, pub_key.into()).build();

        Self { net_config, pub_key: pub_key.into(), secret_key }
    }

    /// Initialize the network with a given secret key, allowing devp2p and
    /// discovery to bind any available IP and port.
    pub async fn with_secret_key(secret_key: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let pub_key: [u8; 64] = PublicKey::from_secret_key(&secp, &secret_key)
            .serialize_uncompressed()[1..]
            .try_into()
            .unwrap();

        let net_config = Self::network_config_builder(secret_key, pub_key.into()).build();

        Self { net_config, pub_key: pub_key.into(), secret_key }
    }

    /// Initialize the network with a given capabilities.
    pub async fn with_capabilities(capabilities: Vec<Capability>) -> Self {
        let (secret_key, pub_key) = secp256k1::generate_keypair(&mut rand::thread_rng());
        let pub_key: [u8; 64] = pub_key.serialize_uncompressed()[1..].try_into().unwrap();

        let builder = Self::network_config_builder(secret_key, pub_key.into());
        let (sig, signed_hello) = builder.get_signed_hello();
        let hello_message = HelloBuilder::new(sig, signed_hello, pub_key.into())
            .capabilities(capabilities)
            .build();
        let net_config = builder.hello_message(hello_message).build();

        Self { net_config, pub_key: pub_key.into(), secret_key }
    }

    pub fn network_config_builder(secret_key: SecretKey, pub_key: PeerId) -> NetworkConfigBuilder {
        NetworkConfigBuilder::new(secret_key, DEFAULT_HELLO_VERIFICATION_MESSAGE, pub_key)
            .listener_addr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)))
            .discovery_addr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)))
            .disable_discv4_discovery()
            .disable_dns_discovery()
    }
}

/// Awaits the next event for an established session filtered by peer id
pub async fn next_session_established(
    rx: &mut UnboundedReceiver<(Option<SwarmEvent>, H512)>,
    filter_id: PeerId
) -> Option<H512> {
    while let Some(event) = rx.recv().await {
        if let (Some(ev), id) = event {
            if id != filter_id {
                continue
            }
            //println!("{:?}\n", ev);
            match ev {
                SwarmEvent::SessionEstablished { peer_id, .. } => return Some(peer_id),
                _ => continue
            }
        }
    }
    None
}

/// Awaits the next event for a session to be closed filtered by peer id
pub async fn next_session_closed(
    rx: &mut UnboundedReceiver<(Option<SwarmEvent>, H512)>,
    filter_id: PeerId
) -> Option<PeerId> {
    while let Some(event) = rx.recv().await {
        //println!("SPAWNED");
        if let (Some(ev), id) = event {
            //println!("THIS HANDLE: {:?}\n{:#x}", ev, filter_id);
            if id != filter_id {
                continue
            }
            //println!("{:?}", ev);
            match ev {
                SwarmEvent::AlreadyConnected { peer_id } => {
                    //println!("\n\n\nLOLOLOLOLOLOL\n\n\n");
                    return Some(peer_id)
                }
                SwarmEvent::SessionClosed { peer_id, .. } => return Some(peer_id),
                _ => continue
            }
        }
    }
    None
}
