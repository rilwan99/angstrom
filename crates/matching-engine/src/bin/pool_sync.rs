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
use std::{ops::Deref, sync::Arc};
use alloy::pubsub::PubSubFrontend;
use amms::amm::AMM;
use amms::amm::uniswap_v3::UniswapV3Pool;
use angstrom_types::primitive::poolManagerCall;
use matching_engine::cfmm::uniswap::pool::EnhancedUniswapV3Pool;
use matching_engine::cfmm::uniswap::pool_manager::UniswapPoolManager;
use tokio::signal::unix::{signal, SignalKind};
use futures::StreamExt;
extern crate arraydeque;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let ws_endpoint = std::env::var("ETHEREUM_WS_ENDPOINT")?;
    let ws = WsConnect::new(ws_endpoint);
    let ws_provider: RootProvider<PubSubFrontend, Ethereum> = ProviderBuilder::default().on_ws(ws).await.unwrap();
    let ws_provider = Arc::new(ws_provider);
    let ticks_per_side = 100;
    let block_number = ws_provider.get_block_number().await?;
    let address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let mut pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
    pool.initialize(Some(block_number), ws_provider.clone()).await?;
    pool.sync_ticks(Some(block_number), ws_provider.clone()).await?;

    let stream_buffer = 1;
    let state_change_buffer = 1;
    let state_space_manager = UniswapPoolManager::new(
        pool.pool,
        block_number,
        stream_buffer,
        state_change_buffer,
        ws_provider.clone(),
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
                    println!("State changes: {:?}", changes);

                    // Load a fresh pool from the chain at the current block number
                    let current_block_number = ws_provider.get_block_number().await?;
                    let mut fresh_pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
                    fresh_pool.initialize(Some(current_block_number), ws_provider.clone()).await?;
                    fresh_pool.sync_ticks(Some(current_block_number), ws_provider.clone()).await?;

                    // Compare the fresh pool with the original pool
                    compare_pools(res, &fresh_pool);
                } else {
                    // Channel closed
                    break;
                }
            }
        }
    }

    println!("Shutting down gracefully");
    Ok(())
}

fn compare_pools(original: &UniswapV3Pool, fresh: &EnhancedUniswapV3Pool) {
    println!("Comparing pools: {}", original.address);
    println!("Address:           A: {} | B: {}", original.address, fresh.address);
    println!("Token A:           A: {} | B: {}", original.token_a, fresh.token_a);
    println!("Token B:           A: {} | B: {}", original.token_b, fresh.token_b);
    println!("Token A Decimals:  A: {} | B: {}", original.token_a_decimals, fresh.token_a_decimals);
    println!("Token B Decimals:  A: {} | B: {}", original.token_b_decimals, fresh.token_b_decimals);
    println!("Fee:               A: {} | B: {}", original.fee, fresh.fee);
    println!("Tick Spacing:      A: {} | B: {}", original.tick_spacing, fresh.tick_spacing);
    println!("Liquidity:         A: {} | B: {}", original.liquidity, fresh.liquidity);
    println!("Sqrt Price:        A: {} | B: {}", original.sqrt_price, fresh.sqrt_price);
    println!("Tick:              A: {} | B: {}", original.tick, fresh.tick);
    println!("Tick Bitmap:       A: {:?} | B: {:?}", original.tick_bitmap, fresh.tick_bitmap);
    println!("Number of Ticks:   A: {} | B: {}", original.ticks.len(), fresh.ticks.len());

    // assert_eq!(original.address, fresh.address, "Address mismatch");
    // assert_eq!(original.token_a, fresh.token_a, "Token A mismatch");
    // assert_eq!(original.token_b, fresh.token_b, "Token B mismatch");
    // assert_eq!(original.token_a_decimals, fresh.token_a_decimals, "Token A decimals mismatch");
    // assert_eq!(original.token_b_decimals, fresh.token_b_decimals, "Token B decimals mismatch");
    // assert_eq!(original.fee, fresh.fee, "Fee mismatch");
    // assert_eq!(original.tick_spacing, fresh.tick_spacing, "Tick spacing mismatch");
    // assert_eq!(original.liquidity, fresh.liquidity, "Liquidity mismatch");
    // assert_eq!(original.sqrt_price, fresh.sqrt_price, "Sqrt price mismatch");
    // assert_eq!(original.tick, fresh.tick, "Tick mismatch");
    // assert_eq!(original.tick_bitmap, fresh.tick_bitmap, "Tick bitmap mismatch");
    // assert_eq!(original.ticks.len(), fresh.ticks.len(), "Number of ticks mismatch");

    // Check for tick mismatches before comparison
    let original_ticks: Vec<_> = original.ticks.keys().collect();
    let fresh_ticks: Vec<_> = fresh.ticks.keys().collect();
    if original_ticks != fresh_ticks {
        println!("Tick mismatch detected!");
        println!("Original ticks: {:?}", original_ticks);
        println!("Fresh ticks: {:?}", fresh_ticks);
    }
    for (tick, info) in original.ticks.iter() {
        let fresh_info = fresh.ticks.get(tick).expect("Tick not found in fresh pool");
        if info.liquidity_gross != fresh_info.liquidity_gross {
            println!("Tick {}: Liquidity gross mismatch: {} | {}", 
                     tick, info.liquidity_gross, fresh_info.liquidity_gross);
        }
        if info.liquidity_net != fresh_info.liquidity_net {
            println!("Tick {}: Liquidity net mismatch: {} | {}", 
                     tick, info.liquidity_net, fresh_info.liquidity_net);
        }
        if info.initialized != fresh_info.initialized {
            println!("Tick {}: Initialized flag mismatch: {} | {}", 
                     tick, info.initialized, fresh_info.initialized);
        }
    }
    println!("Pool comparison completed.");
}