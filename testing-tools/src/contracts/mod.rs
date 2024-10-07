use std::{pin::pin, sync::OnceLock, time::Duration};

use alloy::{
    contract::RawCallBuilder,
    primitives::{address, keccak256, Address, Bytes, B256, U160, U256},
    providers::{ext::AnvilApi, WalletProvider},
    sol_types::SolValue
};
use angstrom_types::{
    contract_bindings::mockrewardsmanager::MockRewardsManager,
    sol_bindings::testnet::{MockERC20, PoolManagerDeployer, TestnetHub}
};
use futures::Future;
use tokio::time::timeout;

use crate::anvil_state_provider::utils::AnvilWalletRpc;

const CREATE2_FACTORY: Address = address!("4e59b44847b379578588920cA78FbF26c0B4956C");

/// Attempt to find a target address that includes the appropriate flags
/// Returns the address found and the salt needed to pad the initcode to
/// deploy to that address
pub fn mine_address(flags: U160, mask: U160, initcode: &Bytes) -> (Address, U256) {
    mine_address_with_factory(None, flags, mask, initcode)
}

pub fn mine_address_with_factory(
    factory: Option<Address>,
    flags: U160,
    mask: U160,
    initcode: &Bytes
) -> (Address, U256) {
    let init_code_hash = keccak256(initcode);
    let mut salt = U256::ZERO;
    let create2_factory = factory.unwrap_or(CREATE2_FACTORY);
    let mut counter: u128 = 0;
    loop {
        let target_address: Address = create2_factory.create2(B256::from(salt), init_code_hash);
        let u_address: U160 = target_address.into();
        if (u_address & mask) == flags {
            break;
        }
        salt += U256::from(1_u8);
        counter += 1;
        if counter > 100_000 {
            panic!("We tried this too many times!")
        }
    }
    let final_address = create2_factory.create2(B256::from(salt), init_code_hash);
    (final_address, salt)
}

pub async fn deploy_mock_rewards_manager<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network
>(
    provider: &P,
    pool_manager: Address
) -> Address
where {
    // Setup our flags and mask
    // Flags for our MockRewardsManager address
    let before_swap = U160::from(1_u8) << 7;
    let before_initialize = U160::from(1_u8) << 13;
    let before_add_liquidity = U160::from(1_u8) << 11;
    let after_remove_liquidity = U160::from(1_u8) << 8;

    let flags = before_swap | before_initialize | before_add_liquidity | after_remove_liquidity;
    let mask: U160 = (U160::from(1_u8) << 14) - U160::from(1_u8);

    let mock_builder = MockRewardsManager::deploy_builder(&provider, pool_manager);
    let (mock_tob, salt) = mine_address(flags, mask, mock_builder.calldata());
    let final_mock_initcode = [salt.abi_encode(), mock_builder.calldata().to_vec()].concat();
    let raw_deploy = RawCallBuilder::new_raw_deploy(&provider, final_mock_initcode.into());
    raw_deploy.call_raw().await.unwrap();
    mock_tob
}

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
    provider
        .anvil_impersonate_account(get_or_set_signer(provider.default_signer_address()))
        .await?;

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
    provider
        .anvil_stop_impersonating_account(get_or_set_signer(provider.default_signer_address()))
        .await?;

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
