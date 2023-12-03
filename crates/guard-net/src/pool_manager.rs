use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use futures::{stream::FuturesUnordered, Future};
use guard_eth::manager::EthEvent;
use guard_types::orders::{GetOrders, Orders};
use order_pool::traits::OrderPool;
use reth_interfaces::p2p::error::RequestResult;
use reth_network::peers::Peer;
use reth_primitives::{PeerId, TxHash};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot
};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
use tracing::log::trace;

use crate::{types::events::StromNetworkEvent, NetworkOrderEvent, StromProtocolHandle};

/// Api to interact with [`PoolManager`] task.
#[derive(Debug, Clone)]
pub struct PoolHandle {
    /// Command channel to the [`TransactionsManager`]
    manager_tx: UnboundedReceiver<OrderCommand>
}

impl PoolHandle {
    fn send(&self, cmd: OrderCommand) {
        let _ = self.manager_tx.send(cmd);
    }
}

//TODO: Tmrw clean up + finish pool manager + pool inner
//TODO: Add metrics + events
pub struct PoolManager<Pool> {
    /// Access to the order pool
    pool:                 Pool,
    /// Network access.
    network:              StromProtocolHandle,
    /// Subscriptions to all the strom-network related events.
    ///
    /// From which we get all new incoming order related messages.
    strom_network_events: UnboundedReceiverStream<StromNetworkEvent>,
    /// Ethereum updates stream that tells the pool manager about orders that
    /// have been filled  
    eth_network_events:   UnboundedReceiverStream<EthEvent>,
    /// Send half for the command channel.
    command_tx:           UnboundedSender<OrderCommand>,
    /// receiver half of the commands to the pool manager
    command_rx:           UnboundedReceiverStream<OrderCommand>,
    /// Order fetcher to handle inflight and missing order requests.
    order_fetcher:        OrderFetcher,
    /// Incoming pending transactions from the pool that should be propagated to
    /// the network
    pending_orders:       ReceiverStream<TxHash>,
    /// All currently pending orders grouped by peers.
    orders_by_peers:      HashMap<TxHash, Vec<PeerId>>,
    /// Incoming events from the ProtocolManager.
    order_events:         UnboundedReceiverStream<NetworkOrderEvent>,
    /// All the connected peers.
    peers:                HashMap<PeerId, Peer>
}

impl<Pool: OrderPool> PoolManager<Pool> {
    pub fn new(
        pool: Pool,
        network: StromProtocolHandle,
        from_network: UnboundedReceiver<NetworkOrderEvent>
    ) {
        let network_events = network.event_listener();
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
    inflight_requests:               FuturesUnordered<GetOrders>,
    /// Set that tracks all hashes that are currently being fetched.
    inflight_hash_to_fallback_peers: HashMap<TxHash, Vec<PeerId>>
}

impl<Pool> PoolManager<Pool>
where
    Pool: OrderPool + 'static
{
    /*#[inline]
    fn update_import_metrics(&self) {
        self.metrics.pending_pool_imports.set(self.pool_imports.len() as f64);
    }

    #[inline]
    fn update_request_metrics(&self) {
        self.metrics
            .inflight_transaction_requests
            .set(self.transaction_fetcher.inflight_requests.len() as f64);
    }*/

    /// Request handler for an incoming request for transactions
    fn on_get_pooled_orders(
        &mut self,
        peer_id: PeerId,
        request: GetOrders,
        response: oneshot::Sender<RequestResult<Orders>>
    ) {
        if let Some(peer) = self.peers.get_mut(&peer_id) {
            if self.network.tx_gossip_disabled() {
                let _ = response.send(Ok(Orders::default()));
                return
            }
            let orders = self.pool.get_pooled_orders(request.0, 8);

            // we sent a response at which point we assume that the peer is aware of the
            // transactions
            peer.transactions.extend(orders.iter().map(|tx| *tx.hash()));

            let resp = Orders(orders);
            let _ = response.send(Ok(resp));
        }
    }
}

impl<Pool> Future for PoolManager<Pool>
where
    Pool: OrderPool + Unpin + 'static
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

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
