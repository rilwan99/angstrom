use tokio::sync::mpsc;

pub mod handle;
pub use handle::*;

pub mod session;
pub use session::*;

pub mod config;
pub use config::*;

pub mod connection_handler;
use std::{collections::HashMap, fmt::Debug, net::SocketAddr};

pub use connection_handler::*;
use reth_network::protocol::ProtocolHandler;
use reth_primitives::PeerId;
use reth_provider::StateProvider;
use tokio::time::Duration;

pub struct StromSessionManager {
    // All pending session that are currently handshaking, exchanging `Hello`s.
    //pending_sessions: FnvHashMap<SessionId, PendingSessionHandle>,
    // All active sessions that are ready to exchange messages.
    //active_sessions:  HashMap<PeerId, ActiveProtocolSessionHandle>
    to_sessions: HashMap<PeerId, StromSessionHandle>,

    from_sessions: mpsc::Receiver<StromSessionMessage>
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
    pub to_session_manager: mpsc::Sender<StromSessionMessage>,
    /// State provider to determine if the pub key is an staked validator with
    /// sufficient balance
    pub state:              DB,
    /// Protocol Sessions Config
    pub config:             SessionsConfig
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
