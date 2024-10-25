use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
    task::{Context, Poll}
};

use alloy::{network::Network, providers::Provider, transports::Transport};
use consensus::ConsensusManager;
use futures::FutureExt;
use parking_lot::Mutex;
use tokio::task::JoinHandle;
use tracing::{span, Level};

pub(crate) struct TestnetConsensusFuture<P, TR, N> {
    consensus: Arc<Mutex<ConsensusManager<P, TR, N>>>,
    /// JoinHandle for the consensus future
    fut:       JoinHandle<()>
}

impl<P, TR, N> TestnetConsensusFuture<P, TR, N>
where
    P: Provider<TR, N> + Send + Unpin + 'static,
    TR: Transport + Clone + Send + Unpin,
    N: Network + Send + Unpin
{
    pub(crate) fn new(
        testnet_node_id: u64,
        consensus: ConsensusManager<P, TR, N>,
        running: Arc<AtomicBool>
    ) -> Self {
        let consensus = Arc::new(Mutex::new(consensus));
        let internal =
            TestnetConsensusFutureInternals::new(testnet_node_id, consensus.clone(), running);
        Self { consensus, fut: tokio::spawn(internal) }
    }

    pub(crate) fn consensus_manager<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&ConsensusManager<P, TR, N>) -> R
    {
        f(&self.consensus.lock())
    }

    pub(crate) fn consensus_manager_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut ConsensusManager<P, TR, N>) -> R
    {
        f(&mut self.consensus.lock())
    }
}

struct TestnetConsensusFutureInternals<P, TR, N> {
    testnet_node_id: u64,
    consensus:       Arc<Mutex<ConsensusManager<P, TR, N>>>,
    running:         Arc<AtomicBool>
}

impl<P, TR, N> TestnetConsensusFutureInternals<P, TR, N>
where
    P: Provider<TR, N> + Send,
    TR: Transport + Clone + Send,
    N: Network + Send
{
    fn new(
        testnet_node_id: u64,
        consensus: Arc<Mutex<ConsensusManager<P, TR, N>>>,
        running: Arc<AtomicBool>
    ) -> Self {
        Self { testnet_node_id, consensus, running }
    }
}

impl<P, TR, N> Future for TestnetConsensusFutureInternals<P, TR, N>
where
    P: Provider<TR, N> + Send + Unpin,
    TR: Transport + Clone + Send + Unpin,
    N: Network + Send + Unpin
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        let span = span!(Level::TRACE, "node", id = this.testnet_node_id);
        let e = span.enter();

        if this.running.load(Ordering::Relaxed) {
            {
                let mut cons = this.consensus.lock_arc();
                if cons.poll_unpin(cx).is_ready() {
                    return Poll::Ready(())
                }
            }
        }

        drop(e);

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl<P, TR, N> Drop for TestnetConsensusFuture<P, TR, N> {
    fn drop(&mut self) {
        self.fut.abort();
    }
}
