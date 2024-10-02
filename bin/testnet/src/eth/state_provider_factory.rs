use std::future::IntoFuture;

use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::keccak256,
    providers::{
        builder,
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, Provider, RootProvider
    },
    pubsub::PubSubFrontend,
    signers::local::PrivateKeySigner,
    transports::TransportResult
};
use futures::Future;
use reth_primitives::{Account, Address, BlockNumber, StorageKey, StorageValue};
use reth_provider::{ProviderError, ProviderResult};
use validation::common::lru_db::{BlockStateProvider, BlockStateProviderFactory};

use super::RpcStateProvider;
use crate::{async_to_sync, AnvilWalletRpc};

#[derive(Clone, Debug)]
pub struct RpcStateProviderFactory {
    pub provider: AnvilWalletRpc
}

impl RpcStateProviderFactory {
    pub async fn spawn_new(block_time: u64, id: u64) -> eyre::Result<Self> {
        let anvil = Anvil::new()
            .block_time(block_time)
            .chain_id(1)
            .arg("--ipc")
            .arg(format!("/tmp/anvil_{id}.ipc"))
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

        Ok(Self { provider: rpc })
    }
}

impl BlockStateProviderFactory for RpcStateProviderFactory {
    type Provider = RpcStateProvider;

    fn state_by_block(&self, block: u64) -> ProviderResult<Self::Provider> {
        Ok(RpcStateProvider::new(block, self.provider.clone()))
    }

    fn best_block_number(&self) -> ProviderResult<BlockNumber> {
        async_to_sync(self.provider.get_block_number())
            .map_err(|_| ProviderError::BestBlockNotFound)
    }
}
