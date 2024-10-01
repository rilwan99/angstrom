use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    providers::{
        builder,
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, RootProvider
    },
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner
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

#[cfg(feature = "ipc")]
pub async fn spawn_anvil(block_time: u64) -> eyre::Result<(AnvilInstance, AnvilWalletRpc)> {
    let anvil = Anvil::new()
        .block_time(block_time)
        .chain_id(1)
        .arg("--ipc")
        .arg("--code-size-limit")
        .arg("393216")
        .arg("--disable-block-gas-limit")
        .try_spawn()?;

    let endpoint = "/tmp/anvil.ipc";
    tracing::info!(?endpoint);
    let ipc = alloy::providers::IpcConnect::new(endpoint.to_string());
    let sk: PrivateKeySigner = anvil.keys()[0].clone().into();

    let wallet = EthereumWallet::new(sk);
    let rpc = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ipc(ipc)
        .await?;

    tracing::info!("connected to anvil");

    Ok((anvil, rpc))
}

#[cfg(feature = "ws")]
pub async fn spawn_anvil(block_time: u64) -> eyre::Result<(AnvilInstance, AnvilWalletRpc)> {
    let anvil = Anvil::new()
        .block_time(block_time)
        .chain_id(1)
        // .arg("--ws")
        .arg("--disable-code-size-limit")
        // .arg("--code-size-limit")
        // .arg("3932160")
        .arg("--disable-block-gas-limit")
        .try_spawn()?;

    let endpoint = "ws://35.245.117.24:8546";
    tracing::info!(?endpoint);
    let ws = alloy::providers::WsConnect::new(endpoint.to_string());
    let sk: PrivateKeySigner = anvil.keys()[0].clone().into();

    let wallet = EthereumWallet::new(sk);
    let rpc = builder::<Ethereum>()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_ws(ws)
        .await?;

    tracing::info!("connected to anvil");

    Ok((anvil, rpc))
}
