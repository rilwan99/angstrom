extern crate arraydeque;
use std::{ops::Deref, sync::Arc};

use alloy::{
    network::Ethereum,
    primitives::address,
    providers::{ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend
};
use alloy_primitives::BlockNumber;
use amms::amm::uniswap_v3::UniswapV3Pool;
use matching_engine::cfmm::uniswap::{
    mock_block_stream::MockBlockStream, pool::EnhancedUniswapV3Pool,
    pool_manager::UniswapPoolManager
};
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    std::env::set_var("RUST_LOG", "amms=trace,matching_engine=debug,info");
    tracing_subscriber::fmt::init();
    let log_level = tracing::level_filters::LevelFilter::current();
    tracing::info!("Logging initialized at level: {}", log_level);

    let ws_endpoint = std::env::var("ETHEREUM_WS_ENDPOINT")?;
    let ws = WsConnect::new(ws_endpoint);
    let ws_provider: RootProvider<PubSubFrontend, Ethereum> =
        ProviderBuilder::default().on_ws(ws).await.unwrap();
    let ws_provider = Arc::new(ws_provider);
    let ticks_per_side = 400;
    let block_number = 20522309;
    let from_block = block_number + 1;
    let to_block = block_number + 100;
    let address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let mut pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
    pool.set_sim_swap_sync(true);
    tracing::info!(block_number = block_number, "loading old pool");
    pool.initialize(Some(block_number), ws_provider.clone())
        .await?;
    pool.sync_ticks(Some(block_number), ws_provider.clone())
        .await?;
    let stream_buffer = 1;
    let state_change_buffer = 1;

    let mock_block_stream = MockBlockStream::new(ws_provider.clone(), from_block, to_block);
    let mut state_space_manager = UniswapPoolManager::new(
        pool,
        block_number,
        stream_buffer,
        state_change_buffer,
        ws_provider.clone()
    );
    state_space_manager.set_mock_block_stream(mock_block_stream);

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
                    let mut fresh_pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
                    fresh_pool.initialize(Some(changes_block_number), ws_provider.clone()).await?;
                    fresh_pool.sync_ticks(Some(changes_block_number), ws_provider.clone()).await?;

                    // Compare the new pool with the old pool
                    compare_pools(res, &fresh_pool, changes_block_number);
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

fn compare_pools(old: &UniswapV3Pool, new: &EnhancedUniswapV3Pool, block_number: BlockNumber) {
    let mut differences_found = false;

    if old.liquidity != new.liquidity {
        differences_found = true;
        let diff = old.liquidity.abs_diff(new.liquidity);
        tracing::warn!(
            block_number = block_number,
            "Liquidity:         Old: {} | New: {} | Diff: {}",
            old.liquidity,
            new.liquidity,
            diff
        );
    }

    if old.sqrt_price != new.sqrt_price {
        differences_found = true;
        let diff = old.sqrt_price.abs_diff(new.sqrt_price);
        tracing::warn!(
            block_number = block_number,
            "Sqrt Price:        Old: {} | New: {} | Diff: {}",
            old.sqrt_price,
            new.sqrt_price,
            diff
        );
    }

    if old.tick != new.tick {
        differences_found = true;
        let diff = old.tick.abs_diff(new.tick);
        tracing::warn!(
            block_number = block_number,
            "Tick:              Old: {} | New: {} | Diff: {}",
            old.tick,
            new.tick,
            diff
        );
    }

    // since loading does not account for tick window moving up or down
    // will show that there is a difference in bitmap, but that won't be relevant
    // because left/right tick end would be -/+1 for the fresh pool
    // if old.tick_bitmap != new.tick_bitmap {
    //     differences_found = true;
    //     tracing::warn!(
    //         block_number = block_number,
    //         "Tick Bitmap:       Old: {:?} | New: {:?}",
    //         old.tick_bitmap,
    //         new.tick_bitmap
    //     );
    // }

    if old.ticks.len() != new.ticks.len() {
        differences_found = true;
        let diff = old.ticks.len().abs_diff(new.ticks.len());
        tracing::warn!(
            block_number = block_number,
            "Number of Ticks:   Old: {} | New: {} | Diff: {}",
            old.ticks.len(),
            new.ticks.len(),
            diff
        );
        if let (Some(old_min), Some(new_min)) = (old.ticks.keys().min(), new.ticks.keys().min()) {
            tracing::warn!(
                block_number = block_number,
                "Min tick:   Old: {} | New: {}",
                old_min,
                new_min
            );
        }
        if let (Some(old_max), Some(new_max)) = (old.ticks.keys().max(), new.ticks.keys().max()) {
            tracing::warn!(
                block_number = block_number,
                "Max tick:   Old: {} | New: {}",
                old_max,
                new_max
            );
        }
        let original_ticks: std::collections::HashSet<_> = old.ticks.keys().collect();
        let fresh_ticks: std::collections::HashSet<_> = new.ticks.keys().collect();
        let left_diff: Vec<_> = fresh_ticks.difference(&original_ticks).collect();
        let right_diff: Vec<_> = original_ticks.difference(&fresh_ticks).collect();
        if !left_diff.is_empty() {
            tracing::warn!(
                block_number = block_number,
                "ticks not in old, but in new: {:?}",
                left_diff
            );
        }
        if !right_diff.is_empty() {
            tracing::warn!(
                block_number = block_number,
                "ticks not in new, but in old: {:?}",
                right_diff
            );
        }
    }
    let mut original_ticks: Vec<_> = old.ticks.keys().collect();
    original_ticks.sort();
    let mut fresh_ticks: Vec<_> = new.ticks.keys().collect();
    fresh_ticks.sort();

    // since loading does not account for tick window moving up or down if
    // fresh_ticks are used then it could happen that there are ticks in the
    // fresh pool that are not in the old simply because left/right end would be
    // -/+1 for the fresh pool
    for (idx, tick) in original_ticks.iter().enumerate() {
        let original_info = &old.ticks[tick];
        if let Some(fresh_info) = new.ticks.get(tick) {
            if original_info.liquidity_gross != fresh_info.liquidity_gross {
                differences_found = true;
                tracing::warn!(
                    block_number = block_number,
                    "Tick {} (idx: {}): Liquidity gross mismatch: {} | {}",
                    tick,
                    idx,
                    original_info.liquidity_gross,
                    fresh_info.liquidity_gross
                );
            }
            if original_info.liquidity_net != fresh_info.liquidity_net {
                differences_found = true;
                tracing::warn!(
                    block_number = block_number,
                    "Tick {} (idx: {}): Liquidity net mismatch: {} | {}",
                    tick,
                    idx,
                    original_info.liquidity_net,
                    fresh_info.liquidity_net
                );
            }
            if original_info.initialized != fresh_info.initialized {
                differences_found = true;
                tracing::warn!(
                    block_number = block_number,
                    "Tick {} (idx: {}): Initialized flag mismatch: {} | {}",
                    tick,
                    idx,
                    original_info.initialized,
                    fresh_info.initialized
                );
            }
        }
    }
    if differences_found {
        tracing::error!(block_number=block_number, address=?old.address, "differences found between pools");
    } else {
        tracing::info!(block_number=block_number, address=?old.address, "pools are the same");
    }
}
