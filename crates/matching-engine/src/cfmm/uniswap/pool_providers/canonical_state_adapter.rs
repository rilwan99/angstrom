use std::{collections::HashMap, sync::Arc};

use alloy::{
    primitives::{BlockNumber, Log as PrimitiveAlloyLog},
    rpc::types::{Filter, Log as AlloyLog}
};
use futures_util::StreamExt;
use reth_primitives::{BlockNumberOrTag, Log as RethLog, SealedBlockWithSenders};
use reth_provider::CanonStateNotification;
use tokio::sync::{broadcast, RwLock};

use crate::cfmm::uniswap::{pool_manager::PoolManagerError, pool_providers::PoolManagerProvider};

const MAX_NUMBER_OF_BLOCKS_LOOKBACK: usize = 20;
pub struct CanonicalStateAdapter {
    canon_state_notifications: broadcast::Receiver<CanonStateNotification>,
    log_cache:                 Arc<RwLock<HashMap<BlockNumber, Vec<AlloyLog>>>>
}

impl CanonicalStateAdapter {
    pub fn new(canon_state_notifications: broadcast::Receiver<CanonStateNotification>) -> Self {
        Self { canon_state_notifications, log_cache: Arc::new(RwLock::new(HashMap::new())) }
    }

    async fn update_log_cache(&self, block_number: BlockNumber, new_logs: Vec<AlloyLog>) {
        let mut cache = self.log_cache.write().await;
        cache.insert(block_number, new_logs);
        
        // Keep only the last 20 blocks
        if cache.len() > MAX_NUMBER_OF_BLOCKS_LOOKBACK {
            let mut block_numbers: Vec<_> = cache.keys().cloned().collect();
            block_numbers.sort_unstable();
            let to_remove = block_numbers.len() - MAX_NUMBER_OF_BLOCKS_LOOKBACK;
            for &block_num in block_numbers.iter().take(to_remove) {
                cache.remove(&block_num);
            }
        }
    }
}

impl PoolManagerProvider for CanonicalStateAdapter {
    fn subscribe_blocks(&self) -> futures::stream::BoxStream<Option<u64>> {
        futures_util::stream::unfold(
            self.canon_state_notifications.resubscribe(),
            move |mut notifications| async move {
                if let Ok(notification) = notifications.recv().await {
                    let block = match notification {
                        CanonStateNotification::Commit { new }
                        | CanonStateNotification::Reorg { new, .. } => {
                            let block = new.tip();
                            let logs: Vec<AlloyLog> = new
                                .execution_outcome()
                                .logs(block.number)
                                .unwrap()
                                .map(|l| reth_log_to_alloy_log(block, l))
                                .collect();
                            self.update_log_cache(block.number, logs).await;
                            Some(Some(block.number))
                        }
                    };
                    Some((block, notifications))
                } else {
                    None
                }
            }
        )
        .filter_map(futures_util::future::ready)
        .boxed()
    }

    fn get_logs(
        &self,
        filter: &Filter
    ) -> impl std::future::Future<Output = Result<Vec<AlloyLog>, PoolManagerError>> + Send {
        let log_cache = Arc::clone(&self.log_cache);

        async move {
            let cache = log_cache.read().await;
            let res = cache
                .values()
                .flat_map(|logs| logs.iter().filter(|log| log_matches_filter(log, filter)))
                .cloned()
                .collect::<Vec<_>>();
            Ok(res)
        }
    }
}
fn log_matches_filter(log: &AlloyLog, filter: &Filter) -> bool {
    filter.address.matches(&log.address())
        && filter.topics.iter().enumerate().all(|(i, topic)| {
            topic.matches(
                log.topics()
                    .get(i)
                    .unwrap_or(&alloy::primitives::B256::ZERO)
            )
        })
        && match &filter.block_option {
            alloy::rpc::types::FilterBlockOption::Range { from_block, to_block } => {
                let log_block = log.block_number.unwrap_or_default();
                from_block.as_ref().map_or(true, |from| matches!(from, BlockNumberOrTag::Number(from_num) if &log_block >= from_num))
                && to_block.as_ref().map_or(true, |to| matches!(to, BlockNumberOrTag::Number(to_num) if &log_block <= to_num))
            }
            alloy::rpc::types::FilterBlockOption::AtBlockHash(hash) => log.block_hash == Some(*hash)
        }
}

fn reth_log_to_alloy_log(block: &SealedBlockWithSenders, log: &RethLog) -> AlloyLog {
    AlloyLog {
        inner: PrimitiveAlloyLog { address: log.address, data: log.data.clone() },
        block_hash: Some(block.hash()),
        block_timestamp: Some(block.timestamp),
        block_number: Some(block.number),
        ..Default::default()
    }
}
