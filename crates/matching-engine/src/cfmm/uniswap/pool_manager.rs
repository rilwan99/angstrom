use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc
};

use alloy::{
    primitives::{Address, BlockNumber},
    rpc::types::eth::{Block, Filter, Log},
    sol_types::SolEvent
};
use amms::{
    amm::{uniswap_v3::IUniswapV3Pool, AutomatedMarketMaker},
    errors::EventLogError
};
use arraydeque::ArrayDeque;
use futures::StreamExt;
use futures_util::stream::BoxStream;
use thiserror::Error;
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock, RwLockReadGuard, RwLockWriteGuard
    },
    task::JoinHandle
};

use super::pool::SwapSimulationError;
use crate::cfmm::uniswap::{pool::EnhancedUniswapV3Pool, pool_providers::PoolManagerProvider};

pub type StateChangeCache = ArrayDeque<StateChange, 150>;

pub struct UniswapPoolManager<P> {
    pool:                Arc<RwLock<EnhancedUniswapV3Pool>>,
    latest_synced_block: u64,
    state_change_buffer: usize,
    state_change_cache:  Arc<RwLock<StateChangeCache>>,
    provider:            Arc<P>,
    sync_started:        AtomicBool
}

impl<P> UniswapPoolManager<P>
where
    P: PoolManagerProvider + Send + Sync + 'static
{
    pub fn new(
        pool: EnhancedUniswapV3Pool,
        latest_synced_block: u64,
        state_change_buffer: usize,
        provider: Arc<P>
    ) -> Self {
        Self {
            pool: Arc::new(RwLock::new(pool)),
            latest_synced_block,
            state_change_buffer,
            state_change_cache: Arc::new(RwLock::new(ArrayDeque::new())),
            provider,
            sync_started: AtomicBool::new(false)
        }
    }

    pub async fn pool(&self) -> RwLockReadGuard<'_, EnhancedUniswapV3Pool> {
        self.pool.read().await
    }

    pub async fn pool_mut(&self) -> RwLockWriteGuard<'_, EnhancedUniswapV3Pool> {
        self.pool.write().await
    }

    pub async fn filter(&self) -> Filter {
        let pool = self.pool().await;
        Filter::new()
            .address(pool.address())
            .event_signature(pool.sync_on_event_signatures())
    }

    /// Listens to new blocks and handles state changes, sending the pool
    /// address if it incurred a state change in the block.
    pub async fn subscribe_state_changes(
        &self
    ) -> Result<
        (Receiver<(Address, BlockNumber)>, JoinHandle<Result<(), PoolManagerError>>),
        PoolManagerError
    > {
        if self
            .sync_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(PoolManagerError::SyncAlreadyStarted);
        }

        let (pool_updated_tx, pool_updated_rx) =
            tokio::sync::mpsc::channel(self.state_change_buffer);

        let address = self.pool().await.address;
        let updated_pool_handle = self
            .handle_state_changes(Some(pool_updated_tx), address)
            .await?;

        Ok((pool_updated_rx, updated_pool_handle))
    }

    /// Listens to new blocks and handles state changes
    pub async fn watch_state_changes(
        &self
    ) -> Result<JoinHandle<Result<(), PoolManagerError>>, PoolManagerError> {
        if self
            .sync_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(PoolManagerError::SyncAlreadyStarted);
        }

        let address = self.pool().await.address;
        let updated_pool_handle = self.handle_state_changes(None, address).await?;

        Ok(updated_pool_handle)
    }

    async fn handle_state_changes(
        &self,
        pool_updated_tx: Option<Sender<(Address, BlockNumber)>>,
        address: Address
    ) -> Result<JoinHandle<Result<(), PoolManagerError>>, PoolManagerError> {
        let mut last_synced_block = self.latest_synced_block;

        let pool = Arc::clone(&self.pool);
        let provider = Arc::clone(&self.provider);
        let filter = self.filter().await;
        let state_change_cache = Arc::clone(&self.state_change_cache);
        let updated_pool_handle = tokio::spawn(async move {
            let mut block_stream: BoxStream<Option<u64>> = provider.subscribe_blocks();
            while let Some(block_number) = block_stream.next().await {
                let chain_head_block_number =
                    block_number.ok_or(PoolManagerError::BlockNumberNotFound)?;
                // If there is a reorg, unwind state changes from last_synced block to the
                // chain head block number
                if chain_head_block_number <= last_synced_block {
                    tracing::trace!(
                        chain_head_block_number,
                        last_synced_block,
                        "reorg detected, unwinding state changes"
                    );
                    let mut pool_guard = pool.write().await;
                    let mut state_change_cache = state_change_cache.write().await;
                    Self::unwind_state_changes(
                        &mut pool_guard,
                        &mut state_change_cache,
                        chain_head_block_number
                    )?;

                    // set the last synced block to the head block number
                    last_synced_block = chain_head_block_number - 1;
                }

                let from_block = last_synced_block + 1;
                let logs = provider
                    .get_logs(
                        &filter
                            .clone()
                            .from_block(from_block)
                            .to_block(chain_head_block_number)
                    )
                    .await?;

                if logs.is_empty() {
                    let mut state_change_cache = state_change_cache.write().await;
                    for block_number in from_block..=chain_head_block_number {
                        Self::add_state_change_to_cache(
                            &mut state_change_cache,
                            StateChange::new(None, block_number)
                        )?;
                    }
                } else {
                    let mut pool_guard = pool.write().await;
                    let mut state_change_cache = state_change_cache.write().await;
                    Self::handle_state_changes_from_logs(
                        &mut pool_guard,
                        &mut state_change_cache,
                        logs
                    )?;

                    if let Some(tx) = &pool_updated_tx {
                        if let Err(e) = tx
                            .send((address, chain_head_block_number as BlockNumber))
                            .await
                        {
                            tracing::error!("Failed to send pool update: {}", e);
                        }
                    }
                }

                last_synced_block = chain_head_block_number;
            }

            Ok(())
        });

        Ok(updated_pool_handle)
    }

    /// Unwinds the state changes cache for every block from the most recent
    /// state change cache back to the block to unwind -1.
    fn unwind_state_changes(
        pool: &mut EnhancedUniswapV3Pool,
        state_change_cache: &mut StateChangeCache,
        block_to_unwind: u64
    ) -> Result<(), PoolManagerError> {
        loop {
            // check if the most recent state change block is >= the block to unwind,
            match state_change_cache.get(0) {
                Some(state_change) if state_change.block_number >= block_to_unwind => {
                    if let Some(option_state_change) = state_change_cache.pop_front() {
                        if let Some(pool_state) = option_state_change.state_change {
                            *pool = pool_state;
                        }
                    } else {
                        // We know that there is a state change from state_change_cache.get(0) so
                        // when we pop front without returning a value,
                        // there is an issue
                        return Err(PoolManagerError::PopFrontError);
                    }
                }
                Some(_) => return Ok(()),
                None => {
                    // We return an error here because we never want to be unwinding past where we
                    // have state changes. For example, if you initialize a state space
                    // that syncs to block 100, then immediately after there is a chain reorg to 95,
                    // we can not roll back the state changes for an accurate state
                    // space. In this case, we return an error
                    return Err(PoolManagerError::NoStateChangesInCache);
                }
            }
        }
    }

    fn add_state_change_to_cache(
        state_change_cache: &mut StateChangeCache,
        state_change: StateChange
    ) -> Result<(), PoolManagerError> {
        if state_change_cache.is_full() {
            state_change_cache.pop_back();
        }
        state_change_cache
            .push_front(state_change)
            .map_err(|_| PoolManagerError::CapacityError)
    }

    fn handle_state_changes_from_logs(
        pool: &mut EnhancedUniswapV3Pool,
        state_change_cache: &mut StateChangeCache,
        logs: Vec<Log>
    ) -> Result<(), PoolManagerError> {
        let log = logs.first().ok_or(PoolManagerError::NoLogsProvided)?;
        let mut last_log_block_number = log
            .block_number
            .ok_or(EventLogError::LogBlockNumberNotFound)?;

        for log in logs {
            let log_block_number = log
                .block_number
                .ok_or(EventLogError::LogBlockNumberNotFound)?;

            let pool_clone = pool.clone();
            let event_signature = log.topics()[0];
            match event_signature {
                IUniswapV3Pool::Burn::SIGNATURE_HASH => pool.sync_from_burn_log(log)?,
                IUniswapV3Pool::Mint::SIGNATURE_HASH => pool.sync_from_mint_log(log)?,
                IUniswapV3Pool::Swap::SIGNATURE_HASH => pool.sync_from_swap_log(log)?,
                _ => return Err(EventLogError::InvalidEventSignature.into())
            }
            if log_block_number != last_log_block_number {
                Self::add_state_change_to_cache(
                    state_change_cache,
                    StateChange::new(Some(pool_clone), last_log_block_number)
                )?;

                last_log_block_number = log_block_number;
            }
        }

        let pool_clone = pool.clone();
        Self::add_state_change_to_cache(
            state_change_cache,
            StateChange::new(Some(pool_clone), last_log_block_number)
        )
    }
}

#[derive(Debug)]
pub struct StateChange {
    state_change: Option<EnhancedUniswapV3Pool>,
    block_number: u64
}

impl StateChange {
    pub fn new(state_change: Option<EnhancedUniswapV3Pool>, block_number: u64) -> Self {
        Self { state_change, block_number }
    }
}

#[derive(Error, Debug)]
pub enum PoolManagerError {
    #[error("No logs provided")]
    NoLogsProvided,
    #[error("No state changes in cache")]
    NoStateChangesInCache,
    #[error("Error when removing a state change from the front of the deque")]
    PopFrontError,
    #[error("State change cache capacity error")]
    CapacityError,
    #[error(transparent)]
    EventLogError(#[from] EventLogError),
    #[error("Invalid event signature")]
    InvalidEventSignature,
    #[error("Provider error")]
    ProviderError,
    #[error("Swap simulation failed")]
    SwapSimulationFailed,
    #[error(transparent)]
    SwapSimulationError(#[from] SwapSimulationError),
    #[error("Block number not found")]
    BlockNumberNotFound,
    #[error(transparent)]
    TransportError(#[from] alloy::transports::TransportError),
    #[error(transparent)]
    ContractError(#[from] alloy::contract::Error),
    #[error(transparent)]
    ABICodecError(#[from] alloy::dyn_abi::Error),
    #[error(transparent)]
    EthABIError(#[from] alloy::sol_types::Error),
    #[error(transparent)]
    AMMError(#[from] amms::errors::AMMError),
    #[error(transparent)]
    ArithmeticError(#[from] amms::errors::ArithmeticError),
    #[error(transparent)]
    BlockSendError(#[from] tokio::sync::mpsc::error::SendError<Block>),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("Synchronization has already been started")]
    SyncAlreadyStarted
}
