use std::{
    net::SocketAddr,
    ops::{Deref, DerefMut},
    task::{Poll, Waker}
};

use ethers_core::types::transaction::eip712::{EIP712Domain, TypedData};
use futures::{Stream, StreamExt};
use hyper::{http::HeaderValue, Method};
use jsonrpsee::{
    proc_macros::rpc, server::ServerHandle, PendingSubscriptionSink, SubscriptionSink
};
use jsonrpsee_core::server::SubscriptionMessage;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tower::{
    layer::{
        util::{Identity, Stack},
        Layer
    },
    ServiceBuilder
};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

/// Error thrown when parsing cors domains went wrong
#[derive(Debug, thiserror::Error)]
pub(crate) enum CorsDomainError {
    #[error("{domain} is an invalid header value")]
    InvalidHeader { domain: String },
    #[error("Wildcard origin (`*`) cannot be passed as part of a list: {input}")]
    WildCardNotAllowed { input: String }
}

/// Creates a [CorsLayer] from the given domains
pub(crate) fn create_cors_layer(http_cors_domains: &str) -> Result<CorsLayer, CorsDomainError> {
    let cors = match http_cors_domains.trim() {
        "*" => CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_origin(Any)
            .allow_headers(Any),
        _ => {
            let iter = http_cors_domains.split(',');
            if iter.clone().any(|o| o == "*") {
                return Err(CorsDomainError::WildCardNotAllowed {
                    input: http_cors_domains.to_string()
                })
            }

            let origins = iter
                .map(|domain| {
                    domain
                        .parse::<HeaderValue>()
                        .map_err(|_| CorsDomainError::InvalidHeader { domain: domain.to_string() })
                })
                .collect::<Result<Vec<HeaderValue>, _>>()?;

            let origin = AllowOrigin::list(origins);
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(origin)
                .allow_headers(Any)
        }
    };
    Ok(cors)
}

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
    SealedBundle(shared::SealedBundle),
    CowTransaction(TypedData)
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

pub struct SubmissionServerConfig {
    addr:         SocketAddr,
    cors_domains: String
}

pub struct SubmissionServer {
    handle:   ServerHandle,
    receiver: ReceiverStream<Submission>
}

impl Deref for SubmissionServer {
    type Target = ReceiverStream<Submission>;

    fn deref(&self) -> &Self::Target {
        &self.receiver
    }
}

impl DerefMut for SubmissionServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.receiver
    }
}

pub struct SubmissionServerInner {
    sender: Sender<Submission>
}

impl SubmissionServerInner {
    pub async fn new(config: SubmissionServerConfig) -> anyhow::Result<SubmissionServer> {
        let SubmissionServerConfig { addr, cors_domains } = config;

        let (tx, rx) = channel(10);

        let middleware: ServiceBuilder<Stack<CorsLayer, Identity>> =
            tower::ServiceBuilder::new().layer(create_cors_layer(&cors_domains)?);

        let server = jsonrpsee::server::ServerBuilder::default()
            .set_middleware(middleware)
            .build(addr)
            .await?;

        let sub_server = Self { sender: tx };

        let handle = server.start(sub_server.into_rpc());
        Ok(SubmissionServer { receiver: ReceiverStream::new(rx), handle })
    }
}

#[async_trait::async_trait]
impl GuardApiServer for SubmissionServerInner {
    async fn submit_eip712(&self, meta_tx: TypedData) -> bool {
        if self.sender.send(Submission::Submission(meta_tx)).await.is_err() {
            // just for testing
            panic!("failed to send a new eip712 tx");
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
        if self.sender.send(Submission::Subscription(kind, tx)).await.is_err() {
            // just for testing
            panic!("failed to subscribe to a stream");
        }
        tokio::spawn(async move { pipe_from_stream(sink, ReceiverStream::new(rx)).await });

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
