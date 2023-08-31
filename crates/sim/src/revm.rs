use std::{sync::Arc, task::Poll};

use futures_util::Future;
use parking_lot::RwLock;
use reth_db::mdbx::WriteMap;
use tokio::{runtime::Handle, sync::mpsc::UnboundedReceiver};

use crate::{
    errors::SimError,
    executor::{TaskKind, ThreadPool},
    state::RevmState,
    TransactionType,
};

/// revm state handler
pub struct Revm {
    transaction_rx: UnboundedReceiver<TransactionType>,
    threadpool: ThreadPool,
    state: Arc<RwLock<RevmState>>,
}

impl Revm {
    pub fn new(
        transaction_rx: UnboundedReceiver<TransactionType>,
        evm_db: Arc<reth_db::mdbx::Env<WriteMap>>,
        max_bytes: usize,
    ) -> Result<Self, SimError> {
        let threadpool = ThreadPool::new()?;
        Ok(Self {
            transaction_rx,
            threadpool,
            state: Arc::new(RwLock::new(RevmState::new(evm_db, max_bytes))),
        })
    }

    pub fn get_threadpool_handle(&self) -> Handle {
        self.threadpool.runtime.handle().clone()
    }

    /// handles incoming transactions from clients
    fn handle_incoming_tx(&mut self, tx_type: TransactionType) {
        let state = self.state.clone();
        // why are we assigning if no value is returned
        match tx_type {
            TransactionType::Single(tx, sender) => {
                let fut = async move { RevmState::simulate_single_tx(state.clone(), tx, sender) };
                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Default);
            }
            TransactionType::Bundle(tx, sender) => {
                let fut = async move { RevmState::simulate_bundle(state.clone(), tx, sender) };
                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Blocking);
            }
        };
    }
}

impl Future for Revm {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        while let Poll::Ready(poll_tx) = this.transaction_rx.poll_recv(cx) {
            match poll_tx {
                Some(tx) => this.handle_incoming_tx(tx),
                None => return Poll::Ready(()),
            }
        }
        return Poll::Pending;
    }
}
