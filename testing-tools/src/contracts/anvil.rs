use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::Anvil,
    providers::{
        builder,
        ext::AnvilApi,
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, RootProvider
    },
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner
};
use alloy_primitives::{Address, Bytes};
use tokio::sync::broadcast;

use crate::{
    anvil_state_provider::AnvilStateProvider,
    contracts::environment::{angstrom::AngstromEnv, uniswap::UniswapEnv},
    mocks::canon_state::AnvilConsensusCanonStateNotification,
    testnet_controllers::AngstromTestnetConfig
};

pub type AnvilWalletRpc = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<
                GasFiller,
                JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    JoinFill<NonceFiller, ChainIdFiller>
                >
            >
        >,
        WalletFiller<EthereumWallet>
    >,
    RootProvider<PubSubFrontend>,
    PubSubFrontend,
    Ethereum
>;

pub(crate) async fn angstrom_address_with_state(
    config: AngstromTestnetConfig
) -> eyre::Result<(Address, Bytes)> {
    let mut anvil_builder = Anvil::new()
        .block_time(2)
        .chain_id(1)
        .arg("--ipc")
        .arg(format!("/tmp/anvil_temp_deploy.ipc"))
        .arg("--code-size-limit")
        .arg("393216")
        .arg("--disable-block-gas-limit");

    if let Some(fork_block) = config.fork_block_number() {
        anvil_builder = anvil_builder
            .arg("--fork-block-number")
            .arg(format!("{}", fork_block));
    }

    let anvil = anvil_builder.try_spawn()?;

    let endpoint = format!("/tmp/anvil_temp_deploy.ipc");
    tracing::info!(?endpoint);
    let ipc = alloy::providers::IpcConnect::new(endpoint.to_string());
    let sk: PrivateKeySigner = anvil.keys()[7].clone().into();

    let wallet = EthereumWallet::new(sk);
    let rpc = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ipc(ipc)
        .await?;

    let (tx, _) = broadcast::channel(100);
    let provider = AnvilStateProvider {
        provider:       rpc,
        controller:     anvil.addresses()[7],
        canon_state_tx: tx,
        canon_state:    AnvilConsensusCanonStateNotification::new()
    };

    let uni_env = UniswapEnv::with_anvil(provider.clone()).await?;
    let angstrom_env = AngstromEnv::new(uni_env).await?;

    tracing::debug!("deploying contracts to anvil");

    let state = provider.provider().anvil_dump_state().await?;

    Ok((angstrom_env.angstrom(), state))
}
