mod session;
use fnv::FnvHashMap;
use reth_eth_wire::{
    capability::SharedCapabilities,
    multiplex::{ProtocolConnection, ProtocolProxy, RlpxProtocolMultiplexer, RlpxSatelliteStream},
    protocol::Protocol,
    EthStream, EthVersion, Status
};
use reth_network::{
    protocol::{ConnectionHandler, OnNotSupported, ProtocolHandler},
    Direction
};
use reth_primitives::{ForkFilter, PeerId};
use session::*;
pub struct StromSessionManager {
    // All pending session that are currently handshaking, exchanging `Hello`s.
    //pending_sessions: FnvHashMap<SessionId, PendingSessionHandle>,
    // All active sessions that are ready to exchange messages.
    //active_sessions:  HashMap<PeerId, ActiveProtocolSessionHandle>
}

pub struct StromConnectionHandler {
    pub protocol:           Protocol,
    pub to_session_manager: MeteredPollSender<ActiveSessionMessage>,
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
        let mut st =
            RlpxProtocolMultiplexer::new(conn)
                .into_eth_satellite_stream(
                    EthVersion::Eth68,
                    self.status.clone(),
                    self.fork_filter.clone()
                )
                .await
                .unwrap();

        st.install_protocol(&cap, |mut conn| {
            ProtocolSession::new(conn, self.to_session_manager.clone())
        })
        .unwrap()
    }
}
