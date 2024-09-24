use alloy::{
    primitives::{
        address,
        aliases::{I24, U24},
        FixedBytes, U256
    },
    providers::ProviderBuilder
};
use angstrom_types::{
    contract_bindings::{
        mockerc20::MockERC20, mockrewardsmanager::MockRewardsManager::MockRewardsManagerInstance,
        poolgate::PoolGate, poolmanager::PoolManager
    },
    matching::SqrtPriceX96
};
use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

async fn deploy_basic_contracts() {
    // Start up our Anvil instance
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet_and_config(|anvil| anvil.args(["--code-size-limit", "30000"]));
    let controller = address!("14dC79964da2C08b23698B3D3cc7Ca32193d9955");
    // Deploy the supporting contracts
    let pool_manager = PoolManager::deploy(&provider).await.unwrap();
    let pool_gate = PoolGate::deploy(&provider, *pool_manager.address())
        .await
        .unwrap();
    let mock_tob_addr = testing_tools::contracts::deploy_mock_rewards_manager(
        &provider,
        *pool_manager.address(),
        controller
    )
    .await;
    let mock_tob = MockRewardsManagerInstance::new(mock_tob_addr, &provider);

    // Mint our two ERC tokens
    let first_token = MockERC20::deploy(&provider).await.unwrap();
    let second_token = MockERC20::deploy(&provider).await.unwrap();
    let (asset0, asset1) = if first_token.address() < second_token.address() {
        (*first_token.address(), *second_token.address())
    } else {
        (*second_token.address(), *first_token.address())
    };

    // Find our initial SqrtPrice
    let initialSqrtPriceX96 = SqrtPriceX96::from(get_sqrt_ratio_at_tick(100020).unwrap());

    // Initialize our things
    // let key = PoolKey {
    //     currency0:   Address::random(),
    //     currency1:   Address::random(),
    //     fee:         U24::ZERO,
    //     hooks:       Address::default(),
    //     tickSpacing: I24::unchecked_from(60_i32)
    // };
    // First we configure the pool
    let pool_config_res = mock_tob
        .configurePool(asset0, asset1, 60, U24::ZERO)
        .from(controller)
        .send()
        .await
        .unwrap()
        .watch()
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
        .watch()
        .await;
    println!("Set Hook result: {:?}", set_hook_res);
    // Then we use the PoolGate to initialize our pool
    let init_pool_res = pool_gate
        .initializePool(asset0, asset1, *initialSqrtPriceX96)
        .from(controller)
        .gas(30_000_000_u128)
        .send()
        .await
        .unwrap()
        .watch()
        .await;
    println!("Init pool result: {:?}", init_pool_res);
    // Then we try to add liquidity to that pool
    let liquidity_add_res = pool_gate
        .addLiquidity(
            asset0,
            asset1,
            I24::unchecked_from(100020),
            I24::unchecked_from(100080),
            U256::from(5000000000000000000000_u128),
            FixedBytes::default()
        )
        .from(controller)
        .gas(30_000_000_u128)
        .send()
        .await
        .unwrap()
        .watch()
        .await;
    println!("Add Liquidity result: {:?}", liquidity_add_res);
    // let call = pool_manager.initialize(key, U160::ZERO, Bytes::new());
    // let res = call.call().await;

    // do_contract_test(provider, mock_tob_addr, controller).await;
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    deploy_basic_contracts().await;
    println!("Done!");
}
