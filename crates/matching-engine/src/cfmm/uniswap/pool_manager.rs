use std::{marker::PhantomData, sync::Arc};

use crate::cfmm::uniswap::{mock_block_stream::MockBlockStream, pool::EnhancedUniswapV3Pool};
use alloy::{
    network::Network,
    primitives::{Address, BlockNumber},
    providers::Provider,
    rpc::types::eth::{Block, Filter, Log},
    sol_types::SolEvent,
    transports::Transport,
};
use amms::{
    amm::{uniswap_v3::IUniswapV3Pool, AutomatedMarketMaker},
    errors::{EventLogError, SwapSimulationError},
};
use arraydeque::ArrayDeque;
use futures::StreamExt;
use thiserror::Error;
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock, RwLockWriteGuard,
    },
    task::JoinHandle,
};

pub type StateChangeCache = ArrayDeque<StateChange, 150>;

#[derive(Debug)]
pub struct UniswapPoolManager<P, T, N> {
    pool: Arc<RwLock<EnhancedUniswapV3Pool>>,
    latest_synced_block: u64,
    stream_buffer: usize,
    state_change_buffer: usize,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    provider: Arc<P>,
    // can't be avoided for now if we want to be able to test
    mock_block_stream: Arc<Option<MockBlockStream<P, T, N>>>,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> UniswapPoolManager<P, T, N>
where
    P: Provider<T, N> + 'static,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(
        pool: EnhancedUniswapV3Pool,
        latest_synced_block: u64,
        stream_buffer: usize,
        state_change_buffer: usize,
        provider: Arc<P>,
        mock_block_stream: Option<MockBlockStream<P, T, N>>,
    ) -> Self {
        Self {
            pool: Arc::new(RwLock::new(pool)),
            latest_synced_block,
            stream_buffer,
            state_change_buffer,
            state_change_cache: Arc::new(RwLock::new(ArrayDeque::new())),
            provider,
            _phantom: PhantomData,
            mock_block_stream: Arc::new(mock_block_stream),
        }
    }

    pub async fn pool(&self) -> tokio::sync::RwLockReadGuard<EnhancedUniswapV3Pool> {
        self.pool.read().await
    }

    pub async fn pool_mut(&self) -> tokio::sync::RwLockWriteGuard<EnhancedUniswapV3Pool> {
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
        (Receiver<(Address, BlockNumber)>, Vec<JoinHandle<Result<(), PoolManagerError>>>),
        PoolManagerError
    > {
        let mut last_synced_block = self.latest_synced_block;

        let (stream_tx, mut stream_rx): (Sender<Block>, Receiver<Block>) =
            tokio::sync::mpsc::channel(self.stream_buffer);

        let provider = self.provider.clone();
        let address = self.pool().await.address;
        let mock_block_stream = self.mock_block_stream.clone();
        let stream_handle = tokio::spawn(async move {
            match mock_block_stream.as_ref() {
                Some(provider) => {
                    let mut block_stream = provider.subscribe_blocks().await.expect("the stream");
                    while let Some(block) = block_stream.next().await {
                        stream_tx.send(block).await?;
                    }
                }
                None => {
                    let subscription = provider.subscribe_blocks().await?;
                    let mut block_stream = subscription.into_stream();
                    while let Some(block) = block_stream.next().await {
                        stream_tx.send(block).await?;
                    }
                }
            };
            Ok::<(), PoolManagerError>(())
        });

        let (pool_updated_tx, pool_updated_rx) =
            tokio::sync::mpsc::channel(self.state_change_buffer);

        let pool = self.pool.clone();
        let provider = self.provider.clone();
        let filter = self.filter().await;
        let state_change_cache = self.state_change_cache.clone();

        let updated_pool_handle: JoinHandle<Result<(), PoolManagerError>> =
            tokio::spawn(async move {
                while let Some(block) = stream_rx.recv().await {
                    if let Some(chain_head_block_number) = block.header.number {
                        // If there is a reorg, unwind state changes from last_synced block to the
                        // chain head block number
                        if chain_head_block_number <= last_synced_block {
                            tracing::trace!(
                                chain_head_block_number,
                                last_synced_block,
                                "reorg detected, unwinding state changes"
                            );
                            unwind_state_changes(
                                pool.clone(),
                                state_change_cache.clone(),
                                chain_head_block_number,
                            )
                                .await?;

                            // set the last synced block to the head block number
                            last_synced_block = chain_head_block_number - 1;
                        }

                        let from_block: u64 = last_synced_block + 1;
                        let logs = provider
                            .get_logs(
                                &filter
                                    .clone()
                                    .from_block(from_block)
                                    .to_block(chain_head_block_number)
                            )
                            .await?;

                        if logs.is_empty() {
                            for block_number in from_block..=chain_head_block_number {
                                add_state_change_to_cache(
                                    state_change_cache.clone(),
                                    StateChange::new(None, block_number),
                                )
                                    .await?;
                            }
                        } else {
                            let write_lock = pool.write().await;
                            // if you don't care for the pool state on each block, pass the pool
                            handle_state_changes_from_logs(
                                write_lock,
                                state_change_cache.clone(),
                                logs,
                            )
                                .await?;

                            if let Err(e) = pool_updated_tx
                                .send((address, chain_head_block_number as BlockNumber))
                                .await
                            {
                                tracing::error!("Failed to send pool update: {}", e);
                            }
                        }

                        last_synced_block = chain_head_block_number;
                    } else {
                        return Err(PoolManagerError::BlockNumberNotFound);
                    }
                }

                Ok::<(), PoolManagerError>(())
            });

        Ok((pool_updated_rx, vec![stream_handle, updated_pool_handle]))
    }

    /// Listens to new blocks and handles state changes
    pub async fn watch_state_changes(
        &self
    ) -> Result<Vec<JoinHandle<Result<(), PoolManagerError>>>, PoolManagerError> {
        let mut last_synced_block = self.latest_synced_block;

        let (stream_tx, mut stream_rx): (Sender<Block>, Receiver<Block>) =
            tokio::sync::mpsc::channel(self.stream_buffer);

        let provider = self.provider.clone();
        let stream_handle = tokio::spawn(async move {
            let subscription = provider.subscribe_blocks().await?;
            let mut block_stream = subscription.into_stream();
            while let Some(block) = block_stream.next().await {
                stream_tx.send(block).await?;
            }

            Ok::<(), PoolManagerError>(())
        });

        let pool = self.pool.clone();
        let provider = self.provider.clone();
        let filter = self.filter().await;
        let state_change_cache = self.state_change_cache.clone();

        let updated_pool_handle: JoinHandle<Result<(), PoolManagerError>> =
            tokio::spawn(async move {
                while let Some(block) = stream_rx.recv().await {
                    if let Some(chain_head_block_number) = block.header.number {
                        // If there is a reorg, unwind state changes from last_synced block to the
                        // chain head block number
                        if chain_head_block_number <= last_synced_block {
                            unwind_state_changes(
                                pool.clone(),
                                state_change_cache.clone(),
                                chain_head_block_number,
                            )
                                .await?;

                            // set the last synced block to the head block number
                            last_synced_block = chain_head_block_number - 1;
                        }

                        let from_block: u64 = last_synced_block + 1;
                        let logs = provider
                            .get_logs(
                                &filter
                                    .clone()
                                    .from_block(from_block)
                                    .to_block(chain_head_block_number)
                            )
                            .await?;

                        if logs.is_empty() {
                            for block_number in from_block..=chain_head_block_number {
                                add_state_change_to_cache(
                                    state_change_cache.clone(),
                                    StateChange::new(None, block_number),
                                )
                                    .await?;
                            }
                        } else {
                            let write_lock = pool.write().await;
                            handle_state_changes_from_logs(
                                write_lock,
                                state_change_cache.clone(),
                                logs,
                            )
                                .await?;
                        }

                        last_synced_block = chain_head_block_number;
                    } else {
                        return Err(PoolManagerError::BlockNumberNotFound);
                    }
                }

                Ok::<(), PoolManagerError>(())
            });

        Ok(vec![stream_handle, updated_pool_handle])
    }
}

#[derive(Debug)]
pub struct StateChange {
    state_change: Option<EnhancedUniswapV3Pool>,
    block_number: u64,
}

impl StateChange {
    pub fn new(state_change: Option<EnhancedUniswapV3Pool>, block_number: u64) -> Self {
        Self { block_number, state_change }
    }
}

/// Unwinds the state changes cache for every block from the most recent state
/// change cache back to the block to unwind -1.
async fn unwind_state_changes(
    pool: Arc<RwLock<EnhancedUniswapV3Pool>>,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    block_to_unwind: u64,
) -> Result<(), PoolManagerError> {
    let mut state_change_cache = state_change_cache.write().await;

    loop {
        // check if the most recent state change block is >= the block to unwind,
        if let Some(state_change) = state_change_cache.get(0) {
            if state_change.block_number >= block_to_unwind {
                if let Some(option_state_change) = state_change_cache.pop_front() {
                    if let Some(pool_state) = option_state_change.state_change {
                        *pool.write().await = pool_state;
                    }
                } else {
                    // We know that there is a state change from state_change_cache.get(0) so when
                    // we pop front without returning a value, there is an issue
                    return Err(PoolManagerError::PopFrontError);
                }
            } else {
                return Ok(());
            }
        } else {
            // We return an error here because we never want to be unwinding past where we
            // have state changes. For example, if you initialize a state space
            // that syncs to block 100, then immediately after there is a chain reorg to 95,
            // we can not roll back the state changes for an accurate state
            // space. In this case, we return an error
            return Err(PoolManagerError::NoStateChangesInCache);
        }
    }
}

async fn add_state_change_to_cache(
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    state_change: StateChange,
) -> Result<(), PoolManagerError> {
    let mut state_change_cache = state_change_cache.write().await;

    if state_change_cache.is_full() {
        state_change_cache.pop_back();
        state_change_cache
            .push_front(state_change)
            .map_err(|_| PoolManagerError::CapacityError)?
    } else {
        state_change_cache
            .push_front(state_change)
            .map_err(|_| PoolManagerError::CapacityError)?
    }
    Ok(())
}

pub async fn handle_state_changes_from_logs(
    mut pool: RwLockWriteGuard<'_, EnhancedUniswapV3Pool>,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    logs: Vec<Log>,
) -> Result<(), PoolManagerError> {
    let log = logs.first().unwrap();
    let mut last_log_block_number = get_block_number_from_log(log)?;

    for log in logs.into_iter() {
        let log_block_number = get_block_number_from_log(&log)?;

        let pool_clone = pool.clone();
        let event_signature = log.topics()[0];
        if event_signature == IUniswapV3Pool::Burn::SIGNATURE_HASH {
            pool.sync_from_burn_log(log)?;
        } else if event_signature == IUniswapV3Pool::Mint::SIGNATURE_HASH {
            pool.sync_from_mint_log(log)?;
        } else if event_signature == IUniswapV3Pool::Swap::SIGNATURE_HASH {
            sync_swap_with_sym(&mut pool, &log, log_block_number)?;
        } else {
            Err(EventLogError::InvalidEventSignature)?
        }
        if log_block_number != last_log_block_number {
            add_state_change_to_cache(
                state_change_cache.clone(),
                StateChange::new(Some(pool_clone), last_log_block_number),
            )
                .await?;

            last_log_block_number = log_block_number;
        }
    }

    let pool_clone = pool.clone();
    add_state_change_to_cache(
        state_change_cache,
        StateChange::new(Some(pool_clone), last_log_block_number),
    )
        .await
}
fn sync_swap_with_sym(
    pool: &mut EnhancedUniswapV3Pool,
    log: &Log,
    log_block_number: u64,
) -> Result<(), PoolManagerError> {
    let swap_event = IUniswapV3Pool::Swap::decode_log(&log.inner, true)?;

    tracing::trace!(block_number=log_block_number, pool_tick = ?pool.tick, pool_price = ?pool.sqrt_price, pool_liquidity = ?pool.liquidity, pool_address = ?pool.address, "pool before");
    tracing::debug!(block_number=log_block_number, swap_tick=swap_event.tick, swap_price=?swap_event.sqrtPriceX96, swap_liquidity=?swap_event.liquidity, swap_amount0=?swap_event.amount0, swap_amount1=?swap_event.amount1, "swap event");

    let combinations = [
        (pool.token_b, swap_event.amount1),
        (pool.token_a, swap_event.amount0),
        (pool.token_a, swap_event.amount1),
        (pool.token_b, swap_event.amount0),
    ];

    let mut simulation_failed = true;
    for (token_in, amount_in) in combinations.iter() {
        let sqrt_price_limit_x96 = Some(swap_event.sqrtPriceX96);
        if let Ok((amount0, amount1)) = pool.simulate_swap(*token_in, *amount_in, sqrt_price_limit_x96) {
            if swap_event.amount0 == amount0 && swap_event.amount1 == amount1 {
                // will not fail
                let (..) = pool.simulate_swap_mut(*token_in, *amount_in, sqrt_price_limit_x96)?;
                simulation_failed = false;
                break;
            }
        }
    }

    if simulation_failed {
        tracing::error!(
            block_number = log_block_number,
            pool_address = ?pool.address,
            pool_price = ?pool.sqrt_price,
            pool_liquidity = ?pool.liquidity,
            pool_tick = ?pool.tick,
            swap_price = ?swap_event.sqrtPriceX96,
            swap_tick = swap_event.tick,
            swap_liquidity = ?swap_event.liquidity,
            swap_amount0 = ?swap_event.amount0,
            swap_amount1 = ?swap_event.amount1,
            "Swap simulation failed"
        );
        return Err(PoolManagerError::SwapSimulationFailed);
    } else {
        tracing::trace!(block_number=log_block_number, pool_tick = ?pool.tick, pool_price = ?pool.sqrt_price, pool_liquidity = ?pool.liquidity, pool_address = ?pool.address, "pool after");
    }

    Ok(())
}

pub fn get_block_number_from_log(log: &Log) -> Result<u64, EventLogError> {
    if let Some(block_number) = log.block_number {
        Ok(block_number)
    } else {
        Err(EventLogError::LogBlockNumberNotFound)
    }
}

#[derive(Error, Debug)]
pub enum PoolManagerError {
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
    BlockSendError(#[from] tokio::sync::mpsc::error::SendError<alloy::rpc::types::eth::Block>),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
}
