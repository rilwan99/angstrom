use alloy::{node_bindings::AnvilInstance, primitives::Address};
use angstrom_types::contract_bindings::{poolgate::PoolGate, poolmanager::PoolManager};

use super::{
    anvil::{spawn_anvil, AnvilWalletRpc},
    deploy::mockreward::deploy_mock_rewards_manager
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
        let (anvil, provider) = spawn_anvil(1, 7).await?;
        let controller = anvil.addresses()[7];
        let pool_manager = *PoolManager::deploy(&provider).await.unwrap().address();
        let pool_gate = *PoolGate::deploy(&provider, pool_manager)
            .await
            .unwrap()
            .address();
        let mock_reward = deploy_mock_rewards_manager(&provider, pool_manager, controller).await;
        Ok(Self { anvil, provider, controller, pool_manager, pool_gate, mock_reward })
    }
}
