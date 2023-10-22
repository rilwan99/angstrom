use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use common::PollExt;
use ethers_core::types::transaction::eip712::TypedData;
use futures::{Stream, StreamExt};
use guard_types::on_chain::{Signature, SubmittedOrder, UserOrder, VanillaBundle};
use hyper::{http::HeaderValue, Method};
use jsonrpsee::{
    proc_macros::rpc, server::ServerHandle, PendingSubscriptionSink, SubscriptionSink
};
use jsonrpsee_core::{server::SubscriptionMessage, RpcResult};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tower::{
    layer::util::{Identity, Stack},
    ServiceBuilder
};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tracing::info;

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
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionKind {
    /// Sends the new best bundle it has been able to build
    NewBestBundle,
    /// Sends a pre-proposal upon receiving it
    PreProposal,
    /// Send a pre-proposal upon receiving it, but only if it is better than the
    /// current best
    NewBestPreProposal,
    /// Sends the proposal upon receiving it from the proposer
    Proposal,
    ///  New best searcher order this guard has received
    NewBestSearcherOrder,
    /// New limit order this guard has received
    LimitOrder
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionResult {
    /// Vanilla Bundle
    Bundle(Arc<VanillaBundle>),
    /// Preprosal
    PreProposal(Arc<PreProposal>),
    /// Simmed User orders
    SearcherOrder(Arc<SubmittedOrder>)
}

#[rpc(server, client, namespace = "guard")]
#[async_trait::async_trait]
pub trait GuardSubmitApi {
    #[method(name = "SubmitOrder")]
    async fn submit_order(&self, signature: Signature, meta_tx: Order) -> RpcResult<bool>;
}

#[rpc(server, namespace = "guard_sub")]
#[async_trait::async_trait]
pub trait GuardSubscribeApi {
    /// Create an ethereum subscription for the given params
    #[subscription( name = "subscribe" => "subscription",
        unsubscribe = "unsubscribe",
        item = SubscriptionResult
    )]
    async fn subscribe_to_data(
        &self,
        kind: SubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult;
}

#[derive(Debug)]
pub enum Submission {
    UserOrder(SubmittedOrder),
    Subscription(SubscriptionKind, Sender<SubscriptionResult>)
}

pub struct SubmissionServerConfig {
    pub addr:                SocketAddr,
    pub cors_domains:        String,
    pub allow_subscriptions: bool
}

pub struct SubmissionServer {
    handle:               ServerHandle,
    receiver:             ReceiverStream<Submission>,
    server_subscriptions: HashMap<SubscriptionKind, Vec<Sender<SubscriptionResult>>>
}

impl SubmissionServer {
    /// used to share new txes with externally subscribed users
    pub fn on_new_user_tx(&mut self, tx: Arc<SubmittedOrder>) {
        self.server_subscriptions
            .entry(SubscriptionKind::CowTransactions)
            .or_default()
            .retain(|sender| {
                sender
                    .try_send(SubscriptionResult::UserOrder(tx.clone()))
                    .is_ok()
            });
    }

    /// used to share new bundles with externally subscribed users
    pub fn on_new_prepropose(&mut self, bundle: Arc<VanillaBundle>) {
        self.server_subscriptions
            .entry(SubscriptionKind::BestBundles)
            .or_default()
            .retain(|sender| {
                sender
                    .try_send(SubscriptionResult::Bundle(bundle.clone()))
                    .is_ok()
            });
    }
}

impl Stream for SubmissionServer {
    type Item = Submission;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_next_unpin(cx).filter_map(|f| {
            f.map(|f| match f {
                Submission::Subscription(sk, sender) => {
                    self.server_subscriptions
                        .entry(sk)
                        .or_default()
                        .push(sender);
                    None
                }
                rest @ _ => Some(rest)
            })
        })
    }
}

#[derive(Debug, Clone)]
pub struct SubmissionServerInner {
    sender: Sender<Submission>
}

impl SubmissionServerInner {
    pub async fn new(config: SubmissionServerConfig) -> anyhow::Result<SubmissionServer> {
        let SubmissionServerConfig { cors_domains, addr, allow_subscriptions } = config;

        let (tx, rx) = channel(10);

        let middleware: ServiceBuilder<Stack<CorsLayer, Identity>> =
            tower::ServiceBuilder::new().layer(create_cors_layer(&cors_domains)?);

        let server = jsonrpsee::server::ServerBuilder::default()
            .set_middleware(middleware)
            .build(addr)
            .await?;

        let sub_server = Self { sender: tx };

        let mut methods = GuardSubmitApiServer::into_rpc(sub_server.clone());
        if allow_subscriptions {
            methods.merge(GuardSubscribeApiServer::into_rpc(sub_server))?;
        }

        let handle = server.start(methods);
        Ok(SubmissionServer {
            receiver: ReceiverStream::new(rx),
            handle,
            server_subscriptions: HashMap::default()
        })
    }
}

#[async_trait::async_trait]
impl GuardSubmitApiServer for SubmissionServerInner {
    async fn submit_order(&self, signature: Signature, meta_tx: TypedData) -> RpcResult<bool> {
        info!(?meta_tx, "new user submission");
        let Ok(user_tx): Result<UserOrder, _> = serde_json::from_value(serde_json::Value::Object(
            serde_json::Map::from_iter(meta_tx.message)
        )) else {
            return Ok(false)
        };

        if self
            .sender
            .send(Submission::UserOrder(SubmittedOrder { signature, details: user_tx }))
            .await
            .is_err()
        {
            // just for testing
            panic!("failed to send a new eip712 tx");
        }
        Ok(true)
    }
}

#[async_trait::async_trait]
impl GuardSubscribeApiServer for SubmissionServerInner {
    async fn subscribe_to_data(
        &self,
        pending: PendingSubscriptionSink,
        kind: SubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        info!(?pending, ?kind, "subscription request");
        let sink = pending.accept().await?;
        let (tx, rx) = channel(5);
        if self
            .sender
            .send(Submission::Subscription(kind, tx))
            .await
            .is_err()
        {
            // just for testing
            panic!("");
        }
        tokio::spawn(async move { pipe_from_stream(sink, ReceiverStream::new(rx)).await });

        Ok(())
    }
}

/// Pipes all stream items to the subscription sink.
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

#[cfg(test)]
pub mod test {

    use jsonrpsee::http_client::HttpClientBuilder;

    use super::*;

    #[tokio::test]
    async fn can_submit_data() -> anyhow::Result<()> {
        let fake_addr = "127.0.0.1:6969".parse()?;

        let server_config = SubmissionServerConfig {
            addr:                fake_addr,
            cors_domains:        "*".into(),
            allow_subscriptions: false
        };
        let server = SubscriptionServerInner::new(server_config).await.unwrap();
        let client = HttpClientBuilder::default()
            .build("http://127.0.0.1:6969")
            .unwrap();

        let typed_data = get_typed_data();

        let res = client.submit_arb_tx(meta_tx.clone()).await;

        let Submssion::ArbTx(server_res) = server.next().await.unwrap() else {
            panic!("wrong value received")
        };

        assert_eq!(meta_tx, server_res.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_subscription() {
        let fake_addr = "127.0.0.1:6970".parse()?;

        let server_config = SubmissionServerConfig {
            addr:                fake_addr,
            cors_domains:        "*".into(),
            allow_subscriptions: false
        };
        let server = SubscriptionServerInner::new(server_config).await.unwrap();
        let client = HttpClientBuilder::default()
            .build("http://127.0.0.1:6970")
            .unwrap();

        let sub_client = client
            .subscribe(SubscriptionKind::CowTransactions)
            .await
            .unwrap();
        let Some(Submission::Subscription(kind, sender)) = server.next().await else { panic!() };
        let payload = SubscriptionResult::CowTransaction(Arc::new(vec![Eip712(get_typed_data())]));

        sender.send(payload.clone()).unwrap();

        let res = sub_client.next().await.unwrap().unwrap();
        assert_eq!(payload, res);
    }

    fn get_typed_data() -> TypedData {
        let json = serde_json::json!({
          "types": {
            "EIP712Domain": [
              {
                "name": "name",
                "type": "string"
              },
              {
                "name": "version",
                "type": "string"
              },
              {
                "name": "chainId",
                "type": "uint256"
              },
              {
                "name": "verifyingContract",
                "type": "address"
              }
            ],
            "OrderComponents": [
              {
                "name": "offerer",
                "type": "address"
              },
              {
                "name": "zone",
                "type": "address"
              },
              {
                "name": "offer",
                "type": "OfferItem[]"
              },
              {
                "name": "startTime",
                "type": "uint256"
              },
              {
                "name": "endTime",
                "type": "uint256"
              },
              {
                "name": "zoneHash",
                "type": "bytes32"
              },
              {
                "name": "salt",
                "type": "uint256"
              },
              {
                "name": "conduitKey",
                "type": "bytes32"
              },
              {
                "name": "counter",
                "type": "uint256"
              }
            ],
            "OfferItem": [
              {
                "name": "token",
                "type": "address"
              }
            ],
            "ConsiderationItem": [
              {
                "name": "token",
                "type": "address"
              },
              {
                "name": "identifierOrCriteria",
                "type": "uint256"
              },
              {
                "name": "startAmount",
                "type": "uint256"
              },
              {
                "name": "endAmount",
                "type": "uint256"
              },
              {
                "name": "recipient",
                "type": "address"
              }
            ]
          },
          "primaryType": "OrderComponents",
          "domain": {
            "name": "Seaport",
            "version": "1.1",
            "chainId": "1",
            "verifyingContract": "0x00000000006c3852cbEf3e08E8dF289169EdE581"
          },
          "message": {
            "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "offer": [
              {
                "token": "0xA604060890923Ff400e8c6f5290461A83AEDACec"
              }
            ],
            "startTime": "1658645591",
            "endTime": "1659250386",
            "zone": "0x004C00500000aD104D7DBd00e3ae0A5C00560C00",
            "zoneHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "salt": "16178208897136618",
            "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
            "totalOriginalConsiderationItems": "2",
            "counter": "0"
          }
        }
                );

        serde_json::from_value(json).unwrap()
    }
}
