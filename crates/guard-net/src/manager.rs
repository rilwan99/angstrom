use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicUsize, Arc},
    task::Context
};

use futures::{task::Poll, StreamExt};
use reth_eth_wire::DisconnectReason;
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_primitives::PeerId;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::error;

use crate::{NetworkOrderEvent, StromNetworkHandleMsg, Swarm};
#[allow(unused_imports)]
use crate::{StromNetworkConfig, StromNetworkHandle, StromSessionManager};

#[allow(dead_code)]
pub struct StromNetworkManager {
    handle:               StromNetworkHandle,
    from_handle_rx:       UnboundedReceiverStream<StromNetworkHandleMsg>,
    swarm:                Swarm,
    to_pool_manager:      Option<UnboundedMeteredSender<NetworkOrderEvent>>,
    to_consensus_manager: Option<UnboundedMeteredSender<StromNetworkEvent>>,

    /// This is updated via internal events and shared via `Arc` with the
    /// [`NetworkHandle`] Updated by the `NetworkWorker` and loaded by the
    /// `NetworkService`.
    num_active_peers: Arc<AtomicUsize>
}

impl StromNetworkManager {
    // Handler for received messages from a handle
    fn on_handle_message(&mut self, msg: StromNetworkHandleMsg) {
        match msg {
            StromNetworkHandleMsg::SendOrders { peer_id, msg } => {
                self.swarm.sessions_mut().send_message(&peer_id, msg)
            }
            StromNetworkHandleMsg::Shutdown(tx) => {
                // Disconnect all active connections
                self.swarm
                    .sessions_mut()
                    .disconnect_all(Some(DisconnectReason::ClientQuitting));

                // drop pending connections

                let _ = tx.send(());
            }
            StromNetworkHandleMsg::RemovePeer(peer_id) => {
                self.swarm.state_mut().remove_peer(peer_id);
            }

            _ => todo!()
        }
    }

    /// Sends an event to the pool manager.
    fn notify_pool_manager(&self, event: NetworkOrderEvent) {
        if let Some(ref tx) = self.to_pool_manager {
            let _ = tx.send(event);
        }
    }
}

impl Future for StromNetworkManager {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // process incoming messages from a handle
        loop {
            match this.from_handle_rx.poll_next_unpin(cx) {
                Poll::Pending => break,
                Poll::Ready(None) => {
                    // This is only possible if the channel was deliberately closed since we always
                    // have an instance of `NetworkHandle`
                    error!("Strom network message channel closed.");
                    return Poll::Ready(())
                }
                Poll::Ready(Some(msg)) => this.on_handle_message(msg)
            };
        }

        Poll::Pending
    }
}

/// (Non-exhaustive) Events emitted by the network that are of interest for
/// subscribers.
///
/// This includes any event types that may be relevant to tasks, for metrics,
/// keep track of peers etc.
#[derive(Debug, Clone)]
pub enum StromNetworkEvent {
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
        /// The client version of the peer to which a session was established.
        client_version: Arc<str>
    },
    /// Event emitted when a new peer is added
    PeerAdded(PeerId),
    /// Event emitted when a new peer is removed
    PeerRemoved(PeerId)
}
