mod session;
use session::*;

mod handle;
use std::{net::SocketAddr, sync::Arc};

use handle::*;
use reth_eth_wire::{
    capability::{Capabilities, CapabilityMessage, RawCapabilityMessage, SharedCapabilities},
    multiplex::{ProtocolConnection, ProtocolProxy, RlpxProtocolMultiplexer, RlpxSatelliteStream},
    protocol::Protocol,
    DisconnectReason, EthStream, EthVersion, Status
};
use reth_metrics::common::mpsc::MeteredPollSender;
use reth_network::{
    protocol::{ConnectionHandler, OnNotSupported},
    Direction
};
use reth_primitives::{ForkFilter, PeerId};
use tokio::sync::mpsc;

use crate::{
    errors::{StromHandshakeError, StromStreamError},
    types::message::{StromBroadcastMessage, StromMessage}
};
pub struct StromSessionManager {
    // All pending session that are currently handshaking, exchanging `Hello`s.
    //pending_sessions: FnvHashMap<SessionId, PendingSessionHandle>,
    // All active sessions that are ready to exchange messages.
    //active_sessions:  HashMap<PeerId, ActiveProtocolSessionHandle>
    to_sessions: Vec<mpsc::Sender<StromSessionCommand>>,

    from_sessions: mpsc::Receiver<StromSessionMessage>
}

/// The protocol handler that is used to announce the strom capability upon
/// successfully establishing a hello handshake on an incoming tcp connection.
pub struct StromProtocolHandler {
    /// When a new connection is created, the conection handler will use
    /// this channel to send the sender half of the sessions command channel to
    /// the manager via the `Established` event.
    pub to_session_manager: MeteredPollSender<StromSessionMessage>
}

pub struct StromConnectionHandler {
    pub protocol:           Protocol,
    pub to_session_manager: MeteredPollSender<StromSessionMessage>,
    pub status:             Status,
    pub fork_filter:        ForkFilter,
    pub to_wire:            mpsc::Sender<StromBroadcastMessage>
}

impl ConnectionHandler for StromConnectionHandler {
    type Connection = StromSession;

    fn protocol(&self) -> Protocol {
        StromProtocol::default()
    }

    fn on_unsupported_by_peer(
        self,
        supported: &SharedCapabilities,
        direction: Direction,
        peer_id: PeerId
    ) -> OnNotSupported {
        OnNotSupported::Disconnect
    }

    fn into_connection(
        self,
        direction: Direction,
        peer_id: PeerId,
        conn: ProtocolConnection
    ) -> Self::Connection {
        let (tx, rx) = mpsc::unbounded_channel();
        self.to_session_manager
            .events
            .send(ProtocolEvent::Established { direction, peer_id: _peer_id, to_connection: tx })
            .ok();
        PingPongProtoConnection {
            conn,
            initial_ping: direction.is_outgoing().then(PingPongProtoMessage::ping),
            commands: UnboundedReceiverStream::new(rx),
            pending_pong: None
        }
    }
}

/// Message variants an active session can produce and send back to the
/// [`SessionManager`](crate::session::SessionManager)
#[derive(Debug)]
pub enum StromSessionMessage {
    /// Session was established.
    Established {
        direction:     Direction,
        peer_id:       PeerId,
        to_connection: mpsc::UnboundedSender<StromSessionCommand>
    },

    /// Session was gracefully disconnected.
    Disconnected {
        /// The remote node's public key
        peer_id:     PeerId,
        /// The remote node's socket address
        remote_addr: SocketAddr
    },
    /// Session was closed due an error
    ClosedOnConnectionError {
        /// The remote node's public key
        peer_id:     PeerId,
        /// The remote node's socket address
        remote_addr: SocketAddr,
        /// The error that caused the session to close
        error:       StromStreamError
    },
    /// A session received a valid message via RLPx.
    ValidMessage {
        /// Identifier of the remote peer.
        peer_id: PeerId,
        /// Message received from the peer.
        message: StromMessage
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
    }
}

#[derive(Debug)]
pub enum StromSessionCommand {
    /// Disconnect the connection
    Disconnect {
        /// Why the disconnect was initiated
        reason: Option<DisconnectReason>
    },
    /// Sends a message to the peer
    Message(StromMessage)
}
