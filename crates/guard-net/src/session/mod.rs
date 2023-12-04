use reth_metrics::common::mpsc::MeteredPollSender;
use tokio::sync::mpsc;

pub mod handle;
pub use handle::*;

pub mod session;
use futures::Stream;
pub use session::*;
pub mod config;
pub use config::*;
use futures::task::Context;
pub mod connection_handler;
use std::{
    collections::HashMap,
    fmt::Debug,
    net::SocketAddr,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc}
};

pub use connection_handler::*;
use futures::{io, task::Poll};
use reth_network::{protocol::ProtocolHandler, Direction};
use reth_primitives::PeerId;
use reth_provider::StateProvider;
use tokio::time::Duration;

use crate::{errors::StromStreamError, StromNetworkHandle, StromProtocolMessage};

#[allow(dead_code)]
pub struct StromSessionManager {
    next_id:     usize,
    // All pending session that are currently handshaking, exchanging `Hello`s.
    //pending_sessions: FnvHashMap<SessionId, PendingSessionHandle>,
    // All active sessions that are ready to exchange messages.
    //active_sessions:  HashMap<PeerId, ActiveProtocolSessionHandle>
    to_sessions: HashMap<PeerId, StromSessionHandle>,

    from_sessions: mpsc::Receiver<StromSessionMessage>
}

impl Stream for StromSessionManager {
    type Item = SessionEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<SessionEvent>> {
        let this = self.get_mut();

        loop {
            match this.from_sessions.poll_recv(cx) {
                Poll::Ready(Some(StromSessionMessage::Established { handle })) => {
                    let event = SessionEvent::SessionEstablished {
                        peer_id:   handle.remote_id,
                        direction: handle.direction,
                        timeout:   Arc::new(AtomicU64::new(40))
                    };
                    this.to_sessions.insert(handle.remote_id, handle);
                    return Poll::Ready(Some(event))
                }

                Poll::Pending => {
                    return Poll::Pending;
                }

                Poll::Ready(None) => {
                    unreachable!("Manager holds both channel halves.")
                }
                _ => {}
            }
        }
    }
}

/// Events produced by the [`SessionManager`]
#[derive(Debug)]
pub enum SessionEvent {
    /// A new session was successfully authenticated.
    ///
    /// This session is now able to exchange data.
    SessionEstablished {
        /// The remote node's public key
        peer_id:   PeerId,
        /// The direction of the session, either `Inbound` or `Outgoing`
        direction: Direction,
        /// The maximum time that the session waits for a response from the peer
        /// before timing out the connection
        timeout:   Arc<AtomicU64>
    },
    /// The peer was already connected with another session.
    AlreadyConnected {
        /// The remote node's public key
        peer_id:     PeerId,
        /// The remote node's socket address
        remote_addr: SocketAddr,
        /// The direction of the session, either `Inbound` or `Outgoing`
        direction:   Direction
    },
    /// A session received a valid message via RLPx.
    ValidMessage {
        /// The remote node's public key
        peer_id: PeerId,
        /// Message received from the peer.
        message: StromProtocolMessage
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

    /// Failed to establish a tcp stream
    OutgoingConnectionError {
        /// The remote node's socket address
        remote_addr: SocketAddr,
        /// The remote node's public key
        peer_id:     PeerId,
        /// The error that caused the outgoing connection to fail
        error:       io::Error
    },
    /// Session was closed due to an error
    SessionClosedOnConnectionError {
        /// The id of the remote peer.
        peer_id:     PeerId,
        /// The socket we were connected to.
        remote_addr: SocketAddr,
        /// The error that caused the session to close
        error:       StromStreamError
    },
    /// Active session was gracefully disconnected.
    Disconnected {
        /// The remote node's public key
        peer_id:     PeerId,
        /// The remote node's socket address that we were connected to
        remote_addr: SocketAddr
    }
}

/// The protocol handler that is used to announce the strom capability upon
/// successfully establishing a hello handshake on an incoming tcp connection.
#[derive(Debug)]
pub struct StromProtocolHandler<DB>
where
    DB: StateProvider + Debug + 'static
{
    /// When a new connection is created, the conection handler will use
    /// this channel to send the sender half of the sessions command channel to
    /// the manager via the `Established` event.
    pub to_session_manager: MeteredPollSender<StromSessionMessage>,
    /// State provider to determine if the pub key is an staked validator with
    /// sufficient balance
    pub state:              DB,
    /// Protocol Sessions Config
    pub config:             SessionsConfig,
    /// Network Handle
    pub network:            StromNetworkHandle
}

impl<DB> ProtocolHandler for StromProtocolHandler<DB>
where
    DB: StateProvider + Debug + 'static
{
    type ConnectionHandler = StromConnectionHandler;

    fn on_incoming(&self, _socket_addr: SocketAddr) -> Option<Self::ConnectionHandler> {
        Some(StromConnectionHandler {
            to_session_manager: self.to_session_manager.clone(),
            status: None,
            protocol_breach_request_timeout: Duration::from_secs(10),
            session_command_buffer: 100
        })
    }

    /// Invoked when a new outgoing connection to the remote is requested.
    fn on_outgoing(
        &self,
        _socket_addr: SocketAddr,
        _peer_id: PeerId
    ) -> Option<Self::ConnectionHandler> {
        Some(StromConnectionHandler {
            to_session_manager: self.to_session_manager.clone(),
            status: None,
            protocol_breach_request_timeout: Duration::from_secs(10),
            session_command_buffer: self.config.session_command_buffer
        })
    }
}
