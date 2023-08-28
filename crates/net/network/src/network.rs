use crate::{
    discovery::DiscoveryEvent,
    peers::PeersHandle, swarm::NetworkEvent,
};
use async_trait::async_trait;
use parking_lot::Mutex;
use reth_eth_wire::DisconnectReason;
use reth_net_common::bandwidth_meter::BandwidthMeter;
use reth_network_api::{
    NetworkError, PeerKind, Peers, PeersInfo, Reputation, ReputationChangeKind,
};
use reth_primitives::{NodeRecord, PeerId};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::sync::{mpsc, mpsc::UnboundedSender, oneshot};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// A _shareable_ network frontend. Used to interact with the network.
///
/// See also [`NetworkManager`](crate::NetworkManager).
#[derive(Clone, Debug)]
pub struct NetworkHandle {
    /// The Arc'ed delegate that contains the state.
    inner: Arc<NetworkInner>,
}

// === impl NetworkHandle ===

impl NetworkHandle {
    /// Creates a single new instance.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        num_active_peers: Arc<AtomicUsize>,
        listener_address: Arc<Mutex<SocketAddr>>,
        to_manager_tx: UnboundedSender<NetworkHandleMessage>,
        local_peer_id: PeerId,
        peers: PeersHandle,
        bandwidth_meter: BandwidthMeter,
    ) -> Self {
        let inner = NetworkInner {
            num_active_peers,
            to_manager_tx,
            listener_address,
            local_peer_id,
            peers,
            bandwidth_meter,
            is_syncing: Arc::new(AtomicBool::new(false)),
            initial_sync_done: Arc::new(AtomicBool::new(false)),
        };
        Self { inner: Arc::new(inner) }
    }

    /// Returns the [`PeerId`] used in the network.
    pub fn peer_id(&self) -> &PeerId {
        &self.inner.local_peer_id
    }

    /// Returns the [`PeersHandle`] that can be cloned and shared.
    ///
    /// The [`PeersHandle`] can be used to interact with the network's peer set.
    pub fn peers_handle(&self) -> &PeersHandle {
        &self.inner.peers
    }

    fn manager(&self) -> &UnboundedSender<NetworkHandleMessage> {
        &self.inner.to_manager_tx
    }

    /// Creates a new [`NetworkEvent`] listener channel.
    pub fn event_listener(&self) -> UnboundedReceiverStream<NetworkEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let _ = self.manager().send(NetworkHandleMessage::EventListener(tx));
        UnboundedReceiverStream::new(rx)
    }

    /// Returns a new [`DiscoveryEvent`] stream.
    ///
    /// This stream yields [`DiscoveryEvent`]s for each peer that is discovered.
    pub fn discovery_listener(&self) -> UnboundedReceiverStream<DiscoveryEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let _ = self.manager().send(NetworkHandleMessage::DiscoveryListener(tx));
        UnboundedReceiverStream::new(rx)
    }


    /// Sends a [`NetworkHandleMessage`] to the manager
    pub(crate) fn send_message(&self, msg: NetworkHandleMessage) {
        let _ = self.inner.to_manager_tx.send(msg);
    }


    /// Provides a shareable reference to the [`BandwidthMeter`] stored on the [`NetworkInner`]
    pub fn bandwidth_meter(&self) -> &BandwidthMeter {
        &self.inner.bandwidth_meter
    }

    /// Send message to gracefully shutdown node.
    ///
    /// This will disconnect all active and pending sessions and prevent
    /// new connections to be established.
    pub async fn shutdown(&self) -> Result<(), oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();
        self.send_message(NetworkHandleMessage::Shutdown(tx));
        rx.await
    }
}

// === API Implementations ===

impl PeersInfo for NetworkHandle {
    fn num_connected_peers(&self) -> usize {
        self.inner.num_active_peers.load(Ordering::Relaxed)
    }

    fn local_node_record(&self) -> NodeRecord {
        let id = *self.peer_id();
        let mut socket_addr = *self.inner.listener_address.lock();

        if socket_addr.ip().is_unspecified() {
            // zero address is invalid
            if socket_addr.ip().is_ipv4() {
                socket_addr.set_ip(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
            } else {
                socket_addr.set_ip(std::net::IpAddr::V6(std::net::Ipv6Addr::LOCALHOST));
            }
        }

        NodeRecord::new(socket_addr, id)
    }
}

#[async_trait]
impl Peers for NetworkHandle {
    /// Sends a message to the [`NetworkManager`](crate::NetworkManager) to add a peer to the known
    /// set, with the given kind.
    fn add_peer_kind(&self, peer: PeerId, kind: PeerKind, addr: SocketAddr) {
        self.send_message(NetworkHandleMessage::AddPeerAddress(peer, kind, addr));
    }

    /// Sends a message to the [`NetworkManager`](crate::NetworkManager) to remove a peer from the
    /// set corresponding to given kind.
    fn remove_peer(&self, peer: PeerId, kind: PeerKind) {
        self.send_message(NetworkHandleMessage::RemovePeer(peer, kind))
    }

    /// Sends a message to the [`NetworkManager`](crate::NetworkManager)  to disconnect an existing
    /// connection to the given peer.
    fn disconnect_peer(&self, peer: PeerId) {
        self.send_message(NetworkHandleMessage::DisconnectPeer(peer, None))
    }

    /// Sends a message to the [`NetworkManager`](crate::NetworkManager)  to disconnect an existing
    /// connection to the given peer using the provided reason
    fn disconnect_peer_with_reason(&self, peer: PeerId, reason: DisconnectReason) {
        self.send_message(NetworkHandleMessage::DisconnectPeer(peer, Some(reason)))
    }

    /// Send a reputation change for the given peer.
    fn reputation_change(&self, peer_id: PeerId, kind: ReputationChangeKind) {
        self.send_message(NetworkHandleMessage::ReputationChange(peer_id, kind));
    }

    async fn reputation_by_id(&self, peer_id: PeerId) -> Result<Option<Reputation>, NetworkError> {
        let (tx, rx) = oneshot::channel();
        let _ = self.manager().send(NetworkHandleMessage::GetReputationById(peer_id, tx));
        Ok(rx.await?)
    }
}


#[derive(Debug)]
struct NetworkInner {
    /// Number of active peer sessions the node's currently handling.
    num_active_peers: Arc<AtomicUsize>,
    /// Sender half of the message channel to the [`crate::NetworkManager`].
    to_manager_tx: UnboundedSender<NetworkHandleMessage>,
    /// The local address that accepts incoming connections.
    listener_address: Arc<Mutex<SocketAddr>>,
    /// The identifier used by this node.
    local_peer_id: PeerId,
    /// Access to the all the nodes.
    peers: PeersHandle,
    /// Used to measure inbound & outbound bandwidth across network streams (currently unused)
    bandwidth_meter: BandwidthMeter,
    /// Represents if the network is currently syncing.
    is_syncing: Arc<AtomicBool>,
    /// Used to differentiate between an initial pipeline sync or a live sync
    initial_sync_done: Arc<AtomicBool>,
}

/// Internal messages that can be passed to the  [`NetworkManager`](crate::NetworkManager).
#[allow(missing_docs)]
pub(crate) enum NetworkHandleMessage {
    /// Adds an address for a peer.
    AddPeerAddress(PeerId, PeerKind, SocketAddr),
    /// Removes a peer from the peerset corresponding to the given kind.
    RemovePeer(PeerId, PeerKind),
    /// Disconnect a connection to a peer if it exists.
    DisconnectPeer(PeerId, Option<DisconnectReason>),
    /// Add a new listener for [`NetworkEvent`].
    EventListener(UnboundedSender<NetworkEvent>),
    /// Apply a reputation change to the given peer.
    ReputationChange(PeerId, ReputationChangeKind),
    /// Get the reputation for a specific peer
    GetReputationById(PeerId, oneshot::Sender<Option<Reputation>>),
    /// Gracefully shutdown network
    Shutdown(oneshot::Sender<()>),
    /// Add a new listener for `DiscoveryEvent`.
    DiscoveryListener(UnboundedSender<DiscoveryEvent>),
}