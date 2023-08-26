use std::{collections::VecDeque, task::Poll};

use tokio::sync::mpsc::Sender;
// use park
use ethers_core::types::transaction::eip712::{EIP712Domain, Eip712, Eip712DomainType};
use futures::Stream;
use jsonrpsee::{
    core::RpcResult,
    proc_macros::rpc,
    server::{IdProvider, Server, ServerHandle},
    Methods, RpcModule
};
use parking_lot::Mutex;
use tower::layer::util::{Identity, Stack};
use tower_http::cors::CorsLayer;

#[rpc(server, namespace = "guard")]
#[async_trait::async_trait]
pub trait GuardApi {
    #[method(name = "SubmitTransaction")]
    async fn submit_eip712(&self, meta_tx: EIP712Domain) -> bool;
    #[method(name = "SubmitSearcherTx")]
    async fn submit_searcher_tx(&self, meta_tx: EIP712Domain) -> bool;
}

pub enum Submissions { 
    CexDex(EIP712Domain),
    Regular(EIP712Domain),

}

pub struct SubmissionServer {
    // server: Server<Stack<CorsLayer, Identity>>,
    handle:  ServerHandle,
    sender: Sender<Submissions>
}

impl SubmissionServer {
    pub fn new(handle: ServerHandle, sender: Sender<Submissions>) -> Self {
        Self {
            handle,
            sender
        }
    }
}

#[async_trait::async_trait]
impl GuardApiServer for SubmissionServer {
    async fn submit_eip712(&self, meta_tx: EIP712Domain) -> bool {
        self.sender.send(Submissions::Regular(meta_tx)).await;
        true
    }


    async fn submit_searcher_tx(&self, meta_tx: EIP712Domain) -> bool {
        self.sender.send(Submissions::CexDex(meta_tx)).await;
        true
    }
}

impl Future for SubmissionServer {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Item> {
        todo!()
    }

}
