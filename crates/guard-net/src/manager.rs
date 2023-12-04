use std::sync::{atomic::AtomicUsize, Arc};

use guard_types::orders::Orders;
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_rpc_types::PeerId;

use crate::{pool_manager::PoolHandle, StromNetworkEvent};

//TODO:
// 1) Implement the order pool manager
// 2) Implement the consensus manager
// 3)
#[derive(Debug)]
pub struct StromNetworkHandle {
    inner: Arc<StromNetworkInner>
}

#[derive(Debug)]
struct StromNetworkInner {
    num_active_peers: Arc<AtomicUsize>,
    to_manager_tx:    UnboundedMeteredSender<StromNetworkEvent>
}
/// All events related to orders emitted by the network.
#[derive(Debug)]
pub enum NetworkOrderEvent {
    IncomingOrders { peer_id: PeerId, orders: Vec<Orders> }
}

pub struct StromNetworkManager {
    inner:  Arc<StromNetworkInner>,
    handle: PoolHandle
}
