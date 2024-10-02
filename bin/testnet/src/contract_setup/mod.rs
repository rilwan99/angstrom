use std::{pin::pin, sync::OnceLock, time::Duration};

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

static CREATE_SIGNER: OnceLock<Address> = OnceLock::new();

fn get_or_set_signer(my_address: Address) -> Address {
    *CREATE_SIGNER.get_or_init(|| my_address)
}

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
    let signer_set = CREATE_SIGNER.get().is_some();

    if signer_set {
        provider
            .anvil_impersonate_account(get_or_set_signer(provider.default_signer_address()))
            .await?;
    }

    let out = anvil_mine_delay(
        Box::pin(async {
            PoolManagerDeployer::deploy(provider.clone(), U256::MAX)
                .await
                .map(|v| v.at(address!("998abeb3e57409262ae5b751f60747921b33613e")))
        }),
        &provider,
        Duration::from_millis(500)
    )
    .await?;

    let v4_address = *out.address();

    let testhub = anvil_mine_delay(
        Box::pin(async {
            TestnetHub::deploy(provider.clone(), Address::ZERO, v4_address)
                .await
                .map(|v| v.at(address!("7969c5ed335650692bc04293b07f5bf2e7a673c0")))
        }),
        &provider,
        Duration::from_millis(500)
    )
    .await?;
    let angstrom_address = *testhub.address();

    // if we don't do these sequentially, the provider nonce messes up and doesn't
    // deploy properly
    let token0 = anvil_mine_delay(
        Box::pin(async {
            MockERC20::deploy(provider.clone())
                .await
                .map(|v| v.at(address!("4ee6ecad1c2dae9f525404de8555724e3c35d07b")))
        }),
        &provider,
        Duration::from_millis(500)
    )
    .await?;
    let token0 = *token0.address();

    let token1 = anvil_mine_delay(
        Box::pin(async {
            MockERC20::deploy(provider.clone())
                .await
                .map(|v| v.at(address!("fbc22278a96299d91d41c453234d97b4f5eb9b2d")))
        }),
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

    if signer_set {
        provider
            .anvil_stop_impersonating_account(get_or_set_signer(provider.default_signer_address()))
            .await?;
    } else {
        get_or_set_signer(provider.default_signer_address());
    };

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
