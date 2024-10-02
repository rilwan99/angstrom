use std::{pin::pin, time::Duration};

use alloy::{
    primitives::{Address, U256},
    providers::{ext::AnvilApi, WalletProvider}
};
use angstrom_types::sol_bindings::testnet::{MockERC20, PoolManagerDeployer, TestnetHub};
use futures::Future;
use reth_primitives::address;
use tokio::time::timeout;

use crate::AnvilWalletRpc;

pub mod contract_bytecodes;

pub struct AngstromTestnetAddresses {
    pub contract: Address,
    pub token0:   Address,
    pub token1:   Address
}
/// deploys the angstrom testhub contract along with two tokens, under the
/// secret key
pub async fn deploy_contract_and_create_pool(
    provider: AnvilWalletRpc
) -> eyre::Result<AngstromTestnetAddresses> {
    println!(
        "A: {:?} -- B: {:?}",
        provider.default_signer_address(),
        PoolManagerDeployer::deploy_builder(provider.clone(), U256::MAX).calculate_create_address(),
    );
    let out = anvil_mine_delay(
        Box::pin(async {
            Ok::<_, eyre::ErrReport>(
                PoolManagerDeployer::deploy(provider.clone(), U256::MAX)
                    .await
                    .unwrap()
                    .at(address!("998abeb3e57409262ae5b751f60747921b33613e"))
            )
        }),
        &provider,
        Duration::from_millis(500)
    )
    .await?;

    let v4_address = *out.address();

    let testhub = anvil_mine_delay(
        Box::pin(TestnetHub::deploy(provider.clone(), Address::ZERO, v4_address)),
        &provider,
        Duration::from_millis(500)
    )
    .await?;
    let angstrom_address = *testhub.address();

    // if we don't do these sequentially, the provider nonce messes up and doesn't
    // deploy properly
    let token0 = anvil_mine_delay(
        Box::pin(MockERC20::deploy(provider.clone())),
        &provider,
        Duration::from_millis(500)
    )
    .await?;
    let token0 = *token0.address();

    let token1 = anvil_mine_delay(
        Box::pin(MockERC20::deploy(provider.clone())),
        &provider,
        Duration::from_millis(500)
    )
    .await?;
    let token1 = *token1.address();

    tracing::info!(
        ?angstrom_address,
        ?v4_address,
        ?token0,
        ?token1,
        "deployed v4 and angstrom test contract on anvil"
    );

    Ok(AngstromTestnetAddresses { contract: angstrom_address, token0, token1 })
}

// will wait for a specific delay and then call anvil mine wallet.
// this will allow us to quick mine and not have to wait the 12 seconds
// between transactions while avoiding race conditions
pub async fn anvil_mine_delay<F0: Future + Unpin>(
    f0: F0,
    provider: &AnvilWalletRpc,
    delay: Duration
) -> F0::Output {
    let mut pinned = pin!(f0);
    if let Ok(v) = timeout(delay, &mut pinned).await {
        return v
    }
    provider
        .anvil_mine(Some(U256::from(1)), None)
        .await
        .expect("anvil failed to mine");

    pinned.await
}
