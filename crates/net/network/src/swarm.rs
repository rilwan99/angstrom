use std::{
    io,
    net::SocketAddr,
    pin::Pin,
    sync::{atomic::AtomicUsize, Arc},
    task::{Context, Poll}
};

use ethers_core::types::transaction::eip712::TypedData;
use futures::Stream;
use parking_lot::Mutex;
use reth_eth_wire::{
    capability::{Capabilities, CapabilityMessage},
    errors::EthStreamError,
    DisconnectReason, EthVersion, Status
};
use reth_net_common::bandwidth_meter::BandwidthMeter;
use reth_network_api::ReputationChangeKind;
use reth_primitives::{listener::EventListeners, ForkId, NodeRecord, PeerId, H256};
use reth_provider::{BlockNumReader, BlockReader};
use shared::{SealedBundle, *};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, trace};

use crate::{
    error::{NetworkError, ServiceKind},
    listener::{ConnectionListener, ListenerEvent},
    messages::{PeerMessages, PeerRequests},
    network::NetworkHandleMessage,
    peers::{InboundConnectionError, PeersHandle, PeersManager},
    session::{Direction, PendingSessionHandshakeError, SessionEvent, SessionId, SessionManager},
    state::{NetworkState, StateAction},
    Discovery, NetworkConfig, NetworkHandle
};

/// Contains the connectivity related state of the network.
///
/// A swarm emits [`SwarmEvent`]s when polled.
///
/// The manages the [`ConnectionListener`] and delegates new incoming
/// connections to the [`SessionManager`]. Outgoing connections are either
/// initiated on demand or triggered by the [`NetworkState`] and also delegated
/// to the [`NetworkState`].
///
/// Following diagram gives displays the dataflow contained in the [`Swarm`]
///
/// The [`ConnectionListener`] yields incoming [`TcpStream`]s from peers that
/// are spawned as session tasks. After a successful RLPx authentication, the
/// task is ready to accept ETH requests or broadcast messages. A task listens
/// for messages from the [`SessionManager`] which include broadcast messages
/// like `Transactions` or internal commands, for example to disconnect the
/// session.
///
/// The [`NetworkState`] keeps track of all connected and discovered peers and
/// can initiate outgoing connections. For each active session, the
/// [`NetworkState`] keeps a sender half of the ETH request channel for the
/// created session and sends requests it receives from the [`StateFetcher`],
/// which receives request objects from the client interfaces responsible for
/// downloading headers and bodies.
#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
///  graph TB
///     connections(TCP Listener)
///     Discovery[(Discovery)]
///     fetchRequest(Client Interfaces)
///     Sessions[(SessionManager)]
///     SessionTask[(Peer Session)]
///     State[(State)]
///     StateFetch[(State Fetcher)]
///   connections --> |incoming| Sessions
///   State --> |initiate outgoing| Sessions
///   Discovery --> |update peers| State
///   Sessions --> |spawns| SessionTask
///   SessionTask <--> |handle state requests| State
///   fetchRequest --> |request Headers, Bodies| StateFetch
///   State --> |poll pending requests| StateFetch
/// ```
#[must_use = "Swarm does nothing unless polled"]
pub struct Swarm {
    /// Listens for new incoming connections.
    incoming:             ConnectionListener,
    /// All sessions.
    sessions:             SessionManager,
    /// Tracks the entire state of the network and handles events received from
    /// the sessions.
    state:                NetworkState,
    /// Tracks the connection state of the node
    net_connection_state: NetworkConnectionState,
    /// Underlying network handle that can be shared.
    handle:               NetworkHandle,
    /// Receiver half of the command channel set up between this type and the
    /// [`NetworkHandle`]
    from_handle_rx:       UnboundedReceiverStream<NetworkHandleMessage>,
    /// All listeners for high level network events.
    event_listeners:      EventListeners<NetworkEvent>,
    /// Tracks the number of active session (connected peers).
    ///
    /// This is updated via internal events and shared via `Arc` with the
    /// [`NetworkHandle`] Updated by the `NetworkWorker` and loaded by the
    /// `NetworkService`.
    num_active_peers:     Arc<AtomicUsize>
}

// === impl Swarm ===

impl Swarm {
    /// Configures a new swarm instance.
    pub async fn new(config: NetworkConfig) -> Result<Self, NetworkError> {
        let NetworkConfig {
            secret_key,
            mut discovery_v4_config,
            discovery_addr,
            listener_addr,
            peers_config,
            sessions_config,
            chain_spec,
            boot_nodes,
            executor,
            hello_message,
            status,
            fork_filter,
            dns_discovery_config,
            ..
        } = config;

        let peers_manager = PeersManager::new(peers_config);
        let peers_handle = peers_manager.handle();

        let incoming = ConnectionListener::bind(listener_addr)
            .await
            .map_err(|err| {
                NetworkError::from_io_error(err, ServiceKind::Listener(listener_addr))
            })?;
        let listener_address = Arc::new(Mutex::new(incoming.local_address()));

        discovery_v4_config = discovery_v4_config.map(|mut disc_config| {
            // merge configured boot nodes
            disc_config.bootstrap_nodes.extend(boot_nodes.clone());
            disc_config.add_eip868_pair("eth", status.forkid);
            disc_config
        });

        let discovery =
            Discovery::new(discovery_addr, secret_key, discovery_v4_config, dns_discovery_config)
                .await?;
        // need to retrieve the addr here since provided port could be `0`
        let local_peer_id = discovery.local_id();

        let num_active_peers = Arc::new(AtomicUsize::new(0));
        let bandwidth_meter: BandwidthMeter = BandwidthMeter::default();

        let sessions = SessionManager::new(
            secret_key,
            sessions_config,
            executor,
            status,
            hello_message,
            fork_filter,
            bandwidth_meter.clone()
        );

        let state = NetworkState::new(
            discovery,
            peers_manager,
            chain_spec.genesis_hash(),
            Arc::clone(&num_active_peers)
        );

        let (to_manager_tx, from_handle_rx) = mpsc::unbounded_channel();

        let handle = NetworkHandle::new(
            Arc::clone(&num_active_peers),
            listener_address,
            to_manager_tx,
            local_peer_id,
            peers_handle,
            bandwidth_meter
        );
        Ok(Self {
            incoming,
            sessions,
            state,
            net_connection_state: NetworkConnectionState::default(),
            handle,
            from_handle_rx: UnboundedReceiverStream::new(from_handle_rx),
            event_listeners: Default::default(),
            num_active_peers
        })
    }

    pub fn propagate_msg(&mut self, msg: PeerMessages) {
        self.sessions_mut().propagate_msg(msg)
    }

    /// Returns a shareable reference to the [`BandwidthMeter`] stored
    /// inside of the [`NetworkHandle`]
    pub fn bandwidth_meter(&self) -> &BandwidthMeter {
        self.handle.bandwidth_meter()
    }

    /// Returns the [`NetworkHandle`] that can be cloned and shared.
    ///
    /// The [`NetworkHandle`] can be used to interact with this
    /// [`NetworkManager`]
    pub fn handle(&self) -> &NetworkHandle {
        &self.handle
    }

    /// Access to the state.
    pub(crate) fn state(&self) -> &NetworkState {
        &self.state
    }

    /// Mutable access to the state.
    pub(crate) fn state_mut(&mut self) -> &mut NetworkState {
        &mut self.state
    }

    /// Access to the [`ConnectionListener`].
    pub(crate) fn listener(&self) -> &ConnectionListener {
        &self.incoming
    }

    /// Access to the [`SessionManager`].
    pub(crate) fn sessions(&self) -> &SessionManager {
        &self.sessions
    }

    /// Mutable access to the [`SessionManager`].
    pub(crate) fn sessions_mut(&mut self) -> &mut SessionManager {
        &mut self.sessions
    }

    /// Triggers a new outgoing connection to the given node
    pub(crate) fn dial_outbound(&mut self, remote_addr: SocketAddr, remote_id: PeerId) {
        self.sessions.dial_outbound(remote_addr, remote_id)
    }

    /// Returns the [`SocketAddr`] that listens for incoming connections.
    pub fn local_addr(&self) -> SocketAddr {
        self.listener().local_address()
    }

    /// Returns the configured genesis hash
    pub fn genesis_hash(&self) -> H256 {
        self.state().genesis_hash()
    }

    /// How many peers we're currently connected to.
    pub fn num_connected_peers(&self) -> usize {
        self.state().num_active_peers()
    }

    /// Returns the [`PeerId`] used in the network.
    pub fn peer_id(&self) -> &PeerId {
        self.handle.peer_id()
    }

    /// Returns an iterator over all peers in the peer set.
    pub fn all_peers(&self) -> impl Iterator<Item = NodeRecord> + '_ {
        self.state().peers().iter_peers()
    }

    /// Returns a new [`PeersHandle`] that can be cloned and shared.
    ///
    /// The [`PeersHandle`] can be used to interact with the network's peer set.
    pub fn peers_handle(&self) -> PeersHandle {
        self.state().peers().handle()
    }

    /// Event hook for an unexpected message from the peer.
    fn on_invalid_message(
        &mut self,
        peer_id: PeerId,
        _capabilities: Arc<Capabilities>,
        _message: CapabilityMessage
    ) {
        trace!(target : "net", ?peer_id,  "received unexpected message");
        self.state_mut()
            .peers_mut()
            .apply_reputation_change(&peer_id, ReputationChangeKind::BadProtocol);
    }

    /// Handles a polled [`SessionEvent`]
    fn on_session_event(&mut self, event: SessionEvent) -> Option<SwarmEvent> {
        match event {
            SessionEvent::Message { request, peer_id } => {
                Some(SwarmEvent::ValidMessage { peer_id, request })
            }

            SessionEvent::SessionEstablished {
                peer_id,
                remote_addr,
                client_version,
                capabilities,
                version,
                status,
                direction,
                timeout
            } => {
                self.state
                    .on_session_activated(peer_id, capabilities.clone(), timeout);
                Some(SwarmEvent::SessionEstablished {
                    peer_id,
                    remote_addr,
                    client_version,
                    capabilities,
                    version,
                    status,
                    direction
                })
            }
            SessionEvent::AlreadyConnected { peer_id, remote_addr, direction } => {
                trace!( target: "net", ?peer_id, ?remote_addr, ?direction, "already connected");
                self.state.peers_mut().on_already_connected(direction);
                None
            }
            SessionEvent::InvalidMessage { peer_id, capabilities, message } => {
                Some(SwarmEvent::InvalidCapabilityMessage { peer_id, capabilities, message })
            }
            SessionEvent::IncomingPendingSessionClosed { remote_addr, error } => {
                Some(SwarmEvent::IncomingPendingSessionClosed { remote_addr, error })
            }
            SessionEvent::OutgoingPendingSessionClosed { remote_addr, peer_id, error } => {
                Some(SwarmEvent::OutgoingPendingSessionClosed { remote_addr, peer_id, error })
            }
            SessionEvent::Disconnected { peer_id, remote_addr } => {
                self.state.on_session_closed(peer_id);
                Some(SwarmEvent::SessionClosed { peer_id, remote_addr, error: None })
            }
            SessionEvent::SessionClosedOnConnectionError { peer_id, remote_addr, error } => {
                self.state.on_session_closed(peer_id);
                Some(SwarmEvent::SessionClosed { peer_id, remote_addr, error: Some(error) })
            }
            SessionEvent::OutgoingConnectionError { remote_addr, peer_id, error } => {
                Some(SwarmEvent::OutgoingConnectionError { peer_id, remote_addr, error })
            }
            SessionEvent::BadMessage { peer_id } => Some(SwarmEvent::BadMessage { peer_id }),
            SessionEvent::ProtocolBreach { peer_id } => Some(SwarmEvent::ProtocolBreach { peer_id })
        }
    }

    /// Callback for events produced by [`ConnectionListener`].
    ///
    /// Depending on the event, this will produce a new [`SwarmEvent`].
    fn on_connection(&mut self, event: ListenerEvent) -> Option<SwarmEvent> {
        match event {
            ListenerEvent::Error(err) => return Some(SwarmEvent::TcpListenerError(err)),
            ListenerEvent::ListenerClosed { local_address: address } => {
                return Some(SwarmEvent::TcpListenerClosed { remote_addr: address })
            }
            ListenerEvent::Incoming { stream, remote_addr } => {
                // Reject incoming connection if node is shutting down.
                if self.is_shutting_down() {
                    return None
                }
                // ensure we can handle an incoming connection from this address
                if let Err(err) = self
                    .state_mut()
                    .peers_mut()
                    .on_incoming_pending_session(remote_addr.ip())
                {
                    match err {
                        InboundConnectionError::IpBanned => {
                            trace!(target: "net", ?remote_addr, "The incoming ip address is in the ban list");
                        }
                        InboundConnectionError::ExceedsLimit(limit) => {
                            trace!(target: "net", %limit, ?remote_addr, "Exceeded incoming connection limit; disconnecting");
                            self.sessions.disconnect_incoming_connection(
                                stream,
                                DisconnectReason::TooManyPeers
                            );
                        }
                    }
                    return None
                }

                match self.sessions.on_incoming(stream, remote_addr) {
                    Ok(session_id) => {
                        trace!(target: "net", ?remote_addr, "Incoming connection");
                        return Some(SwarmEvent::IncomingTcpConnection { session_id, remote_addr })
                    }
                    Err(err) => {
                        debug!(target: "net", ?err, "Incoming connection rejected, capacity already reached.");
                        self.state_mut()
                            .peers_mut()
                            .on_incoming_pending_session_rejected_internally();
                    }
                }
            }
        }
        None
    }

    /// Hook for actions pulled from the state
    fn on_state_action(&mut self, event: StateAction) -> Option<SwarmEvent> {
        match event {
            StateAction::Connect { remote_addr, peer_id } => {
                self.dial_outbound(remote_addr, peer_id);
                return Some(SwarmEvent::OutgoingTcpConnection { remote_addr, peer_id })
            }
            StateAction::Disconnect { peer_id, reason } => {
                self.sessions.disconnect(peer_id, reason);
            }
            StateAction::PeerAdded(peer_id) => return Some(SwarmEvent::PeerAdded(peer_id)),
            StateAction::PeerRemoved(peer_id) => return Some(SwarmEvent::PeerRemoved(peer_id)),
            StateAction::DiscoveredNode { peer_id, socket_addr, fork_id } => {
                // Don't try to connect to peer if node is shutting down
                if self.is_shutting_down() {
                    return None
                }
                // Insert peer only if no fork id or a valid fork id
                if fork_id.map_or_else(|| true, |f| self.sessions.is_valid_fork_id(f)) {
                    self.state_mut()
                        .peers_mut()
                        .add_peer(peer_id, socket_addr, fork_id);
                }
            }
            StateAction::DiscoveredEnrForkId { peer_id, fork_id } => {
                if self.sessions.is_valid_fork_id(fork_id) {
                    self.state_mut()
                        .peers_mut()
                        .set_discovered_fork_id(peer_id, fork_id);
                } else {
                    self.state_mut().peers_mut().remove_peer(peer_id);
                }
            }
        }
        None
    }

    /// Set network connection state to `ShuttingDown`
    pub(crate) fn on_shutdown_requested(&mut self) {
        self.net_connection_state = NetworkConnectionState::ShuttingDown;
    }

    /// Checks if the node's network connection state is 'ShuttingDown'
    #[inline]
    pub(crate) fn is_shutting_down(&self) -> bool {
        matches!(self.net_connection_state, NetworkConnectionState::ShuttingDown)
    }

    /// Handler for received messages from a handle
    fn on_handle_message(&mut self, msg: NetworkHandleMessage) {
        match msg {
            NetworkHandleMessage::EventListener(tx) => {
                self.event_listeners.push_listener(tx);
            }
            NetworkHandleMessage::DiscoveryListener(tx) => {
                self.state_mut().discovery_mut().add_listener(tx);
            }
            NetworkHandleMessage::AddPeerAddress(peer, kind, addr) => {
                // only add peer if we are not shutting down
                if !self.is_shutting_down() {
                    self.state_mut().add_peer_kind(peer, kind, addr);
                }
            }
            NetworkHandleMessage::RemovePeer(peer_id, kind) => {
                self.state_mut().remove_peer(peer_id, kind);
            }
            NetworkHandleMessage::DisconnectPeer(peer_id, reason) => {
                self.sessions_mut().disconnect(peer_id, reason);
            }
            NetworkHandleMessage::Shutdown(tx) => {
                // Set connection status to `Shutdown`. Stops node to accept
                // new incoming connections as well as sending connection requests to newly
                // discovered nodes.
                self.on_shutdown_requested();
                // Disconnect all active connections
                self.sessions_mut()
                    .disconnect_all(Some(DisconnectReason::ClientQuitting));
                // drop pending connections
                self.sessions_mut().disconnect_all_pending();
                let _ = tx.send(());
            }
            NetworkHandleMessage::ReputationChange(peer_id, kind) => {
                self.state_mut()
                    .peers_mut()
                    .apply_reputation_change(&peer_id, kind);
            }
            NetworkHandleMessage::GetReputationById(peer_id, tx) => {
                let _ = tx.send(self.state_mut().peers().get_reputation(&peer_id));
            }
        }
    }
}

impl Stream for Swarm {
    type Item = SwarmEvent;

    /// This advances all components.
    ///
    /// Processes, delegates (internal) commands received from the
    /// [`NetworkManager`](crate::NetworkManager), then polls the
    /// [`SessionManager`] which yields messages produced by individual peer
    /// sessions that are then handled. Least priority are incoming
    /// connections that are handled and delegated to the [`SessionManager`]
    /// to turn them into a session.
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            while let Poll::Ready(action) = this.state.poll(cx) {
                if let Some(event) = this.on_state_action(action) {
                    return Poll::Ready(Some(event))
                }
            }

            // poll all sessions
            match this.sessions.poll(cx) {
                Poll::Pending => {}
                Poll::Ready(event) => {
                    if let Some(event) = this.on_session_event(event) {
                        return Poll::Ready(Some(event))
                    }
                    continue
                }
            }

            // poll listener for incoming connections
            match Pin::new(&mut this.incoming).poll(cx) {
                Poll::Pending => {}
                Poll::Ready(event) => {
                    if let Some(event) = this.on_connection(event) {
                        return Poll::Ready(Some(event))
                    }
                    continue
                }
            }

            return Poll::Pending
        }
    }
}

/// All events created or delegated by the [`Swarm`] that represents changes to
/// the state of the network.
#[derive(Debug)]
pub enum SwarmEvent {
    /// Events related to the actual network protocol.
    ValidMessage {
        /// The peer that sent the message
        peer_id: PeerId,
        /// the request the peer is making
        request: PeerMessages
    },
    /// Received a message that does not match the announced capabilities of the
    /// peer.
    InvalidCapabilityMessage {
        peer_id:      PeerId,
        /// Announced capabilities of the remote peer.
        capabilities: Arc<Capabilities>,
        /// Message received from the peer.
        message:      CapabilityMessage
    },
    /// Received a bad message from the peer.
    BadMessage {
        /// Identifier of the remote peer.
        peer_id: PeerId
    },
    /// Remote peer is considered in protocol violation
    ProtocolBreach {
        /// Identifier of the remote peer.
        peer_id: PeerId
    },
    /// The underlying tcp listener closed.
    TcpListenerClosed {
        /// Address of the closed listener.
        remote_addr: SocketAddr
    },
    /// The underlying tcp listener encountered an error that we bubble up.
    TcpListenerError(io::Error),
    /// Received an incoming tcp connection.
    ///
    /// This represents the first step in the session authentication process.
    /// The swarm will produce subsequent events once the stream has been
    /// authenticated, or was rejected.
    IncomingTcpConnection {
        /// The internal session identifier under which this connection is
        /// currently tracked.
        session_id:  SessionId,
        /// Address of the remote peer.
        remote_addr: SocketAddr
    },
    /// An outbound connection is initiated.
    OutgoingTcpConnection {
        /// Address of the remote peer.
        peer_id:     PeerId,
        remote_addr: SocketAddr
    },
    SessionEstablished {
        peer_id:        PeerId,
        remote_addr:    SocketAddr,
        client_version: Arc<String>,
        capabilities:   Arc<Capabilities>,
        /// negotiated eth version
        version:        EthVersion,
        status:         Status,
        direction:      Direction
    },
    SessionClosed {
        peer_id:     PeerId,
        remote_addr: SocketAddr,
        /// Whether the session was closed due to an error
        error:       Option<EthStreamError>
    },
    /// Admin rpc: new peer added
    PeerAdded(PeerId),
    /// Admin rpc: peer removed
    PeerRemoved(PeerId),
    /// Closed an incoming pending session during authentication.
    IncomingPendingSessionClosed {
        remote_addr: SocketAddr,
        error:       Option<PendingSessionHandshakeError>
    },
    /// Closed an outgoing pending session during authentication.
    OutgoingPendingSessionClosed {
        remote_addr: SocketAddr,
        peer_id:     PeerId,
        error:       Option<PendingSessionHandshakeError>
    },
    /// Failed to establish a tcp stream to the given address/node
    OutgoingConnectionError { remote_addr: SocketAddr, peer_id: PeerId, error: io::Error }
}

/// Represents the state of the connection of the node. If shutting down,
/// new connections won't be established.
#[derive(Default)]
pub(crate) enum NetworkConnectionState {
    #[default]
    Active,
    ShuttingDown
}

/// (Non-exhaustive) Events emitted by the network that are of interest for
/// subscribers.
///
/// This includes any event types that may be relevant to tasks, for metrics,
/// keep track of peers etc.
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Closed the peer session.
    SessionClosed {
        /// The identifier of the peer to which a session was closed.
        peer_id: PeerId,
        /// Why the disconnect was triggered
        reason:  Option<DisconnectReason>
    },
    /// Established a new session with the given peer.
    SessionEstablished {
        /// The identifier of the peer to which a session was established.
        peer_id:        PeerId,
        /// The remote addr of the peer to which a session was established.
        remote_addr:    SocketAddr,
        /// The client version of the peer to which a session was established.
        client_version: Arc<String>,
        /// Capabilities the peer announced
        capabilities:   Arc<Capabilities>,
        /// The status of the peer to which a session was established.
        status:         Status,
        /// negotiated eth version of the session
        version:        EthVersion
    },
    /// Event emitted when a new peer is added
    PeerAdded(PeerId),
    /// Event emitted when a new peer is removed
    PeerRemoved(PeerId)
}

#[derive(Debug, Clone)]
pub enum DiscoveredEvent {
    EventQueued { peer_id: PeerId, socket_addr: SocketAddr, fork_id: Option<ForkId> }
}
