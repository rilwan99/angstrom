use std::sync::{atomic::AtomicUsize, Arc};

use guard_types::orders::PooledOrder;
use order_pool::OrderPoolHandle;
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_network::DisconnectReason;
use reth_rpc_types::PeerId;
use tokio::sync::{mpsc::UnboundedSender, oneshot};

use crate::{ReputationChangeKind, StromMessage, StromNetworkEvent};

//TODO:
// 1) Implement the order pool manager
// 2) Implement the consensus manager
// 3)
#[derive(Debug)]
#[allow(dead_code)]
pub struct StromNetworkHandle {
    inner: Arc<StromNetworkInner>
}

impl StromNetworkHandle {
    /// Sends a [`NetworkHandleMessage`] to the manager
    pub(crate) fn send_message(&self, msg: StromNetworkHandleMsg) {
        let _ = self.inner.to_manager_tx.send(msg);
    }

    /// Send full transactions to the peer
    pub fn send_transactions(&self, peer_id: PeerId, msg: StromMessage) {
        self.send_message(StromNetworkHandleMsg::SendOrders { peer_id, msg })
    }

    pub fn broadcast_tx(&self, msg: StromMessage) {
        self.send_message(StromNetworkHandleMsg::BroadcastOrder { msg });
    }

    pub fn peer_reputation_change(&self, peer: PeerId, change: ReputationChangeKind) {
        self.send_message(StromNetworkHandleMsg::ReputationChange(peer, change));
    }

    /// Send message to gracefully shutdown node.
    ///
    /// This will disconnect all active and pending sessions and prevent
    /// new connections to be established.
    pub async fn shutdown(&self) -> Result<(), oneshot::error::RecvError> {
        let (tx, rx) = oneshot::channel();
        self.send_message(StromNetworkHandleMsg::Shutdown(tx));
        rx.await
    }

    /// Sends a message to the [`NetworkManager`](crate::NetworkManager) to
    /// remove a peer from the set corresponding to given kind.
    fn remove_peer(&self, peer: PeerId) {
        self.send_message(StromNetworkHandleMsg::RemovePeer(peer))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct StromNetworkInner {
    num_active_peers: Arc<AtomicUsize>,

    to_manager_tx: UnboundedMeteredSender<StromNetworkHandleMsg>
}

/// All events related to orders emitted by the network.
#[derive(Debug)]
pub enum NetworkOrderEvent {
    IncomingOrders { peer_id: PeerId, orders: Vec<PooledOrder> }
}

#[derive(Debug)]
pub enum StromNetworkHandleMsg {
    /// Removes a peer from the peer set corresponding to the given kind.
    RemovePeer(PeerId),
    /// Disconnect a connection to a peer if it exists.
    DisconnectPeer(PeerId, Option<DisconnectReason>),
    /// Add a new listener for [`NetworkEvent`].
    EventListener(UnboundedSender<StromNetworkEvent>),

    /// Sends the list of transactions to the given peer.
    SendOrders { peer_id: PeerId, msg: StromMessage },

    /// broadcasts the order
    BroadcastOrder { msg: StromMessage },

    /// Apply a reputation change to the given peer.
    ReputationChange(PeerId, ReputationChangeKind),
    /// Gracefully shutdown network
    Shutdown(oneshot::Sender<()>)
}
