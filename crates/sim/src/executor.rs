use std::{future::Future, task::Poll};
use futures_util::pin_mut;
use tokio::{runtime::Handle, task::JoinHandle, sync::mpsc::UnboundedReceiver};

/// executes tasks on the runtime
/// used for a thread pool for the simulator
#[derive(Clone)]
pub(crate) struct SimThreadPool {
    handle: Handle,
    reciever: UnboundedReceiver<TransactionType>
    /// id for callback
    /// cache db
}

impl SimThreadPool {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        Self { handle: runtime.handle().clone() }
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

impl Future for SimThreadPool {
    
}






pub(crate) enum TaskKind {
    Default,
    Blocking
}