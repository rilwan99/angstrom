use std::task::{Poll, Waker};

use ethers_core::types::transaction::eip712::{EIP712Domain, TypedData};
use futures::{Stream, StreamExt};
use jsonrpsee::{
    proc_macros::rpc, server::ServerHandle, PendingSubscriptionSink, SubscriptionSink
};
use jsonrpsee_core::server::SubscriptionMessage;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[cfg(feature = "subscription")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionKind {
    /// New best sealed bundle seen by this guard
    SealedBundle,
    /// New cow transactions that this guard has seen
    CowTransactions
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionResult {
    SealedBundle(Vec<shared::SealedBundle>),
    CowTransaction(Vec<TypedData>)
}

#[rpc(server, namespace = "guard")]
#[async_trait::async_trait]
pub trait GuardApi {
    #[method(name = "SubmitTransaction")]
    async fn submit_eip712(&self, meta_tx: TypedData) -> bool;

    /// Create an ethereum subscription for the given params
    #[cfg(feature = "subscription")]
    #[subscription(
        name = "subscribe" => "subscription",
        unsubscribe = "unsubscribe",
        item = reth_rpc_types::pubsub::SubscriptionResult
    )]
    async fn subscribe(&self, kind: SubscriptionKind) -> jsonrpsee::core::SubscriptionResult;
}

pub enum Submission {
    Submission(TypedData),
    #[cfg(feature = "subscription")]
    Subscription(SubscriptionKind, Sender<SubscriptionResult>)
}

pub struct SubmissionServer {
    /// The handle is server: Server<Stack<CorsLayer, Identity>>,
    handle:      ServerHandle,
    // so gay
    submissions: Mutex<Vec<Submission>>,
    waker:       Option<Waker>
}

impl SubmissionServer {
    pub fn new(handle: ServerHandle) -> Self {
        Self { handle, submissions: Mutex::new(vec![]), waker: None }
    }
}

#[async_trait::async_trait]
impl GuardApiServer for SubmissionServer {
    async fn submit_eip712(&self, meta_tx: TypedData) -> bool {
        let mut lock = self.submissions.lock();
        lock.push(Submission::Submission(meta_tx));

        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
        true
    }

    #[cfg(feature = "subscription")]
    async fn subscribe(
        &self,
        pending: PendingSubscriptionSink,
        kind: SubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        let sink = pending.accept().await?;
        let (tx, rx) = channel(5);
        let mut lock = self.submissions.lock();
        lock.push(Submission::Subscription(kind, tx));
        tokio::spawn(async move { pipe_from_stream(sink, ReceiverStream::new(rx)).await });

        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }

        Ok(())
    }
}

/// Pipes all stream items to the subscription sink.
#[cfg(feature = "subscription")]
async fn pipe_from_stream<T, St>(
    sink: SubscriptionSink,
    mut stream: St
) -> Result<(), jsonrpsee::core::Error>
where
    St: Stream<Item = T> + Unpin,
    T: Serialize
{
    loop {
        tokio::select! {
            _ = sink.closed() => {
                // connection dropped
                break Ok(())
            },
            maybe_item = stream.next() => {
                let item = match maybe_item {
                    Some(item) => item,
                    None => {
                        // stream ended
                        break  Ok(())
                    },
                };
                let msg = SubscriptionMessage::from_json(&item)?;
                if sink.send(msg).await.is_err() {
                    break Ok(());
                }
            }
        }
    }
}

impl Stream for SubmissionServer {
    type Item = Vec<Submission>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.handle.is_stopped() {
            return Poll::Ready(None)
        }

        self.waker = Some(cx.waker().clone());
        let mut lock = self.submissions.lock();
        Poll::Ready(Some(lock.drain(..).collect()))
    }
}
