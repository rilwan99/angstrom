mod common;
mod config;
mod inner;
mod limit;
mod searcher;
pub mod traits;
mod validator;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use alloy_primitives::TxHash;
use config::PoolConfig;
use futures_util::{stream::FuturesUnordered, Future, Stream};
use guard_eth::manager::EthEvent;
use guard_types::{
    orders::{
        OrderOrigin, OrderPriorityData, PooledComposableOrder, PooledLimitOrder, PooledOrder,
        PooledSearcherOrder, SearcherPriorityData, ValidatedOrder
    },
    rpc::{
        EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
        EcRecoveredSearcherOrder
    }
};
pub use guard_utils::*;
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
use traits::OrderPool;
use validation::{order::OrderValidator, RevmClient};

#[derive(Clone)]
pub struct Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    pool: Arc<OrderPoolInner<L, CL, S, CS, V>>
}

impl<L, CL, S, CS, V> Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder<ValidationData = ValidatedOrder<L, OrderPriorityData>>,
    CL: PooledComposableOrder
        + PooledLimitOrder<ValidationData = ValidatedOrder<CL, OrderPriorityData>>,

    S: PooledSearcherOrder<ValidationData = ValidatedOrder<S, SearcherPriorityData>>,
    CS: PooledComposableOrder
        + PooledSearcherOrder<ValidationData = ValidatedOrder<CS, SearcherPriorityData>>,
    V: OrderValidator<
        LimitOrder = L,
        SearcherOrder = S,
        ComposableLimitOrder = CL,
        ComposableSearcherOrder = CS
    >
{
    pub fn new(validator: V, config: PoolConfig) -> Self {
        let pool = Arc::new(OrderPoolInner::new(validator, config));

        Self { pool }
    }

    pub fn inner(&self) -> &OrderPoolInner<L, CL, S, CS, V> {
        &self.pool
    }
}

//TODO: Tmrw, finish the impl of the pool handle that impls the Pool trait
// which will be the pool api

impl<L, CL, S, CS, V> OrderPool for Pool<L, CL, S, CS, V>
where
    L: PooledLimitOrder,
    CL: PooledComposableOrder + PooledLimitOrder,
    S: PooledSearcherOrder,
    CS: PooledComposableOrder + PooledSearcherOrder,
    V: OrderValidator
{
    type ComposableLimitOrder = CL;
    type ComposableSearcherOrder = CS;
    type LimitOrder = L;
    type SearcherOrder = S;
}
