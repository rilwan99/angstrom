use alloy::{
    hex,
    network::Ethereum, primitives::{address, Bytes, Log as AlloyLog, U256},
    providers::{Provider, ProviderBuilder, RootProvider, WsConnect},
    rpc::{client::ClientBuilder, types::eth::Log as RpcLog},
    transports::{
        http::{Client, Http},
        layers::{RetryBackoffLayer, RetryBackoffService},
    },
};
use alloy_primitives::BlockNumber;
use std::{ops::Deref, sync::Arc};
use alloy::eips::BlockNumberOrTag;
use alloy::pubsub::PubSubFrontend;
use amms::amm::AMM;
use amms::amm::uniswap_v3::UniswapV3Pool;
use angstrom_types::primitive::poolManagerCall;
use matching_engine::cfmm::uniswap::pool::EnhancedUniswapV3Pool;
use matching_engine::cfmm::uniswap::pool_manager::UniswapPoolManager;
use tokio::signal::unix::{signal, SignalKind};
use futures::StreamExt;
use matching_engine::cfmm::uniswap::mock_block_stream::MockBlockStream;

extern crate arraydeque;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    std::env::set_var("RUST_LOG", "amms=trace,matching_engine=trace,info");
    tracing_subscriber::fmt::init();
    let log_level = tracing::level_filters::LevelFilter::current();
    tracing::info!("Logging initialized at level: {}", log_level);

    let ws_endpoint = std::env::var("ETHEREUM_WS_ENDPOINT")?;
    let ws = WsConnect::new(ws_endpoint);
    let ws_provider: RootProvider<PubSubFrontend, Ethereum> = ProviderBuilder::default().on_ws(ws).await.unwrap();
    let ws_provider = Arc::new(ws_provider);
    // let http_endpoint = std::env::var("ETHEREUM_HTTP_ENDPOINT")?;
    // let client = ClientBuilder::default()
    //     .layer(RetryBackoffLayer::new(4, 1000, 100))
    //     .http(http_endpoint.parse()?);
    // let provider: Arc<RootProvider<RetryBackoffService<Http<Client>>, Ethereum>> = Arc::new(ProviderBuilder::default().on_client(client));
    // let sim_provider = SimProvider::new(provider.clone(), 20522210, 20522310).await?;
    // let sim_provider = Arc::new(sim_provider);

    let ticks_per_side = 200;
    let block_number = ws_provider.get_block_number().await?;
    let block_number = 20522211;
    // let block_number = 20522212;
    let from_block = block_number + 1;
    let to_block = block_number + 100;
    let address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let mut pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
    tracing::info!(block_number=block_number, "loading original pool");
    pool.initialize(Some(block_number), ws_provider.clone()).await?;
    pool.sync_ticks(Some(block_number), ws_provider.clone()).await?;

    let stream_buffer = 1;
    let state_change_buffer = 1;

    let mock_block_stream = MockBlockStream::new(from_block, to_block, ws_provider.clone());
    let state_space_manager = UniswapPoolManager::new(
        pool.pool,
        block_number,
        stream_buffer,
        state_change_buffer,
        ws_provider.clone(),
        Some(mock_block_stream),
    );

    let (mut rx, _join_handles) = state_space_manager.subscribe_state_changes().await?;

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    loop {
        tokio::select! {
            _ = sigterm.recv() => break,
            _ = sigint.recv() => break,
            state_changes = rx.recv() => {
                if let Some(changes) = state_changes {
                    let pool_guard = state_space_manager.pool().await;
                    let res = pool_guard.deref();
                    let changes_block_number = changes.1;
                    // Load a fresh pool from the chain at the current block number
                    // let current_block_number = ws_provider.get_block_by_number(BlockNumberOrTag::Number(changes.1)).await?;
                    // tracing::info!("Loading fresh pool at block number: {}", changes_block_number);
                    let mut fresh_pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
                    fresh_pool.initialize(Some(changes_block_number), ws_provider.clone()).await?;
                    fresh_pool.sync_ticks(Some(changes_block_number), ws_provider.clone()).await?;

                    // Compare the fresh pool with the original pool
                    compare_pools(res, &fresh_pool,  changes_block_number);
                } else {
                    // Channel closed
                    break;
                }
            }
        }
    }

    tracing::info!("Shutting down gracefully");
    Ok(())
}

fn compare_pools(original: &UniswapV3Pool, fresh: &EnhancedUniswapV3Pool, block_number: BlockNumber) {
    let mut differences_found = false;

    if original.liquidity != fresh.liquidity {
        differences_found = true;
        let diff = original.liquidity.abs_diff(fresh.liquidity);
        tracing::warn!(block_number=block_number, "Liquidity:         A: {} | B: {} | Diff: {}", original.liquidity, fresh.liquidity, diff);
    }
    if original.sqrt_price != fresh.sqrt_price {
        differences_found = true;
        let diff = original.sqrt_price.abs_diff(fresh.sqrt_price);
        tracing::warn!(block_number=block_number, "Sqrt Price:        A: {} | B: {} | Diff: {}", original.sqrt_price, fresh.sqrt_price, diff);
    }
    if original.tick != fresh.tick {
        differences_found = true;
        let diff = original.tick.abs_diff(fresh.tick);
        tracing::warn!(block_number=block_number, "Tick:              A: {} | B: {} | Diff: {}", original.tick, fresh.tick, diff);
    }
    if original.tick_bitmap != fresh.tick_bitmap {
        differences_found = true;
        tracing::warn!(block_number=block_number, "Tick Bitmap:       A: {:?} | B: {:?}", original.tick_bitmap, fresh.tick_bitmap);
    }
    if original.ticks.len() != fresh.ticks.len() {
        differences_found = true;
        let diff = original.ticks.len().abs_diff(fresh.ticks.len());
        tracing::warn!(block_number=block_number, "Number of Ticks:   A: {} | B: {} | Diff: {}", original.ticks.len(), fresh.ticks.len(), diff);
    }

    let mut original_ticks: Vec<_> = original.ticks.keys().collect();
    original_ticks.sort();
    let mut fresh_ticks: Vec<_> = fresh.ticks.keys().collect();
    fresh_ticks.sort();
    if original_ticks != fresh_ticks {
        differences_found = true;
        tracing::warn!(block_number=block_number, "Tick mismatch detected!");
        let differing_ticks: Vec<_> = original_ticks.iter()
            .filter(|&tick| !fresh_ticks.contains(tick))
            .chain(fresh_ticks.iter().filter(|&tick| !original_ticks.contains(tick)))
            .collect();
        tracing::warn!(block_number=block_number, "Differing ticks: {:?}", differing_ticks);
    }
    for (idx, (tick, info)) in original.ticks.iter().enumerate() {
        match fresh.ticks.get(tick) {
            Some(fresh_info) => {
                if info.liquidity_gross != fresh_info.liquidity_gross {
                    differences_found = true;
                    tracing::warn!(block_number=block_number, "Tick {} (idx: {}): Liquidity gross mismatch: {} | {}", 
                             tick, idx, info.liquidity_gross, fresh_info.liquidity_gross);
                }
                if info.liquidity_net != fresh_info.liquidity_net {
                    differences_found = true;
                    tracing::warn!(block_number=block_number, "Tick {} (idx: {}): Liquidity net mismatch: {} | {}", 
                             tick, idx, info.liquidity_net, fresh_info.liquidity_net);
                }
                if info.initialized != fresh_info.initialized {
                    differences_found = true;
                    tracing::warn!(block_number=block_number, "Tick {} (idx: {}): Initialized flag mismatch: {} | {}", 
                             tick, idx, info.initialized, fresh_info.initialized);
                }
            },
            None => {
                differences_found = true;
                tracing::warn!(block_number=block_number, "Tick {} (idx: {}) not found in fresh pool", tick, idx);
            }
        }
    }

    if differences_found {
        tracing::error!(block_number=block_number, address=?original.address, "Differences found between pools");
    } else {
        tracing::info!(block_number=block_number, address=?original.address, "No differences found between pools");
    }
}