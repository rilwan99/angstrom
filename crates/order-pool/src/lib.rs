mod common;
mod inner;
mod limit;
mod searcher;
mod traits;
mod validator;

use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll}
};

use alloy_primitives::TxHash;
pub use common::*;
use eth::manager::EthNetworkEvent;
use futures_util::{stream::FuturesUnordered, Future, Stream};
use guard_types::{
    orders::{
        ComposableLimitOrderValidation, ComposableSearcherOrderValidation, LimitOrderValidation,
        OrderOrigin, PooledComposableOrder, PooledLimitOrder, PooledOrder, PooledSearcherOrder,
        SearcherOrderValidation
    },
    rpc::{
        EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
        EcRecoveredSearcherOrder
    }
};
use inner::OrderPoolInner;
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_network::{peers::Peer, NetworkEvent, NetworkHandle};
use reth_primitives::PeerId;
use tokio::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
    oneshot
};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
use validation::{order::OrderValidator, RevmClient};

type DefaultOrderPool = OrderPool<
    EcRecoveredLimitOrder,
    EcRecoveredComposableLimitOrder,
    EcRecoveredSearcherOrder,
    EcRecoveredComposableSearcherOrder,
    RevmClient
>;

#[derive(Debug, Clone)]
pub struct OrderPoolHandle {
    sender: Sender<OrderPoolCommand>
}

#[derive(Debug, Clone)]
pub enum OrderPoolCommand {}

impl traits::OrderPool for OrderPoolHandle {
    type ComposableLimitOrder = EcRecoveredComposableLimitOrder;
    type ComposableSearcherOrder = EcRecoveredComposableSearcherOrder;
    type LimitOrder = EcRecoveredLimitOrder;
    type SearcherOrder = EcRecoveredSearcherOrder;
}

struct OrderPool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    inner: OrderPoolInner<L, CL, S, CS, V>,

    /// Ethereum Data updates
    eth_network_events:    Pin<Box<dyn Stream<Item = EthNetworkEvent>>>,
    /// allows for external services to access data.
    command_rx:            ReceiverStream<OrderPoolCommand>,
    /// Network access.
    network:               NetworkHandle,
    /// Subscriptions to all network related events.
    ///
    /// From which we get all new incoming transaction related messages.
    network_events:        UnboundedReceiverStream<NetworkEvent>,
    /// Transaction fetcher to handle inflight and missing transaction requests.
    // transaction_fetcher:   TransactionFetcher,
    /// All currently pending transactions grouped by peers.
    ///
    /// This way we can track incoming transactions and prevent multiple pool
    /// imports for the same transaction
    transactions_by_peers: HashMap<TxHash, Vec<PeerId>>,

    // /// Transactions that are currently imported into the `Pool`
    // pool_imports:          FuturesUnordered<PoolImportFuture>,
    // /// All the connected peers.
    peers: HashMap<PeerId, Peer>,
    // /// Incoming events from the [`NetworkManager`](crate::NetworkManager).
    // transaction_events:    UnboundedMeteredReceiver<NetworkTransactionEvent>,
    // /// TransactionsManager metrics
    // metrics:               TransactionsManagerMetrics
    // TODO: placeholder to avoid bad fmt
    _p:    ()
}

impl<L, CL, S, CS, V> Future for OrderPool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >,
    <L as PooledOrder>::ValidationData: LimitOrderValidation,
    <CL as PooledOrder>::ValidationData: ComposableLimitOrderValidation,
    <S as PooledOrder>::ValidationData: SearcherOrderValidation,
    <CS as PooledOrder>::ValidationData: ComposableSearcherOrderValidation
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
