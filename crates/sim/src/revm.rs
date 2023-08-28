use std::{task::Poll, sync::Arc};
use ethers_core::types::transaction::eip712::EIP712Domain;
use futures_util::Future;
use revm::{db::{CacheDB, DatabaseRef, EmptyDB}, EVM, Database};
use revm_primitives::EVMResult;
use tokio::sync::{mpsc::{UnboundedReceiver, UnboundedSender}, oneshot::Sender};
use parking_lot::RwLock;
use crate::{sim::SimResult, executor::{ThreadPool, TaskKind}, Simulator, TransactionType};
use ethers_middleware::Middleware;
use crate::state::RevmState;


/// revm state handler
pub struct Revm<M: Middleware + 'static> {
    transaction_rx: UnboundedReceiver<TransactionType>,
    threadpool: ThreadPool,
    state: Arc<RwLock<RevmState<M>>>
}

impl<M> Revm<M> 
where
    M: Middleware
{
    pub fn new(transaction_rx: UnboundedReceiver<TransactionType>, evm_db: M, max_bytes: usize) -> Self {
        Self { transaction_rx, threadpool: ThreadPool::new(), state: Arc::new(RwLock::new(RevmState::new(evm_db, max_bytes))) }
    }


    /// handles incoming transactions from clients
    fn handle_incoming_tx(&mut self, tx_type: TransactionType) {
        let handle = self.threadpool.handle.clone();
        let state = self.state.clone();
        let _ = match tx_type {
            TransactionType::Single(tx, sender) => {
                let fut = async move { RevmState::simulate_single_tx(state.clone(), tx, sender) };
                let _ = ThreadPool::spawn_task_as(handle, fut, TaskKind::Default);
            },
            TransactionType::Bundle(tx, sender) => {
                //self.threadpool.spawn_task_as(, TaskKind::Blocking);
            },
        };
    }
}


impl<M> Future for Revm<M>
where
    M: Middleware + Unpin
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        
        while let Poll::Ready(poll_tx) = this.transaction_rx.poll_recv(cx) {
            match poll_tx {
                Some(tx) => this.handle_incoming_tx(tx),
                None => return Poll::Ready(())
            }
        }
        return Poll::Pending
    }
}
