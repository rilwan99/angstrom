//! Keeps track of the state of the network.

use std::{
    collections::{HashMap, VecDeque},
    net::{IpAddr, SocketAddr},
    sync::{
        atomic::{AtomicU64, AtomicUsize},
        Arc
    },
    task::{Context, Poll}
};

use guard_eth_wire::{capability::Capabilities, DisconnectReason};
use reth_network_api::PeerKind;
use reth_primitives::{ForkId, PeerId, H256};
use tracing::debug;

use crate::{
    discovery::{Discovery, DiscoveryEvent},
    peers::{PeerAction, PeersManager},
    swarm::DiscoveredEvent
};

/// Cache limit of blocks to keep track of for a single peer.
const PEER_BLOCK_CACHE_LIMIT: usize = 512;

/// The [`NetworkState`] keeps track of the state of all peers in the network.
///
/// This includes:
///   - [`Discovery`]: manages the discovery protocol, essentially a stream of
///     discovery updates
///   - [`PeersManager`]: keeps track of connected peers and issues new outgoing
///     connections depending on the configured capacity.
///   - [`StateFetcher`]: streams download request (received from outside via
///     channel) which are then send to the session of the peer.
///
/// This type is also responsible for responding for received request.
pub struct NetworkState {
    /// All active peers and their state.
    active_peers:    HashMap<PeerId, ActivePeer>,
    /// Manages connections to peers.
    peers_manager:   PeersManager,
    /// Buffered messages until polled.
    queued_messages: VecDeque<StateAction>,
    /// The client type that can interact with the chain.
    ///
    /// This type is used to fetch the block number after we established a
    /// session and received the [Status] block hash.
    /// Network discovery.
    discovery:       Discovery,
    /// The genesis hash of the network we're on
    genesis_hash:    H256
}

impl NetworkState {
    /// Create a new state instance with the given params
    pub(crate) fn new(
        discovery: Discovery,
        peers_manager: PeersManager,
        genesis_hash: H256,
        _num_active_peers: Arc<AtomicUsize>
    ) -> Self {
        Self {
            active_peers: Default::default(),
            peers_manager,
            queued_messages: Default::default(),
            discovery,
            genesis_hash
        }
    }

    /// Returns mutable access to the [`PeersManager`]
    pub(crate) fn peers_mut(&mut self) -> &mut PeersManager {
        &mut self.peers_manager
    }

    /// Returns mutable access to the [`Discovery`]
    pub(crate) fn discovery_mut(&mut self) -> &mut Discovery {
        &mut self.discovery
    }

    /// Returns access to the [`PeersManager`]
    pub(crate) fn peers(&self) -> &PeersManager {
        &self.peers_manager
    }

    /// Configured genesis hash.
    pub fn genesis_hash(&self) -> H256 {
        self.genesis_hash
    }

    /// How many peers we're currently connected to.
    pub fn num_active_peers(&self) -> usize {
        self.active_peers.len()
    }

    /// Event hook for an activated session for the peer.
    ///
    /// Returns `Ok` if the session is valid, returns an `Err` if the session is
    /// not accepted and should be rejected.
    pub(crate) fn on_session_activated(
        &mut self,
        peer: PeerId,
        capabilities: Arc<Capabilities>,
        _timeout: Arc<AtomicU64>
    ) {
        debug_assert!(!self.active_peers.contains_key(&peer), "Already connected; not possible");

        self.active_peers.insert(peer, ActivePeer { capabilities });
    }

    /// Event hook for a disconnected session for the given peer.
    ///
    /// This will remove the peer from the available set of peers and close all
    /// inflight requests.
    pub(crate) fn on_session_closed(&mut self, peer: PeerId) {
        self.active_peers.remove(&peer);
    }

    /// Invoked when a new [`ForkId`] is activated.
    pub(crate) fn update_fork_id(&mut self, fork_id: ForkId) {
        self.discovery.update_fork_id(fork_id)
    }

    /// Bans the [`IpAddr`] in the discovery service.
    pub(crate) fn ban_ip_discovery(&self, ip: IpAddr) {
        debug!(target: "net", ?ip, "Banning discovery");
        self.discovery.ban_ip(ip)
    }

    /// Bans the [`PeerId`] and [`IpAddr`] in the discovery service.
    pub(crate) fn ban_discovery(&self, peer_id: PeerId, ip: IpAddr) {
        debug!(target: "net", ?peer_id, ?ip, "Banning discovery");
        self.discovery.ban(peer_id, ip)
    }

    /// Adds a peer and its address with the given kind to the peerset.
    pub(crate) fn add_peer_kind(&mut self, peer_id: PeerId, kind: PeerKind, addr: SocketAddr) {
        self.peers_manager.add_peer_kind(peer_id, kind, addr, None)
    }

    pub(crate) fn remove_peer(&mut self, peer_id: PeerId, kind: PeerKind) {
        match kind {
            PeerKind::Basic => self.peers_manager.remove_peer(peer_id),
            PeerKind::Trusted => self.peers_manager.remove_peer_from_trusted_set(peer_id)
        }
    }

    /// Event hook for events received from the discovery service.
    fn on_discovery_event(&mut self, event: DiscoveryEvent) {
        match event {
            DiscoveryEvent::NewNode(DiscoveredEvent::EventQueued {
                peer_id,
                socket_addr,
                fork_id
            }) => {
                self.queued_messages.push_back(StateAction::DiscoveredNode {
                    peer_id,
                    socket_addr,
                    fork_id
                });
            }
            DiscoveryEvent::EnrForkId(peer_id, fork_id) => {
                self.queued_messages
                    .push_back(StateAction::DiscoveredEnrForkId { peer_id, fork_id });
            }
        }
    }

    /// Event hook for new actions derived from the peer management set.
    fn on_peer_action(&mut self, action: PeerAction) {
        match action {
            PeerAction::Connect { peer_id, remote_addr } => {
                self.queued_messages
                    .push_back(StateAction::Connect { peer_id, remote_addr });
            }
            PeerAction::Disconnect { peer_id, reason } => {
                self.queued_messages
                    .push_back(StateAction::Disconnect { peer_id, reason });
            }
            PeerAction::DisconnectBannedIncoming { peer_id } => {
                self.queued_messages
                    .push_back(StateAction::Disconnect { peer_id, reason: None });
            }
            PeerAction::DiscoveryBanPeerId { peer_id, ip_addr } => {
                self.ban_discovery(peer_id, ip_addr)
            }
            PeerAction::DiscoveryBanIp { ip_addr } => self.ban_ip_discovery(ip_addr),
            PeerAction::PeerAdded(peer_id) => self
                .queued_messages
                .push_back(StateAction::PeerAdded(peer_id)),
            PeerAction::PeerRemoved(peer_id) => self
                .queued_messages
                .push_back(StateAction::PeerRemoved(peer_id)),
            PeerAction::BanPeer { .. } => {}
            PeerAction::UnBanPeer { .. } => {}
        }
    }

    /// Advances the state
    pub(crate) fn poll(&mut self, cx: &mut Context<'_>) -> Poll<StateAction> {
        loop {
            // drain buffered messages
            if let Some(message) = self.queued_messages.pop_front() {
                return Poll::Ready(message)
            }

            while let Poll::Ready(discovery) = self.discovery.poll(cx) {
                self.on_discovery_event(discovery);
            }

            // while let Poll::Ready(action) = self.state_fetcher.poll(cx) {
            //     match action {
            //         FetchAction::BlockRequest { peer_id, request } => {
            //             self.handle_block_request(peer_id, request)
            //         }
            //     }
            // }

            // need to buffer results here to make borrow checker happy
            let closed_sessions = Vec::new();

            for peer in closed_sessions {
                self.on_session_closed(peer)
            }

            // poll peer manager
            while let Poll::Ready(action) = self.peers_manager.poll(cx) {
                self.on_peer_action(action);
            }

            if self.queued_messages.is_empty() {
                return Poll::Pending
            }
        }
    }
}

/// Tracks the state of a Peer with an active Session.
///
/// For example known blocks,so we can decide what to announce.
pub(crate) struct ActivePeer {
    /// The capabilities of the remote peer.
    #[allow(unused)]
    pub(crate) capabilities: Arc<Capabilities>
}

/// Message variants triggered by the [`NetworkState`]
pub(crate) enum StateAction {
    /// Create a new connection to the given node.
    Connect { remote_addr: SocketAddr, peer_id: PeerId },
    /// Disconnect an existing connection
    Disconnect {
        peer_id: PeerId,
        /// Why the disconnect was initiated
        reason:  Option<DisconnectReason>
    },
    /// Retrieved a [`ForkId`] from the peer via ENR request, See <https://eips.ethereum.org/EIPS/eip-868>
    DiscoveredEnrForkId {
        peer_id: PeerId,
        /// The reported [`ForkId`] by this peer.
        fork_id: ForkId
    },
    /// A new node was found through the discovery, possibly with a ForkId
    DiscoveredNode { peer_id: PeerId, socket_addr: SocketAddr, fork_id: Option<ForkId> },
    /// A peer was added
    PeerAdded(PeerId),
    /// A peer was dropped
    PeerRemoved(PeerId)
}
