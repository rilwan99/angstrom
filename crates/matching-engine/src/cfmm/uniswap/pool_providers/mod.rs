use std::future::Future;

use alloy::rpc::types::eth::{Filter, Log};

use crate::cfmm::uniswap::pool_manager::PoolManagerError;

pub mod canonical_state_adapter;
pub mod mock_block_stream;
pub mod provider_adapter;

pub trait PoolManagerProvider {
    fn subscribe_blocks(&self) -> futures::stream::BoxStream<Option<u64>>;
    fn get_logs(
        &self,
        filter: &Filter
    ) -> impl Future<Output = Result<Vec<Log>, PoolManagerError>> + Send;
}
