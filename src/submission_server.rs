use std::task::{Poll, Waker};

// use park
use ethers_core::types::transaction::eip712::EIP712Domain;
use futures::Stream;
use jsonrpsee::{proc_macros::rpc, server::ServerHandle};
use parking_lot::Mutex;
use tokio::sync::mpsc::Sender;

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
    Regular(EIP712Domain)
}

pub struct SubmissionServer {
    /// The handle is server: Server<Stack<CorsLayer, Identity>>,
    handle:      ServerHandle,
    // so gay
    submissions: Mutex<Vec<Submissions>>,
    waker:       Option<&'static Waker>
}

impl SubmissionServer {
    pub fn new(handle: ServerHandle, sender: Sender<Submissions>) -> Self {
        Self { handle, submissions: Mutex::new(vec![]), waker: None }
    }
}

#[async_trait::async_trait]
impl GuardApiServer for SubmissionServer {
    async fn submit_eip712(&self, meta_tx: EIP712Domain) -> bool {
        let mut lock = self.submissions.lock();
        lock.push(Submissions::Regular(meta_tx));

        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
        true
    }

    async fn submit_searcher_tx(&self, meta_tx: EIP712Domain) -> bool {
        let mut lock = self.submissions.lock();
        lock.push(Submissions::CexDex(meta_tx));

        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }

        true
    }
}

impl Stream for SubmissionServer {
    type Item = Vec<Submissions>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        self.waker = Some(cx.waker());

        let mut lock = self.submissions.lock();
        Poll::Ready(Some(lock.drain(..).collect()))
    }
}
