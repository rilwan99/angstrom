use alloy::{
    node_bindings::AnvilInstance,
    primitives::{
        aliases::{I24, U24},
        Address, U256
    },
    pubsub::PubSubFrontend
};
use alloy_primitives::FixedBytes;
use angstrom_types::{
    contract_bindings::{
        mockrewardsmanager::MockRewardsManager::MockRewardsManagerInstance,
        poolgate::PoolGate::{self, PoolGateInstance},
        poolmanager::PoolManager
    },
    matching::SqrtPriceX96,
    primitive::PoolKey
};

use super::{
    anvil::{spawn_anvil, AnvilWalletRpc},
    deploy::{mockreward::deploy_mock_rewards_manager, tokens::mint_token_pair},
    DebugTransaction
};

pub struct RewardTestEnv {
    anvil:        AnvilInstance,
    provider:     AnvilWalletRpc,
    controller:   Address,
    pool_manager: Address,
    pool_gate:    Address,
    mock_reward:  Address
}

impl RewardTestEnv {
    pub async fn new() -> eyre::Result<Self> {
        println!("Spawning Anvil...");
        let (anvil, provider) = spawn_anvil(7).await?;
        let controller = anvil.addresses()[7];
        println!("Deploying pool manager...");
        let pool_manager = *PoolManager::deploy(&provider).await.unwrap().address();
        println!("Deploying pool gate...");
        let pool_gate_instance = PoolGate::deploy(&provider, pool_manager).await.unwrap();
        let pool_gate = *pool_gate_instance.address();
        println!("Deploying mock rewards manager...");
        let mock_reward = deploy_mock_rewards_manager(&provider, pool_manager, controller).await;
        // Set the PoolGate's hook to be our Mock
        println!("Setting PoolGate hook...");
        pool_gate_instance
            .setHook(mock_reward)
            .from(controller)
            .run_safe()
            .await?;
        println!("Deploy complete!");
        Ok(Self { anvil, provider, controller, pool_manager, pool_gate, mock_reward })
    }

    pub fn mock_reward(&self) -> MockRewardsManagerInstance<PubSubFrontend, &AnvilWalletRpc> {
        MockRewardsManagerInstance::new(self.mock_reward, &self.provider)
    }

    pub fn pool_gate(&self) -> PoolGateInstance<PubSubFrontend, &AnvilWalletRpc> {
        PoolGateInstance::new(self.pool_gate, &self.provider)
    }

    pub async fn create_pool(
        &self,
        initial_price: SqrtPriceX96,
        tick_spacing: I24,
        pool_fee: U24
    ) -> eyre::Result<PoolKey> {
        let (asset0, asset1) = mint_token_pair(&self.provider).await;
        self.mock_reward()
            .configurePool(asset0, asset1, 60, pool_fee)
            .from(self.controller)
            .run_safe()
            .await?;

        self.pool_gate()
            .initializePool(asset0, asset1, *initial_price, 0)
            .from(self.controller)
            .run_safe()
            .await?;

        let pool_key = PoolKey {
            currency0:   asset0,
            currency1:   asset1,
            fee:         pool_fee,
            tickSpacing: tick_spacing,
            hooks:       self.mock_reward
        };

        Ok(pool_key)
    }

    pub async fn add_liquidity_position(
        &self,
        pool_key: &PoolKey,
        lower_tick: I24,
        upper_tick: I24,
        liquidity: U256
    ) -> eyre::Result<()> {
        self.pool_gate()
            .addLiquidity(
                pool_key.currency0,
                pool_key.currency1,
                lower_tick,
                upper_tick,
                liquidity,
                FixedBytes::default()
            )
            .run_safe()
            .await
    }
}
