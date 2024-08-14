use crate::cfmm::uniswap::loader::get_uniswap_v3_tick_data_batch_request;

use alloy::{
    network::Network,
    primitives::Address,
    providers::Provider,
    transports::Transport,
};
use amms::{
    errors::AMMError, amm::{
        uniswap_v3::{
            Info, UniswapV3Pool,
        },
    },
};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Arc;
use alloy_primitives::{B256, I256, U256};
use alloy_primitives::private::serde::{Deserialize, Serialize};
use amms::amm::{AutomatedMarketMaker};

pub const U256_1: U256 = U256::from_limbs([1, 0, 0, 0]);
pub const MIN_SQRT_RATIO: U256 = U256::from_limbs([4295128739, 0, 0, 0]);
pub const MAX_SQRT_RATIO: U256 = U256::from_limbs([6743328256752651558, 17280870778742802505, 4294805859, 0]);
#[derive(Default)]
pub struct StepComputations {
    pub sqrt_price_start_x_96: U256,
    pub tick_next: i32,
    pub initialized: bool,
    pub sqrt_price_next_x96: U256,
    pub amount_in: U256,
    pub amount_out: U256,
    pub fee_amount: U256,
}

pub struct CurrentState {
    amount_specified_remaining: I256,
    amount_calculated: I256,
    sqrt_price_x_96: U256,
    tick: i32,
    liquidity: u128,
}

// at around 190 is when "max code size exceeded" comes up
const MAX_TICKS_PER_REQUEST: u16 = 150;
#[derive(Debug, Clone)]
pub struct EnhancedUniswapV3Pool {
    pub pool: UniswapV3Pool,
    initial_ticks_per_side: u16,
}

impl EnhancedUniswapV3Pool {
    pub fn new(address: Address, initial_ticks_per_side: u16) -> Self {
        Self {
            pool: UniswapV3Pool { address, ..Default::default() },
            initial_ticks_per_side,
        }
    }

    pub async fn initialize<T, N, P>(
        &mut self,
        block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        self.pool.populate_data(block_number, provider.clone()).await
    }

    pub fn is_initialized(&self) -> bool {
        self.pool.token_a != Address::default() && self.pool.token_b != Address::default()
    }

    pub async fn sync_ticks<T, N, P>(
        &mut self,
        block_number: Option<u64>,
        provider: Arc<P>,
    ) -> Result<(), AMMError>
    where
        T: Transport + Clone,
        N: Network,
        P: Provider<T, N>,
    {
        if !self.is_initialized() {
            return Err(AMMError::PoolDataError);
        }

        // refreshing the state just in case
        self.pool.ticks = HashMap::new();
        self.pool.tick_bitmap = HashMap::new();

        let mut remaining_ticks = self.initial_ticks_per_side;
        let mut current_tick = self.pool.tick;

        // Fetch left ticks
        let mut left_ticks = Vec::new();
        while remaining_ticks > 0 {
            let ticks_to_fetch = remaining_ticks.min(MAX_TICKS_PER_REQUEST);
            let (mut fetched_ticks, _) = get_uniswap_v3_tick_data_batch_request(
                &self.pool,
                current_tick,
                true,
                ticks_to_fetch,
                block_number,
                provider.clone(),
            )
                .await?;
            left_ticks.extend(fetched_ticks.drain(..));
            remaining_ticks -= ticks_to_fetch;
            if let Some(last_tick) = left_ticks.last() {
                current_tick = last_tick.tick - self.pool.tick_spacing;
            } else {
                break;
            }
        }

        // Reset for right side
        remaining_ticks = self.initial_ticks_per_side;
        current_tick = self.pool.tick;
        // Fetch right ticks
        let mut right_ticks = Vec::new();
        while remaining_ticks > 0 {
            let ticks_to_fetch = remaining_ticks.min(MAX_TICKS_PER_REQUEST);
            let (mut fetched_ticks, _) = get_uniswap_v3_tick_data_batch_request(
                &self.pool,
                current_tick,
                false,
                ticks_to_fetch,
                block_number,
                provider.clone(),
            )
                .await?;
            right_ticks.extend(fetched_ticks.drain(..));
            remaining_ticks -= ticks_to_fetch;
            if let Some(last_tick) = right_ticks.last() {
                current_tick = last_tick.tick + self.pool.tick_spacing;
            } else {
                break;
            }
        }


        for tick in left_ticks.into_iter().chain(right_ticks.into_iter()) {
            if tick.initialized {
                self.pool.ticks.insert(
                    tick.tick,
                    Info {
                        liquidity_gross: tick.liquidity_gross,
                        liquidity_net: tick.liquidity_net,
                        initialized: true,
                    },
                );
                self.pool.flip_tick(tick.tick, self.pool.tick_spacing);
            }
        }

        Ok(())
    }
}

impl std::ops::Deref for EnhancedUniswapV3Pool {
    type Target = UniswapV3Pool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl std::ops::DerefMut for EnhancedUniswapV3Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;

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
    use alloy_primitives::{BlockHash, LogData, TxHash, B256};
    use serde_json;

    use super::*;

    async fn setup_provider() -> Arc<RootProvider<RetryBackoffService<Http<Client>>, Ethereum>> {
        let rpc_endpoint =
            std::env::var("ETHEREUM_RPC_ENDPOINT").expect("ETHEREUM_RPC_ENDPOINT must be set");
        let url = rpc_endpoint.parse().unwrap();
        let client = ClientBuilder::default()
            .layer(RetryBackoffLayer::new(4, 1000, 100))
            .http(url);
        let provider: RootProvider<RetryBackoffService<Http<Client>>, Ethereum> =
            ProviderBuilder::default().on_client(client);
        Arc::new(provider)
    }

    async fn setup_pool(
        provider: Arc<RootProvider<RetryBackoffService<Http<Client>>, Ethereum>>,
        block_number: u64,
        ticks_per_side: u16,
    ) -> EnhancedUniswapV3Pool {
        let address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
        let mut pool = EnhancedUniswapV3Pool::new(address, ticks_per_side);
        pool.initialize(Some(block_number), provider.clone())
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_load_pool() {
        let block_number = 20498069;
        let ticks_per_side = 10;
        let provider = setup_provider().await;
        let pool = setup_pool(provider.clone(), block_number, ticks_per_side).await;

        assert_eq!(pool.address, address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"));
        assert_eq!(pool.token_a, address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"));
        assert_eq!(pool.token_a_decimals, 6);
        assert_eq!(pool.token_b, address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"));
        assert_eq!(pool.token_b_decimals, 18);
        assert!(pool.liquidity > 0);
        assert!(pool.sqrt_price > U256::ZERO);
        assert_eq!(pool.fee, 500);
        assert_eq!(pool.tick, 197547);
        assert_eq!(pool.tick_spacing, 10);
        assert!(pool.tick_bitmap.is_empty());
        assert!(pool.ticks.is_empty());
    }

    #[tokio::test]
    async fn test_swap_in_pool() {
        let swap_block = 20480828;
        let previous_block = swap_block - 1;
        let ticks_per_side = 10;
        let provider = setup_provider().await;
        let mut pool = setup_pool(provider.clone(), previous_block, ticks_per_side).await;
        pool.sync_ticks(Some(previous_block), provider.clone()).await.expect("failed to sync ticks");

        // Check pool state before the swap
        assert_eq!(pool.liquidity, 2694411943070307563, "Liquidity mismatch before swap");
        assert_eq!(pool.sqrt_price, U256::from_str("1598677116547625698577517567213640").unwrap(), "Sqrt price mismatch before swap");
        assert_eq!(pool.tick, 198257, "Tick mismatch before swap");

        // Perform the swap
        let token_in = pool.token_a;
        let amount_in = U256::from_str("66741928781").unwrap(); // 66741.928781 * 10^6
        let amount_out = pool.simulate_swap_mut(token_in, amount_in).expect("Swap simulation failed");

        // Check the results
        assert_eq!(amount_out, U256::from_str("27147321967958680641").unwrap(), "Incorrect amount out");
        assert_eq!(pool.sqrt_price, U256::from_str("1597878859828850322653524929880487").unwrap(), "Incorrect sqrtPriceX96");
        assert_eq!(pool.liquidity, 2694411943070307563, "Incorrect liquidity");
        assert_eq!(pool.tick, 198247, "Incorrect tick");

        // Compare with the state after the swap
        let mut after_swap_pool = setup_pool(provider.clone(), swap_block, ticks_per_side).await;
        after_swap_pool.sync_ticks(Some(swap_block), provider.clone()).await.expect("failed to sync ticks");

        assert_eq!(pool.tick_spacing, after_swap_pool.tick_spacing, "Tick spacing mismatch");
        assert_eq!(pool.liquidity, after_swap_pool.liquidity, "Liquidity mismatch");
        assert_eq!(pool.sqrt_price, after_swap_pool.sqrt_price, "Sqrt price mismatch");
        assert_eq!(pool.tick, after_swap_pool.tick, "Tick mismatch");
        assert_eq!(pool.tick_bitmap, after_swap_pool.tick_bitmap, "Tick bitmap mismatch");
    }

    #[tokio::test]
    async fn test_burn_pool_outside_range() {
        let burn_block = 20505115;
        let previous_block = burn_block - 1;
        // needs a fairly wide range
        let ticks_per_side = 250;
        let provider = setup_provider().await;
        let mut pool = setup_pool(provider.clone(), previous_block, ticks_per_side).await;
        pool.sync_ticks(Some(previous_block), provider.clone()).await.expect("failed to sync ticks");
        // owner: 0xC36442b4a4522E871399CD717aBDD847Ab11FE88, tickLower: 195090, tickUpper: 195540, amount: 136670924187209102, amount0: 0, amount1: 53560594103808093362
        pool.sync_from_burn_log(RpcLog {
            inner: AlloyLog {
                address: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
                data: LogData::new(
                    vec![
                        B256::from_slice(&hex::decode("0c396cd989a39f4459b5fa1aed6a9a8dcdbc45908acfd67e028cd568da98982c").unwrap()),
                        B256::from_slice(&hex::decode("000000000000000000000000c36442b4a4522e871399cd717abdd847ab11fe88").unwrap()),
                        B256::from_slice(&hex::decode("000000000000000000000000000000000000000000000000000000000002fa12").unwrap()),
                        B256::from_slice(&hex::decode("000000000000000000000000000000000000000000000000000000000002fbd4").unwrap()),
                    ],
                    Bytes::copy_from_slice(&hex::decode("00000000000000000000000000000000000000000000000001e58d7b3f4d8d8e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002e74d748fac5cecb2").unwrap()),
                ).unwrap(),
            },
            block_hash: Some(BlockHash::from_slice(&hex::decode("8777a4798a4f74c24a30847c646921939ae8422f55fb1ccaa2718fa7bd3b3381").unwrap())),
            block_number: Some(burn_block),
            block_timestamp: None,
            transaction_hash: Some(TxHash::from_slice(&hex::decode("20b553d3067e12892051e2d07e63cd12bd58ee7f31fa5402f7c4065505890d39").unwrap())),
            transaction_index: Some(21),
            log_index: Some(100),
            removed: false,
        }).unwrap();

        let mut after_burn_pool = setup_pool(provider.clone(), burn_block, ticks_per_side).await;
        after_burn_pool.sync_ticks(Some(burn_block), provider.clone()).await.expect("failed to sync ticks");

        // Compare fields of after_burn_pool and pool
        assert_eq!(pool.address, after_burn_pool.address, "Address mismatch");
        assert_eq!(pool.token_a, after_burn_pool.token_a, "Token A mismatch");
        assert_eq!(pool.token_b, after_burn_pool.token_b, "Token B mismatch");
        assert_eq!(pool.token_a_decimals, after_burn_pool.token_a_decimals, "Token A decimals mismatch");
        assert_eq!(pool.token_b_decimals, after_burn_pool.token_b_decimals, "Token B decimals mismatch");
        assert_eq!(pool.fee, after_burn_pool.fee, "Fee mismatch");
        assert_eq!(pool.tick_spacing, after_burn_pool.tick_spacing, "Tick spacing mismatch");
        assert_eq!(pool.liquidity, after_burn_pool.liquidity, "Liquidity mismatch");
        assert_eq!(pool.sqrt_price, after_burn_pool.sqrt_price, "Sqrt price mismatch");
        assert_eq!(pool.tick, after_burn_pool.tick, "Tick mismatch");
        assert_eq!(pool.tick_bitmap, after_burn_pool.tick_bitmap, "Tick bitmap mismatch");
        assert_eq!(pool.ticks.len(), after_burn_pool.ticks.len(), "Number of ticks mismatch");
        for (tick, info) in pool.ticks.iter() {
            let after_burn_info = after_burn_pool.ticks.get(tick).expect("Tick not found in after_burn_pool");
            assert_eq!(info.liquidity_gross, after_burn_info.liquidity_gross, "Liquidity gross mismatch for tick {}", tick);
            assert_eq!(info.liquidity_net, after_burn_info.liquidity_net, "Liquidity net mismatch for tick {}", tick);
            assert_eq!(info.initialized, after_burn_info.initialized, "Initialized flag mismatch for tick {}", tick);
        }
    }

    #[tokio::test]
    async fn test_mint_pool() {
        // TODO: add an interesting case for minting
        let block_number = 20505110;
        let ticks_per_side = 10;
        let provider = setup_provider().await;
        let pool = setup_pool(provider.clone(), block_number, ticks_per_side).await;
    }

    #[tokio::test]
    async fn test_load_ticks() {
        let block_number = 20498069;
        let ticks_per_side = 10;
        let provider = setup_provider().await;
        let mut pool = setup_pool(provider.clone(), block_number, ticks_per_side).await;
        pool.sync_ticks(Some(block_number), provider.clone()).await.expect("failed to sync ticks");

        // Print all loaded ticks
        assert_eq!(pool.ticks.len(), 20, "No ticks were loaded");
        let expected_ticks = vec![
            (197430, 7384325092267, -7384325092267),
            (197440, 111865134864137602, 102733576284820334),
            (197450, 2771971676516941, -2771834237563469),
            (197460, 3859798908378, 3722359954906),
            (197470, 1193748914015, -1193748914015),
            (197480, 44433936013305, -44433936013305),
            (197500, 309999197921697743, 273427767344046023),
            (197520, 192996722135755, 192996722135755),
            (197530, 539241987876, 539241987876),
            (197540, 13222195433615963, 11816097785443223),
            (197560, 27595709583757, 27595709583757),
            (197570, 6112184035950571, -5149755217930223),
            (197580, 190509459765420, -77595848930828),
            (197590, 4325345455099573, 4139136533538185),
            (197600, 92637809125804, -92637809125804),
            (197610, 2047252361319, -2047252361319),
            (197620, 14063986970207, 5353547592293),
            (197630, 1349269385245271, 1345601874435831),
            (197640, 6516724029717570, 6516724029717570),
            (197650, 7142580627476203506, 7142569105915081148),
        ];

        for (tick, expected_gross, expected_net) in expected_ticks {
            let tick_data = pool.ticks.get(&tick);
            assert!(tick_data.is_some(), "tick {} should be initialized", tick);

            let info = tick_data.unwrap();
            assert_eq!(
                info.liquidity_gross, expected_gross,
                "Mismatch in liquidity_gross for tick {}",
                tick
            );
            assert_eq!(
                info.liquidity_net, expected_net,
                "Mismatch in liquidity_net for tick {}",
                tick
            );
            assert!(info.initialized, "Tick {} should be initialized", tick);
        }
    }

    #[tokio::test]
    async fn test_multiple_swaps() {
        let block_number = 20522215;
        let ticks_per_side = 200;
        let provider = setup_provider().await;
        let mut pool = setup_pool(provider.clone(), block_number, ticks_per_side).await;
        pool.sync_ticks(Some(block_number), provider.clone()).await.expect("failed to sync ticks");

        assert_eq!(pool.sqrt_price, U256::from_str_radix("1522541228652157746214186795710203", 10).unwrap());
        assert_eq!(pool.liquidity, 14623537689052122812u128);
        assert_eq!(pool.tick, 197281);

        // First swap
        let token_in = pool.token_b;
        let amount_in = U256::from_str_radix("300532960990132029", 10).unwrap();
        let amount_out = pool.simulate_swap_mut(token_in, amount_in).expect("First swap simulation failed");
        assert_eq!(amount_out, U256::from_str_radix("813383744", 10).unwrap());
        assert_eq!(pool.sqrt_price, U256::from_str_radix("1522542856081131714601312592562953", 10).unwrap());
        assert_eq!(pool.liquidity, 14623537689052122812u128);
        assert_eq!(pool.tick, 197281);

        // Second swap causing issues
        let token_in = pool.token_b;
        let amount_in = U256::from_str_radix("36948528148148111", 10).unwrap();
        let amount_out = pool.simulate_swap_mut(token_in, amount_in).expect("Second swap simulation failed");
        assert_eq!(amount_out, U256::from_str_radix("100000000", 10).unwrap());
        assert_eq!(pool.sqrt_price, U256::from_str_radix("1522543056162696996744021728687215", 10).unwrap());
        assert_eq!(pool.liquidity, 14623537689052122812u128);
        assert_eq!(pool.tick, 197281);

        // Third swap
        let token_in = pool.token_b;
        let amount_in = U256::from_str_radix("41238263733788147", 10).unwrap();
        let amount_out = pool.simulate_swap_mut(token_in, amount_in).expect("Third swap simulation failed");
        assert_eq!(amount_out, U256::from_str_radix("111610000", 10).unwrap());
        assert_eq!(pool.sqrt_price, U256::from_str_radix("1522543279473794107054723771988160", 10).unwrap());
        assert_eq!(pool.liquidity, 14623537689052122812u128);
        assert_eq!(pool.tick, 197281);
    }

    #[tokio::test]
    #[ignore]
    async fn test_pool_manager() {
        let block_number = 20498069;
        let ticks_per_side = 10;
        let provider = setup_provider().await;
        let mut pool = setup_pool(provider.clone(), block_number, ticks_per_side).await;
        pool.sync_ticks(Some(block_number), provider.clone()).await.expect("failed to sync ticks");

        let ws_endpoint = std::env::var("ETHEREUM_WS_ENDPOINT").unwrap();

        // Initialize WS provider
        let ws = WsConnect::new(ws_endpoint);
        let provider = Arc::new(ProviderBuilder::new().on_ws(ws).await.unwrap());

        let step: u64 = 1000;
        // let state_space_manager = StateSpaceManager::new(vec![pool], block_number, 100, 100, provider);
        // let (mut rx, _join_handles) = state_space_manager.subscribe_state_changes().await?;
        // for _ in 0..10 {
        //     if let Some(state_changes) = rx.recv().await {
        //         println!("State changes: {:?}", state_changes);
        //     }
        // }
    }
}
