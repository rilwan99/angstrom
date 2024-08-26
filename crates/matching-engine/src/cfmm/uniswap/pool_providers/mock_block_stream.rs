use std::{marker::PhantomData, sync::Arc};

use alloy::{
    network::Network,
    providers::Provider,
    rpc::types::{Filter, Log},
    transports::Transport
};
use futures_util::StreamExt;

use crate::cfmm::uniswap::{pool_manager::PoolManagerError, pool_providers::PoolManagerProvider};

#[derive(Debug, Clone)]
pub struct MockBlockStream<P, T, N> {
    inner:      Arc<P>,
    from_block: u64,
    to_block:   u64,
    _phantom:   PhantomData<(T, N)>
}

impl<P, T, N> MockBlockStream<P, T, N>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network
{
    pub fn new(inner: Arc<P>, from_block: u64, to_block: u64) -> Self {
        Self { inner, from_block, to_block, _phantom: PhantomData }
    }
}

impl<P, T, N> PoolManagerProvider for MockBlockStream<P, T, N>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network
{
    fn subscribe_blocks(&self) -> futures::stream::BoxStream<Option<u64>> {
        futures::stream::iter(self.from_block..=self.to_block)
            .map(move |i| async move { Some(i) })
            .then(|fut| fut)
            .boxed()
    }

    async fn get_logs(&self, filter: &Filter) -> Result<Vec<Log>, PoolManagerError> {
        self.inner.get_logs(filter).await.map_err(|e| e.into())
    }
}
