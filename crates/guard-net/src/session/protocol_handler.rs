use std::{fmt::Debug, net::SocketAddr};

use reth_metrics::common::mpsc::MeteredPollSender;
use reth_network::protocol::ProtocolHandler;
use reth_primitives::PeerId;
use reth_provider::StateProvider;
use secp256k1::SecretKey;
use tokio::time::Duration;

use crate::{
    SessionsConfig, Status, StromConnectionHandler, StromNetworkHandle, StromSessionMessage
};

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
    pub network:            StromNetworkHandle,
    #[allow(dead_code)]
    //TODO: Use status / connect message to verify that the connection is a valid staker
    status: Status,
    #[allow(dead_code)]
    secret_key:             SecretKey
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
    /// here we have to add the outgoing connect message and send it to the peer
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

impl<DB> StromProtocolHandler<DB>
where
    DB: StateProvider + Debug + 'static
{
    /* TODO: Implement the builder pattern for the network + protocol components
    pub fn new(network: StromNetworkHandle, state: DB) -> Self {
        let (to_session_manager, from_session_manager) =
            mpsc::channel(config.session_command_buffer);

        let to_session_manager = PollSender::new(to_session_manager);
        Self {
            to_session_manager: MeteredPollSender::new(
                to_session_manager,
                "protocol_handler_to_session_manager"
            ),
            state,
            config,
            network
        }
    } */
}
