use std::task::Poll;

use ethers_core::types::transaction::eip712::EIP712Domain;
use futures_util::Future;
use revm::{db::{CacheDB, DatabaseRef, EmptyDB}, EVM, Database};
use revm_primitives::EVMResult;
use tokio::sync::{mpsc::{UnboundedReceiver, UnboundedSender}, oneshot::Sender};
use crate::{sim::SimResult, executor::{ThreadPool, TaskKind}, Simulator, TransactionType};
use ethers_middleware::Middleware;


/// revm state handler
pub struct Revm<M: Middleware + DatabaseRef + 'static> {
    transaction_rx: UnboundedReceiver<TransactionType>,
    threadpool: ThreadPool,
    cache_db: CacheDB<EmptyDB>,
    evm: EVM<CacheDB<M>>
}

impl<M> Revm<M> 
where
    M: Middleware + DatabaseRef
{
    pub fn new(transaction_rx: UnboundedReceiver<TransactionType>, evm_db: CacheDB<M>) -> Self {
        let mut evm = EVM::new();
        evm.database(evm_db);
        Self { transaction_rx, threadpool: ThreadPool::new(), cache_db: CacheDB::new(EmptyDB {}), evm }
    }


    /// handles incoming transactions from clients
    fn handle_incoming_tx(&self, tx_type: TransactionType) {
        let _ = match tx_type {
            TransactionType::Single(tx, sender) => {
                let fut = async move { self.simulate_single_tx(tx, sender) };
                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Default);
            },
            TransactionType::Bundle(tx, sender) => {
                //self.threadpool.spawn_task_as(, TaskKind::Blocking);
            },
        };
    }


    fn simulate_single_tx(&mut self, tx: EIP712Domain, sender: Sender<SimResult>) {
        if let Ok(result) = self.evm.transact() {
            //let accounts = result.
            sender.send(SimResult {});
        } else {
            sender.send(SimResult {});
        }
    }

}



impl<M> Future for Revm<M>
where
    M: Middleware + DatabaseRef + Unpin
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        
        loop {
            if let Poll::Ready(poll_tx) = this.transaction_rx.poll_recv(cx) {
                match poll_tx {
                    Some(tx) => this.handle_incoming_tx(tx),
                    None => return Poll::Ready(())
                }
            }
            return Poll::Pending
        }
    }
}







