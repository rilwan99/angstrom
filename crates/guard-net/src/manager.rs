use std::sync::{atomic::AtomicUsize, Arc};

use guard_types::orders::{
    ComposableLimitOrders, ComposableSearcherOrders, LimitOrders, Orders, SearcherOrders
};
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_rpc_types::PeerId;

use crate::{pool_manager::PoolHandle, StromNetworkEvent};

//TODO:
// 1) Implement the order pool manager
// 2) Implement the consensus manager
// 3)
#[derive(Debug)]
pub struct StromProtocolHandle {
    inner: Arc<StromInner>
}

#[derive(Debug)]
struct StromInner {
    num_active_peers: Arc<AtomicUsize>,
    to_manager_tx:    UnboundedMeteredSender<StromNetworkEvent>
}
/// All events related to orders emitted by the network.
#[derive(Debug)]
pub enum NetworkOrderEvent {
    /// Received list of orders from a peer
    IncomingLimitOrders {
        peer_id: PeerId,
        orders:  Vec<LimitOrders>
    },
    IncomingComposableOrders {
        peer_id: PeerId,
        orders:  Vec<ComposableLimitOrders>
    },
    IncomingSearcherOrders {
        peer_id: PeerId,
        orders:  Vec<SearcherOrders>
    },
    IncommingComposableSearcherOrders {
        peer_id: PeerId,
        orders:  Vec<ComposableSearcherOrders>
    },
    IncomingOrders {
        peer_id: PeerId,
        orders:  Vec<Orders>
    }
}

pub struct StromNetworkManager {
    inner:  Arc<StromInner>,
    handle: PoolHandle
}

/*
impl ProtocolHandler for NetworkManager {
    type ConnectionHandler = StromConnectionHandler;

    fn on_incoming(&self, socket_addr: SocketAddr) -> Option<Self::ConnectionHandler> {
        todo!()
    }

    fn on_outgoing(
        &self,
        socket_addr: SocketAddr,
        peer_id: PeerId
    ) -> Option<Self::ConnectionHandler> {
        todo!()
    }
}

pub struct StromConnectionHandler {
    protocol: Protocol
}

impl Default for StromConnectionHandler {
    fn default() -> Self {
        Self { protocol: Protocol::new("strom", 1) }
    }
}

impl ConnectionHandler for StromConnectionHandler {
    type Connection:

    fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Invoked when the RLPx connection has been established by the peer does
    /// not share the protocol.
    fn on_unsupported_by_peer(
        self,
        supported: &SharedCapabilities,
        direction: Direction,
        peer_id: PeerId
    ) -> OnNotSupported {
        todo!()}

    /// Invoked when the RLPx connection was established.
    ///
    /// The returned future should resolve when the connection should
    /// disconnect.
    fn into_connection(
        self,
        direction: Direction,
        peer_id: PeerId,
        conn: ProtocolConnection
    ) -> Self::Connection {
        todo!()
        //Our handashake re: checking stake + pub key
        // 1) Check if the peer's pub key _ time + peer id
        //
    }
}


*/
