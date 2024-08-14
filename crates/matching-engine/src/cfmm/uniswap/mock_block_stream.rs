use std::{marker::PhantomData, sync::Arc};
use alloy::{
    eips::BlockNumberOrTag,
    network::Network,
    providers::Provider,
    rpc::types::Block,
    transports::Transport,
};
use futures::{Stream, StreamExt};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct MockBlockStream<T, N, P> {
    from_block: u64,
    to_block: u64,
    provider: Arc<P>,
    transport: PhantomData<T>,
    network: PhantomData<N>,
}

impl<T, N, P> MockBlockStream<T, N, P>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N> + 'static,
{
    pub fn new(from_block: u64, to_block: u64, provider: Arc<P>) -> Self {
        Self {
            from_block,
            to_block,
            provider,
            transport: PhantomData,
            network: PhantomData,
        }
    }

    pub async fn subscribe_blocks(&self) -> Result<impl Stream<Item=Block> + Send, Box<dyn std::error::Error + Send + Sync>> {
        let from_block = self.from_block;
        let to_block = self.to_block;
        let provider = self.provider.clone();

        let stream = futures::stream::unfold(from_block, move |current_block| {
            let provider = provider.clone();
            async move {
                if current_block > to_block {
                    return None;
                }

                match provider.get_block_by_number(BlockNumberOrTag::Number(current_block), false).await {
                    Ok(Some(block)) => {
                        // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        Some((Ok(block), current_block + 1))
                    },
                    Ok(None) => None,
                    Err(e) => {
                        tracing::error!("RPC ERROR {:?}", e);
                        Some((Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>), current_block))
                    },
                }
            }
        }).filter_map(|result| async move {
            match result {
                Ok(block) => Some(block),
                Err(e) => {
                    tracing::error!("Error fetching block: {:?}", e);
                    None
                }
            }
        });
        Ok(Box::pin(stream))
    }
}