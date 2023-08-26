use std::{future::Future, task::Poll};
use futures_util::pin_mut;
use tokio::{runtime::Handle, task::JoinHandle, sync::{oneshot::Sender, mpsc::UnboundedReceiver}};

use crate::{revm::TransactionType, Simulator};

/// executes tasks on the runtime
/// used for a thread pool for the simulator
pub(crate) struct SimThreadPool<S: Simulator + 'static> {
    handle: Handle,
    transaction_rx: UnboundedReceiver<TransactionType>,
    //id: u64,
    sim: S
}

impl<S> SimThreadPool<S> 
where 
    S: Simulator,
{
    pub fn new(transaction_rx: UnboundedReceiver<TransactionType>, sim: S) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { handle: runtime.handle().clone(), transaction_rx, sim }
    }

    /// Spawns a regular task depending on the given [TaskKind]
    fn spawn_task_as<F>(&self, fut: F, task_kind: TaskKind) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = async move {
            pin_mut!(fut);
            let _ = fut.await;
        };

        self.spawn_on_rt(task, task_kind)
    }

    /// Spawns a future on the tokio runtime depending on the [TaskKind]
    fn spawn_on_rt<F>(&self, fut: F, task_kind: TaskKind) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        match task_kind {
            TaskKind::Default => self.handle.spawn(fut),
            TaskKind::Blocking => {
                let handle = self.handle.clone();
                self.handle.spawn_blocking(move || handle.block_on(fut))
            }
        }
    }
}

impl<S> Future for SimThreadPool<S> 
where
    S: Simulator + Unpin + 'static
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        
        loop {
            match this.transaction_rx.poll_recv(cx) {
                Poll::Ready(Some(transaction_type)) => { 
                    match transaction_type {
                        TransactionType::Single(transaction, tx) => {
                            //this.spawn_task_as(this.sim.run_sim(transaction, tx), TaskKind::Default);
                        },
                        TransactionType::Bundle(bundle, tx) =>  {
                            //this.spawn_task_as(this.sim.run_sim(bundle, tx), TaskKind::Default); // GIVE PRIORITY
                        },
                    }
                    ()
                },
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => break
            };
        }

        Poll::Pending
    }
}


pub(crate) enum TaskKind {
    Default,
    Blocking
}