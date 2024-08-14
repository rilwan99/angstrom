
use alloy_primitives::{BlockNumber, I256, U256};
use amms::{
    amm::uniswap_v3::IUniswapV3Pool, errors::EventLogError
};
use alloy::{
    network::Network,
    primitives::{Address, B256},
    providers::Provider,
    rpc::types::eth::{Block, Filter, Log},
    transports::Transport,
};

use arraydeque::ArrayDeque;
use amms::state_space::error::{StateChangeError, StateSpaceError};
use futures::StreamExt;
use std::{
    marker::PhantomData,
    sync::Arc,
};
use std::ops::Deref;
use alloy::sol_types::SolEvent;
use amms::amm::AutomatedMarketMaker;
use amms::amm::uniswap_v3::UniswapV3Pool;
use amms::state_space::error::StateSpaceError::{StateChangeError as StateChangeErrorEnum};
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};
use crate::cfmm::uniswap::mock_block_stream::MockBlockStream;

pub type StateChangeCache = ArrayDeque<StateChange, 150>;

#[derive(Debug)]
pub struct UniswapPoolManager<T, N, P> {
    pool: Arc<RwLock<UniswapV3Pool>>,
    latest_synced_block: u64,
    stream_buffer: usize,
    state_change_buffer: usize,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    provider: Arc<P>,
    transport: PhantomData<T>,
    network: PhantomData<N>,
    mock_block_stream: Arc<Option<MockBlockStream<T, N, P>>>,
}

impl<T, N, P> UniswapPoolManager<T, N, P>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N> + 'static,
{
    pub fn new(
        pool: UniswapV3Pool,
        latest_synced_block: u64,
        stream_buffer: usize,
        state_change_buffer: usize,
        provider: Arc<P>,
        mock_block_stream: Option<MockBlockStream<T, N, P>>
    ) -> Self {
        Self {
            pool: Arc::new(RwLock::new(pool)),
            latest_synced_block,
            stream_buffer,
            state_change_buffer,
            state_change_cache: Arc::new(RwLock::new(ArrayDeque::new())),
            provider,
            transport: PhantomData,
            network: PhantomData,
            mock_block_stream: Arc::new(mock_block_stream),
        }
    }

    pub async fn pool(&self) -> tokio::sync::RwLockReadGuard<UniswapV3Pool> {
        self.pool.read().await
    }

    pub async fn pool_mut(&self) -> tokio::sync::RwLockWriteGuard<UniswapV3Pool> {
        self.pool.write().await
    }
    pub async fn filter(&self) -> Filter {
        let pool = self.pool().await;
        Filter::new().event_signature(pool.sync_on_event_signatures())
    }

    /// Listens to new blocks and handles state changes, sending the pool address if it incurred a state change in the block.
    pub async fn subscribe_state_changes(
        &self,
    ) -> Result<
        (
            Receiver<(Option<Address>,BlockNumber)>,
            Vec<JoinHandle<Result<(), StateSpaceError>>>,
        ),
        StateSpaceError,
    > {
        let mut last_synced_block = self.latest_synced_block;

        let (stream_tx, mut stream_rx): (Sender<Block>, Receiver<Block>) =
            tokio::sync::mpsc::channel(self.stream_buffer);

        let provider = self.provider.clone();
        let mock_block_stream = self.mock_block_stream.clone();
        let stream_handle = tokio::spawn(async move {
             match mock_block_stream.as_ref() {
                Some(provider) => {
                    let mut block_stream = provider.subscribe_blocks().await.expect("the stream");
                    while let Some(block) = block_stream.next().await {
                        stream_tx.send(block).await?;
                    }
                },
                None => {
                    let subscription =  provider.subscribe_blocks().await?;
                    let mut block_stream = subscription.into_stream();
                    while let Some(block) = block_stream.next().await {
                        stream_tx.send(block).await?;
                    }
                }
            };
            Ok::<(), StateSpaceError>(())
        });

        let (pool_updated_tx, pool_updated_rx) =
            tokio::sync::mpsc::channel(self.state_change_buffer);

        let pool = self.pool.clone();
        let provider = self.provider.clone();
        let filter = self.filter().await;
        let state_change_cache = self.state_change_cache.clone();

        let updated_pool_handle: JoinHandle<Result<(), StateSpaceError>> =
            tokio::spawn(async move {
                while let Some(block) = stream_rx.recv().await {
                    if let Some(chain_head_block_number) = block.header.number {
                        // If there is a reorg, unwind state changes from last_synced block to the chain head block number
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
                                    .to_block(chain_head_block_number),
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
                            let pool_updated = handle_state_changes_from_logs(
                                pool.clone(),
                                state_change_cache.clone(),
                                logs,
                            )
                                .await?;
                            tracing::info!(
                                block_number = chain_head_block_number,
                                pool_address = ?pool_updated,
                                "log changes applied to pool"
                            );

                            if let Err(e) = pool_updated_tx.send((pool_updated, chain_head_block_number as BlockNumber)).await {
                                tracing::error!("Failed to send pool update: {}", e);
                            }
                        }

                        last_synced_block = chain_head_block_number;
                    } else {
                        return Err(StateSpaceError::BlockNumberNotFound);
                    }
                }

                Ok::<(), StateSpaceError>(())
            });

        Ok((pool_updated_rx, vec![stream_handle, updated_pool_handle]))
    }

    /// Listens to new blocks and handles state changes
    pub async fn watch_state_changes(
        &self,
    ) -> Result<Vec<JoinHandle<Result<(), StateSpaceError>>>, StateSpaceError> {
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

            Ok::<(), StateSpaceError>(())
        });

        let pool = self.pool.clone();
        let provider = self.provider.clone();
        let filter = self.filter().await;
        let state_change_cache = self.state_change_cache.clone();

        let updated_pool_handle: JoinHandle<Result<(), StateSpaceError>> =
            tokio::spawn(async move {
                while let Some(block) = stream_rx.recv().await {
                    if let Some(chain_head_block_number) = block.header.number {
                        // If there is a reorg, unwind state changes from last_synced block to the chain head block number
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
                                    .to_block(chain_head_block_number),
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
                            let _pool_updated = handle_state_changes_from_logs(
                                pool.clone(),
                                state_change_cache.clone(),
                                logs,
                            )
                                .await?;
                        }

                        last_synced_block = chain_head_block_number;
                    } else {
                        return Err(StateSpaceError::BlockNumberNotFound);
                    }
                }

                Ok::<(), StateSpaceError>(())
            });

        Ok(vec![stream_handle, updated_pool_handle])
    }
}

#[derive(Debug)]
pub struct StateChange {
    state_change: Option<UniswapV3Pool>,
    block_number: u64,
}

impl StateChange {
    pub fn new(state_change: Option<UniswapV3Pool>, block_number: u64) -> Self {
        Self {
            block_number,
            state_change,
        }
    }
}

/// Unwinds the state changes cache for every block from the most recent state change cache back to the block to unwind -1.
async fn unwind_state_changes(
    pool: Arc<RwLock<UniswapV3Pool>>,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    block_to_unwind: u64,
) -> Result<(), StateChangeError> {
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
                    // We know that there is a state change from state_change_cache.get(0) so when we pop front without returning a value, there is an issue
                    return Err(StateChangeError::PopFrontError);
                }
            } else {
                return Ok(());
            }
        } else {
            // We return an error here because we never want to be unwinding past where we have state changes.
            // For example, if you initialize a state space that syncs to block 100, then immediately after there is a chain reorg to 95, we can not roll back the state
            // changes for an accurate state space. In this case, we return an error
            return Err(StateChangeError::NoStateChangesInCache);
        }
    }
}

async fn add_state_change_to_cache(
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    state_change: StateChange,
) -> Result<(), StateChangeError> {
    let mut state_change_cache = state_change_cache.write().await;

    if state_change_cache.is_full() {
        state_change_cache.pop_back();
        state_change_cache
            .push_front(state_change)
            .map_err(|_| StateChangeError::CapacityError)?
    } else {
        state_change_cache
            .push_front(state_change)
            .map_err(|_| StateChangeError::CapacityError)?
    }
    Ok(())
}

pub async fn handle_state_changes_from_logs(
    pool: Arc<RwLock<UniswapV3Pool>>,
    state_change_cache: Arc<RwLock<StateChangeCache>>,
    logs: Vec<Log>,
) -> Result<Option<Address>, StateChangeError> {
    let mut pool_updated = false;
    let mut pool_address = None;

    let mut last_log_block_number = if let Some(log) = logs.first() {
        get_block_number_from_log(log)?
    } else {
        return Ok(None);
    };

    for log in logs.into_iter() {
        let log_block_number = get_block_number_from_log(&log)?;

        // check if the log is for our pool
        if log.address() == pool.read().await.address() {
            tracing::debug!(block_number=log_block_number, pool_address=?log.address(), "log change");
            if !pool_updated {
                pool_updated = true;
                pool_address = Some(log.address());
            }

            let mut pool = pool.write().await;
            let pool_clone = pool.clone();

            // ==== CHANGES ==== //
            // pool.sync_from_log(log)?;
            let event_signature = log.topics()[0];

            if event_signature == IUniswapV3Pool::Burn::SIGNATURE_HASH {
                pool.sync_from_burn_log(log).expect("applying the burn");
            } else if event_signature == IUniswapV3Pool::Mint::SIGNATURE_HASH {
                pool.sync_from_mint_log(log).expect("applying the mint");
            } else if event_signature == IUniswapV3Pool::Swap::SIGNATURE_HASH {
                // Parse the swap event and call simulate_swap_mut
                let swap_event = IUniswapV3Pool::Swap::decode_log(&log.inner, true).expect("decoding the log");
                let token_in = if swap_event.amount0 > I256::ZERO {
                    pool.token_a
                } else {
                    pool.token_b
                };
                let amount_in = if swap_event.amount0 < I256::ZERO {
                    U256::try_from(swap_event.amount1).map_err(|e| StateChangeError::CapacityError)?
                } else {
                    U256::try_from(swap_event.amount0).map_err(|e| StateChangeError::CapacityError)?
                };
                 tracing::debug!(?swap_event, address = ?pool.address, sqrt_price = ?pool.sqrt_price, liquidity = ?pool.liquidity, tick = ?pool.tick, "UniswapV3 swap event");
                let amount = pool.simulate_swap_mut(token_in, amount_in).expect("applying the simulate");
                // pool.sync_from_swap_log(log).map_err(StateChangeError)?;
            } else {
                Err(EventLogError::InvalidEventSignature)?
            }
            // ==== CHANGES ==== //

            // Commit state changes if the block has changed since last log
            if log_block_number != last_log_block_number {
                add_state_change_to_cache(
                    state_change_cache.clone(),
                    StateChange::new(Some(pool_clone), last_log_block_number),
                )
                    .await?;

                last_log_block_number = log_block_number;
            }
        }
    }

    if !pool_updated {
        add_state_change_to_cache(
            state_change_cache,
            StateChange::new(None, last_log_block_number),
        )
            .await?;
    } else {
        let pool_clone = pool.read().await.clone();
        add_state_change_to_cache(
            state_change_cache,
            StateChange::new(Some(pool_clone), last_log_block_number),
        )
            .await?;
    };

    Ok(pool_address)
}

pub fn get_block_number_from_log(log: &Log) -> Result<u64, EventLogError> {
    if let Some(block_number) = log.block_number {
        Ok(block_number)
    } else {
        Err(EventLogError::LogBlockNumberNotFound)
    }
}
