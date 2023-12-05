use std::{
    collections::HashMap,
    num::NonZeroUsize,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use futures::{stream::FuturesUnordered, Future, FutureExt, StreamExt};
use guard_eth::manager::EthEvent;
use guard_types::{
    orders::{
        GetPooledOrders, OrderId, OrderLocation, OrderOrigin, OrderPriorityData, Orders,
        PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
        SearcherPriorityData, ValidatedOrder, ValidationResults
    },
    primitive::PoolId,
    rpc::*
};
use order_pool::{OrderPool, OrderPoolInner};
use reth_primitives::{PeerId, TxHash, B256};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot
};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
use validation::order::OrderValidator;

use crate::{LruCache, NetworkOrderEvent, RequestResult, StromNetworkEvent, StromNetworkHandle};
/// Cache limit of transactions to keep track of for a single peer.
const PEER_ORDER_CACHE_LIMIT: usize = 1024 * 10;

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

impl OrderPool for PoolHandle {
    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder = EcRecoveredComposableLimitOrder;
    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder = EcRecoveredComposableSearcherOrder;
    /// The transaction type of the limit order pool
    type LimitOrder = EcRecoveredLimitOrder;
    /// The transaction type of the searcher order pool
    type SearcherOrder = EcRecoveredSearcherOrder;

    fn get_pooled_orders_by_hashes(
        &self,
        tx_hashes: Vec<TxHash>,
        limit: Option<usize>
    ) -> Vec<PooledOrder> {
        todo!()
    }
}

//TODO: Tmrw clean up + finish pool manager + pool inner
//TODO: Add metrics + events
pub struct PoolManager<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    /// The order pool
    pool:                 OrderPoolInner<L, CL, S, CS, V>,
    /// Network access.
    _network:             StromNetworkHandle,
    /// Subscriptions to all the strom-network related events.
    ///
    /// From which we get all new incoming order related messages.
    strom_network_events: UnboundedReceiverStream<StromNetworkEvent>,
    /// Ethereum updates stream that tells the pool manager about orders that
    /// have been filled  
    _eth_network_events:  UnboundedReceiverStream<EthEvent>,
    /// Send half for the command channel.
    command_tx:           UnboundedSender<OrderCommand>,
    /// receiver half of the commands to the pool manager
    command_rx:           UnboundedReceiverStream<OrderCommand>,
    /// Order fetcher to handle inflight and missing order requests.
    _order_fetcher:       OrderFetcher,
    /// Incoming pending transactions from the pool that should be propagated to
    /// the network
    _pending_orders:      ReceiverStream<TxHash>,
    /// All currently pending orders grouped by peers.
    _orders_by_peers:     HashMap<TxHash, Vec<PeerId>>,
    /// Incoming events from the ProtocolManager.
    order_events:         UnboundedReceiverStream<NetworkOrderEvent>,
    /// All the connected peers.
    peers:                HashMap<PeerId, StromPeer>
}

impl<L, CL, S, CS, V> PoolManager<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    pub fn new(
        _pool: OrderPoolInner<L, CL, S, CS, V>,
        _network: StromNetworkHandle,
        _from_network: UnboundedReceiver<NetworkOrderEvent>
    ) {
        todo!()
    }
}

impl<L, CL, S, CS, V> PoolManager<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    /// Returns a new handle that can send commands to this type.
    pub fn handle(&self) -> PoolHandle {
        PoolHandle { manager_tx: self.command_tx.clone() }
    }

    //TODO
    fn on_command(&mut self, cmd: OrderCommand) {
        match cmd {
            OrderCommand::PropagateOrders(orders) => {}
            OrderCommand::PropagateOrdersTo(orders, peer_id) => {}
        }
    }

    //TODO
    fn on_network_order_event(&mut self, event: NetworkOrderEvent) {
        match event {
            NetworkOrderEvent::IncomingOrders { peer_id, orders } => {}
        }
    }

    fn on_network_event(&mut self, event: StromNetworkEvent) {
        match event {
            StromNetworkEvent::SessionEstablished { peer_id, client_version } => {
                // insert a new peer into the peerset
                self.peers.insert(
                    peer_id,
                    StromPeer {
                        orders: LruCache::new(NonZeroUsize::new(PEER_ORDER_CACHE_LIMIT).unwrap()),
                        //request_tx: messages,
                        client_version
                    }
                );
            }
            StromNetworkEvent::SessionClosed { peer_id, .. } => {
                // remove the peer
                self.peers.remove(&peer_id);
            }

            _ => {}
        }
    }
}

impl<L, CL, S, CS, V> Future for PoolManager<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator + Unpin
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // drain network/peer related events
        while let Poll::Ready(Some(event)) = this.strom_network_events.poll_next_unpin(cx) {
            this.on_network_event(event);
        }

        // drain commands
        while let Poll::Ready(Some(cmd)) = this.command_rx.poll_next_unpin(cx) {
            this.on_command(cmd);
        }

        // drain incoming transaction events
        while let Poll::Ready(Some(event)) = this.order_events.poll_next_unpin(cx) {
            this.on_network_order_event(event);
        }

        if let Poll::Ready(_) = this.pool.poll_unpin(cx) {
            return Poll::Ready(())
        }

        Poll::Pending
    }
}

#[derive(Debug)]
pub enum OrderCommand {
    PropagateOrders(Vec<TxHash>),
    PropagateOrdersTo(Vec<TxHash>, PeerId)
}

/// All events related to orders emitted by the network.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum NetworkTransactionEvent {
    /// Received list of transactions from the given peer.
    ///
    /// This represents transactions that were broadcasted to use from the peer.
    IncomingOrders { peer_id: PeerId, msg: Orders },
    /// Incoming `GetPooledOrders` request from a peer.
    GetPooledOrders {
        peer_id:  PeerId,
        request:  GetPooledOrders,
        response: oneshot::Sender<RequestResult<Orders>>
    }
}

/// Tracks a single peer
#[derive(Debug)]
struct StromPeer {
    /// Keeps track of transactions that we know the peer has seen.
    #[allow(dead_code)]
    orders:         LruCache<B256>,
    /// A communication channel directly to the peer's session task.
    //request_tx:     PeerRequestSender,
    /// negotiated version of the session.
    /// The peer's client version.
    #[allow(unused)]
    client_version: Arc<str>
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
