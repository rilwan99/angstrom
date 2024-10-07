use std::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll}
};

use angstrom_network::StromNetworkManager;
use futures::FutureExt;

pub(crate) struct TestnetPeerFuture {
    eth_peer_fut:      Pin<Box<dyn Future<Output = ()>>>,
    /// the default ethereum network peer
    strom_network_fut: Pin<Box<dyn Future<Output = ()>>>,
    running:           AtomicBool
}

impl TestnetPeerFuture {
    pub(crate) fn new<C: Unpin>(
        eth_peer: Peer<C>,
        strom_network: StromNetworkManager<C>,
        running: AtomicBool
    ) -> Self {
        Self {
            eth_peer_fut: Box::pin(eth_peer),
            strom_network_fut: Box::pin(strom_network),
            running
        }
    }
}

impl Future for TestnetPeerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.running.load(Ordering::Relaxed) {
            if this.eth_peer_fut.poll_unpin(cx).is_ready() {
                return Poll::Ready(())
            }

            if this.strom_network_fut.poll_unpin(cx).is_ready() {
                return Poll::Ready(())
            }
        }

        Poll::Pending
    }
}
