use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use futures::{stream::FuturesUnordered, Future};
use guard_eth::manager::EthEvent;
use guard_types::orders::GetPooledOrders;
use order_pool::traits::OrderPool;
use reth_network::peers::Peer;
use reth_primitives::{PeerId, TxHash};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};

use crate::{types::events::StromNetworkEvent, NetworkOrderEvent, StromNetworkHandle};

/// Api to interact with [`PoolManager`] task.
#[derive(Debug, Clone)]
pub struct PoolHandle {
    #[allow(dead_code)]
    /// Command channel to the [`TransactionsManager`]
    manager_tx: UnboundedSender<OrderCommand>
}

impl PoolHandle {
    #[allow(dead_code)]
    fn send(&self, cmd: OrderCommand) {
        let _ = self.manager_tx.send(cmd);
    }
}

//TODO: Tmrw clean up + finish pool manager + pool inner
//TODO: Add metrics + events
pub struct PoolManager<Pool> {
    /// Access to the order pool
    _pool:                 Pool,
    /// Network access.
    _network:              StromNetworkHandle,
    /// Subscriptions to all the strom-network related events.
    ///
    /// From which we get all new incoming order related messages.
    _strom_network_events: UnboundedReceiverStream<StromNetworkEvent>,
    /// Ethereum updates stream that tells the pool manager about orders that
    /// have been filled  
    _eth_network_events:   UnboundedReceiverStream<EthEvent>,
    /// Send half for the command channel.
    command_tx:            UnboundedSender<OrderCommand>,
    /// receiver half of the commands to the pool manager
    _command_rx:           UnboundedReceiverStream<OrderCommand>,
    /// Order fetcher to handle inflight and missing order requests.
    _order_fetcher:        OrderFetcher,
    /// Incoming pending transactions from the pool that should be propagated to
    /// the network
    _pending_orders:       ReceiverStream<TxHash>,
    /// All currently pending orders grouped by peers.
    _orders_by_peers:      HashMap<TxHash, Vec<PeerId>>,
    /// Incoming events from the ProtocolManager.
    _order_events:         UnboundedReceiverStream<NetworkOrderEvent>,
    /// All the connected peers.
    _peers:                HashMap<PeerId, Peer>
}

impl<Pool: OrderPool> PoolManager<Pool> {
    pub fn new(
        _pool: Pool,
        _network: StromNetworkHandle,
        _from_network: UnboundedReceiver<NetworkOrderEvent>
    ) {
        todo!()
    }
}

impl<Pool> PoolManager<Pool>
where
    Pool: OrderPool
{
    /// Returns a new handle that can send commands to this type.
    pub fn handle(&self) -> PoolHandle {
        PoolHandle { manager_tx: self.command_tx.clone() }
    }
}

/// The type responsible for fetching missing orders from peers.
///
/// This will keep track of unique transaction hashes that are currently being
/// fetched and submits new requests on announced hashes.
#[derive(Debug, Default)]
struct OrderFetcher {
    /// All currently active requests for pooled transactions.
    _inflight_requests:               FuturesUnordered<GetPooledOrders>,
    /// Set that tracks all hashes that are currently being fetched.
    _inflight_hash_to_fallback_peers: HashMap<TxHash, Vec<PeerId>>
}

impl<Pool> Future for PoolManager<Pool>
where
    Pool: OrderPool + Unpin + 'static
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _this = self.get_mut();

        Poll::Pending
    }
}

#[derive(Debug)]
pub enum OrderCommand {
    PropagateOrder(TxHash),
    PropagateComposableOrder(TxHash),
    PropagateSearcherOrder(TxHash),
    PropagateOrdersTo(Vec<TxHash>, PeerId),
    PropagateComposableOrdersTo(Vec<TxHash>, PeerId),
    PropagateSearcherOrdersTo(Vec<TxHash>, PeerId)
}
