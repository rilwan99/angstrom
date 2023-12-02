mod session;
use session::*;

mod handle;
use std::{net::SocketAddr, sync::Arc};

use handle::*;
use reth_eth_wire::{
    capability::{Capabilities, CapabilityMessage, RawCapabilityMessage, SharedCapabilities},
    multiplex::{ProtocolConnection, ProtocolProxy, RlpxProtocolMultiplexer, RlpxSatelliteStream},
    protocol::Protocol,
    EthStream, EthVersion, Status
};
use reth_metrics::common::mpsc::MeteredPollSender;
use reth_network::{
    protocol::{ConnectionHandler, OnNotSupported},
    Direction
};
use reth_primitives::{ForkFilter, PeerId};

use crate::{
    errors::{StromHandshakeError, StromStreamError},
    types::message::StromMessage
};
pub struct StromSessionManager {
    // All pending session that are currently handshaking, exchanging `Hello`s.
    //pending_sessions: FnvHashMap<SessionId, PendingSessionHandle>,
    // All active sessions that are ready to exchange messages.
    //active_sessions:  HashMap<PeerId, ActiveProtocolSessionHandle>
}

pub struct StromConnectionHandler {
    pub protocol:           Protocol,
    pub to_session_manager: MeteredPollSender<StromSessionMessage>,
    pub status:             Status,
    pub fork_filter:        ForkFilter
}

impl ConnectionHandler for StromConnectionHandler {
    type Connection = RlpxSatelliteStream<ProtocolSession, EthStream<ProtocolProxy>>;

    fn protocol(&self) -> Protocol {
        self.protocol
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
        let cap = self.protocol.capability();
        let mut st = RlpxProtocolMultiplexer::new(conn)
            .into_eth_satellite_stream(
                EthVersion::Eth68,
                self.status.clone(),
                self.fork_filter.clone()
            )
            .unwrap();

        st.install_protocol(&cap, |mut conn| {
            ProtocolSession::new(conn, self.to_session_manager.clone())
        })
        .unwrap()
    }
}

/// Message variants an active session can produce and send back to the
/// [`SessionManager`](crate::session::SessionManager)
#[derive(Debug)]
pub enum StromSessionMessage {
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
