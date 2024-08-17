use alloy::eips::BlockNumberOrTag;
use alloy::providers::RootProvider;
use alloy::rpc::types::Block;
use alloy::{
    network::Network, providers::Provider,
    transports::Transport,
};
use futures::Stream;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_stream::{
    wrappers::BroadcastStream,
    StreamExt,
};

#[derive(Debug, Clone)]
pub struct MockBlockStream<P, T, N> {
    inner: Arc<P>,
    from_block: u64,
    to_block: u64,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> MockBlockStream<P, T, N>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(inner: Arc<P>, from_block: u64, to_block: u64) -> Self {
        Self {
            inner,
            from_block,
            to_block,
            _phantom: PhantomData,
        }
    }

    pub async fn subscribe_blocks(
        &self
    ) -> Result<impl Stream<Item=Block> + Send, Box<dyn std::error::Error + Send + Sync>> {
        let (tx, rx) = broadcast::channel(100);
        let from_block = self.from_block;
        let to_block = self.to_block;
        let inner = self.inner.clone();

        tokio::spawn(async move {
            for block_number in from_block..=to_block {
                if let Some(block) = inner
                    .get_block_by_number(BlockNumberOrTag::Number(block_number), false)
                    .await
                    .ok()
                    .and_then(|b| b)
                {
                    if tx.send(block).is_err() {
                        break;
                    }
                }
            }
        });
        Ok(BroadcastStream::new(rx).filter_map(|result| result.ok()))
    }
}

#[async_trait::async_trait]
impl<P: Provider<T, N> + 'static, T: Transport + Clone, N: Network> Provider<T, N> for MockBlockStream<P, T, N> {
    fn root(&self) -> &RootProvider<T, N> {
        self.inner.root()
    }
}
