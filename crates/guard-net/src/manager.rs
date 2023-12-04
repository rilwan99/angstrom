use std::sync::{atomic::AtomicUsize, Arc};

use reth_metrics::common::mpsc::UnboundedMeteredSender;

use crate::{StromNetworkEvent, StromNetworkHandle, StromSessionManager};

#[allow(dead_code)]
pub struct StromNetworkManager {
    handle:               StromNetworkHandle,
    from_handle:          UnboundedMeteredSender<StromNetworkEvent>,
    session_manager:      StromSessionManager,
    to_pool_manager:      Option<UnboundedMeteredSender<StromNetworkEvent>>,
    to_consensus_manager: Option<UnboundedMeteredSender<StromNetworkEvent>>,

    /// This is updated via internal events and shared via `Arc` with the
    /// [`NetworkHandle`] Updated by the `NetworkWorker` and loaded by the
    /// `NetworkService`.
    num_active_peers: Arc<AtomicUsize>
}
