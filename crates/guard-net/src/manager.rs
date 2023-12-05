use std::sync::{atomic::AtomicUsize, Arc};

use reth_eth_wire::DisconnectReason;
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_primitives::PeerId;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[allow(unused_imports)]
use crate::{StromNetworkConfig, StromNetworkHandle, StromSessionManager};

#[allow(dead_code)]
pub struct StromNetworkManager {
    handle:               StromNetworkHandle,
    from_handle:          UnboundedReceiverStream<StromNetworkEvent>,
    session_manager:      StromSessionManager,
    to_pool_manager:      Option<UnboundedMeteredSender<StromNetworkEvent>>,
    to_consensus_manager: Option<UnboundedMeteredSender<StromNetworkEvent>>,

    /// This is updated via internal events and shared via `Arc` with the
    /// [`NetworkHandle`] Updated by the `NetworkWorker` and loaded by the
    /// `NetworkService`.
    num_active_peers: Arc<AtomicUsize>
}

impl StromNetworkManager {
    /*pub async fn new(network: StromNetworkConfig) {
        let (from_handle, to_handle) = mpsc::unbounded_channel();
        let (to_session_manager, from_session_manager) = mpsc::unbounded_channel();
        let (to_pool_manager, from_pool_manager) = mpsc::unbounded_channel();
        let (to_consensus_manager, from_consensus_manager) = mpsc::unbounded_channel();

        let session_manager = StromSessionManager::new(from_session_manager);

        let num_active_peers = Arc::new(AtomicUsize::new(0));
        let handle = StromNetworkHandle::new(to_handle);

        let num_active_peers = Arc::new(AtomicUsize::new(0));

        let manager = Self {
            handle,
            from_handle: UnboundedReceiverStream::new(from_handle),
            session_manager,
            to_pool_manager: Some(to_pool_manager),
            to_consensus_manager: Some(to_consensus_manager),
            num_active_peers
        };

        manager.start().await;
    }
    */
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
