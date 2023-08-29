use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll}
};

use futures_util::{
    future::{FusedFuture, Shared},
    pin_mut, FutureExt
};
use tokio::{
    runtime::Handle,
    sync::{mpsc::UnboundedReceiver, oneshot},
    task::JoinHandle
};

/// executes tasks on the runtime
/// used for a thread pool for the simulator
#[derive(Clone)]
pub(crate) struct ThreadPool {
    //TODO: why are we having the handle which is a ref
    // why don't we just take the runtime to avoid the clone at the start?
    pub handle: Handle
}

impl ThreadPool {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        //let (signal, shutdown ) = signal();

        // TODO: see above
        Self { handle: runtime.handle().clone() }

        //Self { handle: runtime.handle().clone(), shutdown, signal }
    }

    /// Spawns a regular task depending on the given [TaskKind]
    pub fn spawn_task_as<F>(handle: self::Handle, fut: F, task_kind: TaskKind) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + Sync + 'static
    {
        let task = async move {
            pin_mut!(fut);
            let _ = fut.await;
        };

        Self::spawn_on_rt(handle, task, task_kind)
    }

    /// Spawns a future on the tokio runtime depending on the [TaskKind]
    fn spawn_on_rt<F>(handle: self::Handle, fut: F, task_kind: TaskKind) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static
    {
        match task_kind {
            TaskKind::Default => handle.spawn(fut),
            TaskKind::Blocking => handle.clone().spawn_blocking(move || handle.block_on(fut))
        }
    }
}

/// specifies a blocking or non blocking task
pub(crate) enum TaskKind {
    Default,
    Blocking
}

// finish shutdown mechanism
/*

/// A Future that resolves when the shutdown event has been fired.
#[derive(Debug, Clone)]
pub struct Shutdown(Shared<oneshot::Receiver<()>>);

impl Future for Shutdown {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pin = self.get_mut();
        if pin.0.is_terminated() || pin.0.poll_unpin(cx).is_ready() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Shutdown signal that fires either manually or on drop by closing the channel
#[derive(Debug)]
pub struct Signal(oneshot::Sender<()>);

impl Signal {
    /// Fire the signal manually.
    pub fn fire(self) {
        let _ = self.0.send(());
    }
}

/// Create a channel pair that's used to propagate shutdown event
pub fn signal() -> (Signal, Shutdown) {
    let (sender, receiver) = oneshot::channel();
    (Signal(sender), Shutdown(receiver.shared()))
}
*/

