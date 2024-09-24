use std::str::FromStr;

use alloy::{
    network::{Ethereum, EthereumWallet, ReceiptResponse, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{
        address,
        aliases::{I24, U24},
        keccak256, Bytes, FixedBytes, U256
    },
    providers::{builder, ext::DebugApi, IpcConnect, Provider, ProviderBuilder},
    rpc::types::trace::geth::{
        GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
        GethDefaultTracingOptions
    },
    signers::local::PrivateKeySigner,
    sol_types::SolValue
};
use angstrom_types::{
    contract_bindings::{
        mintablemockerc20::MintableMockERC20,
        mockrewardsmanager::MockRewardsManager::{MockRewardsManagerInstance, PoolKey},
        poolgate::PoolGate,
        poolmanager::PoolManager
    },
    matching::SqrtPriceX96
};
use contract_payloads::{Asset, MockContractMessage, Pair, PoolUpdate, RewardsUpdate};
use pade::PadeEncode;
use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

mod contract_payloads;

async fn internal_anvil() -> eyre::Result<()> {
    // Spawn an Anvil instance
    let anvil = Anvil::new()
        .chain_id(1)
        .arg("--ipc")
        .arg("--code-size-limit")
        .arg("393216")
        .arg("--disable-block-gas-limit")
        .try_spawn()?;

    let endpoint = "/tmp/anvil.ipc";
    let ipc = IpcConnect::new(endpoint.to_string());
    let controller = anvil.addresses()[7];
    let sk: PrivateKeySigner = anvil.keys()[7].clone().into();

    let wallet = EthereumWallet::new(sk.clone());
    let provider = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ipc(ipc)
        .await?;
    do_contract_test(provider).await
}

async fn local_anvil() -> eyre::Result<()> {
    let sk: PrivateKeySigner = PrivateKeySigner::from_str(
        //"ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
        "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"
    )
    .unwrap();
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(EthereumWallet::new(sk))
        .on_http("http://localhost:8545".parse().unwrap());
    do_contract_test(provider).await
}
async fn do_contract_test<T, N, P>(provider: P) -> eyre::Result<()>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    N: alloy::providers::Network,
    P: alloy::providers::Provider<T, N>
{
    // Start up our Anvil instance
    // let provider = ProviderBuilder::new()
    //     .with_recommended_fillers()
    //     .on_anvil_with_wallet_and_config(|anvil| anvil.args(["--code-size-limit",
    // "30000"]));

    let controller = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
    // Deploy the supporting contracts
    let pool_manager = PoolManager::deploy(&provider).await.unwrap();
    let pool_manager_address = pool_manager.address();
    let pool_gate = PoolGate::deploy(&provider, *pool_manager.address()).await?;
    let mock_tob_addr = testing_tools::contracts::deploy_mock_rewards_manager(
        &provider,
        *pool_manager_address,
        controller
    )
    .await;
    let mock_tob = MockRewardsManagerInstance::new(mock_tob_addr, &provider);

    println!(
        "Contracts deployed:\n\tPoolManager: {}\n\tPoolGate: {}\n\tMockTOB: {}",
        pool_manager_address,
        pool_gate.address(),
        mock_tob_addr
    );
    // Mint our two ERC tokens
    let first_token = MintableMockERC20::deploy(&provider).await.unwrap();
    let second_token = MintableMockERC20::deploy(&provider).await.unwrap();
    let (asset0, asset1) = if first_token.address() < second_token.address() {
        (*first_token.address(), *second_token.address())
    } else {
        (*second_token.address(), *first_token.address())
    };

    println!("Tokens minted:\n\t{}\n\t{}", asset0, asset1);

    // Define some pool constants
    let pool_fee = U24::ZERO;
    let pool_tick_spacing = I24::unchecked_from(60_i32);

    // Find our initial SqrtPrice
    let initialSqrtPriceX96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(100020).unwrap());

    // Define the Pool ID
    let pool_id = keccak256(
        PoolKey {
            currency0:   asset0,
            currency1:   asset1,
            fee:         pool_fee,
            tickSpacing: pool_tick_spacing,
            hooks:       mock_tob_addr
        }
        .abi_encode()
    );

    println!("Pool ID: {:?}", pool_id);

    // First we configure the pool
    let pool_config_res = mock_tob
        .configurePool(asset0, asset1, 60, pool_fee)
        .from(controller)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await;
    println!("Pool config result: {:?}", pool_config_res);
    // The we set the PoolGate's hook to be our Mock Contract
    let set_hook_res = pool_gate
        .setHook(mock_tob_addr)
        .from(controller)
        .gas(30_000_000_u128)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await;
    println!("Set Hook result: {:?}", set_hook_res);
    // Then we use the PoolGate to initialize our pool
    let init_pool_res = pool_gate
        .initializePool(asset0, asset1, *initialSqrtPriceX96, 0)
        .from(controller)
        .gas(30_000_000_u128)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await;
    println!("Init pool result: {:?}", init_pool_res);

    // Then we try to add liquidity to that pool
    let liquidity_add_res = pool_gate
        .addLiquidity(
            asset0,
            asset1,
            I24::unchecked_from(99900),
            I24::unchecked_from(100140),
            U256::from(5_000_000_000_000_000_000_000_u128),
            FixedBytes::default()
        )
        .gas(1_000_000_u128)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await;
    println!("Add Liquidity result: {:?}", liquidity_add_res);
    // let call = pool_manager.initialize(key, U160::ZERO, Bytes::new());
    // let res = call.call().await;
    let message = MockContractMessage {
        assetList: vec![
            Asset { addr: asset0, borrow: 0, save: 0, settle: 0 },
            Asset { addr: asset1, borrow: 0, save: 0, settle: 0 },
        ],
        pairList:  vec![Pair {
            index0:       0,
            index1:       1,
            store_index:  0,
            price_1over0: initialSqrtPriceX96.into()
        }],
        update:    PoolUpdate {
            pair_index:       0,
            zero_for_one:     false,
            swap_in_quantity: 0,
            rewards_update:   RewardsUpdate {
                onlyCurrent:         false,
                onlyCurrentQuantity: 0,
                startTick:           I24::unchecked_from(100080_i32),
                startLiquidity:      5_000_000_000_000_000_000_000_u128,
                quantities:          vec![100000000_u128, 200000000_u128]
            }
        }
    };

    let update_res = mock_tob
        .update(Bytes::from(message.pade_encode()))
        .gas(10_000_000_u128)
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();

    println!("MockTOB Update result: {:?}", update_res);
    if !update_res.status() {
        // Our update failed
        let trace_options = GethDebugTracingOptions {
            config: GethDefaultTracingOptions {
                disable_storage: Some(true),
                enable_memory: Some(false),
                ..Default::default()
            },
            tracer: Some(GethDebugTracerType::BuiltInTracer(
                GethDebugBuiltInTracerType::CallTracer
            )),
            ..Default::default()
        };
        let trace = provider
            .debug_trace_transaction(update_res.transaction_hash(), trace_options)
            .await
            .unwrap();
        println!("CALL_TRACE: {trace:?}");
    }

    for tick in [99899, 100139].iter() {
        println!("Looking up output for tick {}", tick);
        let tick_check_res = mock_tob
            .getGrowthInsideTick(pool_id, I24::unchecked_from(*tick), pool_tick_spacing)
            .gas(10_000_000_u128)
            .call()
            .await
            .unwrap()
            ._0;
        println!("Success for tick {}: {:?}", tick, tick_check_res);
        // match tick_check_res {
        //     Ok(k) => println!("Success for tick {}: {:?}", tick, k),
        //     Err(e) => println!("Error for tick {}: {:?}", tick, e)
        // }
    }
    Ok(())

    // do_contract_test(provider, mock_tob_addr, controller).await;
}

#[tokio::main]
async fn main() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    println!("Hello, world!");
    local_anvil().await.unwrap();
    println!("Done!");
}
