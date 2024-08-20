use std::{marker::PhantomData, sync::Arc};

use alloy::{
    eips::BlockNumberOrTag, network::Network, providers::Provider, rpc::types::Block,
    transports::Transport
};
use futures_util::StreamExt;

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

    pub async fn subscribe_blocks(&self) -> futures::stream::BoxStream<Block> {
        let from_block = self.from_block;
        let to_block = self.to_block;
        let inner = self.inner.clone();

        futures::stream::iter(from_block..=to_block)
            .filter_map(move |i| {
                let value = inner.clone();
                async move {
                    value
                        .get_block_by_number(BlockNumberOrTag::Number(i), false)
                        .await
                        .ok()
                        .flatten()
                }
            })
            .boxed()
    }
}
