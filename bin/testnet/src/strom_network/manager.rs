use std::sync::Arc;

use alloy::{primitives::FixedBytes, providers::Provider, sol_types::SolValue};
use angstrom::cli::{initialize_strom_handles, StromHandles};
use angstrom_eth::handle::Eth;
use angstrom_network::{pool_manager::PoolHandle, NetworkOrderEvent, PoolManagerBuilder};
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use angstrom_types::sol_bindings::{
    grouped_orders::AllOrders,
    sol::ContractBundle,
    testnet::{random::RandomValues, TestnetHub}
};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};
use reth_tasks::TokioTaskExecutor;
use tracing::{span, Level};
use validation::init_validation;

use crate::{
    eth::{anvil_cleanser::AnvilEthDataCleanser, RpcStateProviderFactoryWrapper},
    strom_network::peers::TestnetPeer,
    types::SendingStromHandles,
    StromContractInstance, CACHE_VALIDATION_SIZE
};

pub struct TestnetPeerManager<C = NoopProvider> {
    pub id:               u64,
    pub port:             u64,
    pub peer:             TestnetPeer<C>,
    pub state_provider:   RpcStateProviderFactoryWrapper,
    pub order_storage:    Arc<OrderStorage>,
    pub pool_handle:      PoolHandle,
    pub tx_strom_handles: SendingStromHandles,
    pub testnet_hub:      StromContractInstance
}

impl<C> TestnetPeerManager<C>
where
    C: BlockReader + HeaderProvider + Unpin + Clone + 'static
{
    // pub async fn send_network_event_to_peer(
    //     &self,
    //     peer: &TestnetPeerManager<C>,
    //     event: NetworkOrderEvent
    // ) -> eyre::Result<()> {
    //     let id = peer.id;
    //     tracing::trace!("sending network event to peer {id} - {:?}", event);

    //     //self.peer.eth_peer;

    //     tracing::trace!("sent network event to peer {id} - {:?}", event);

    //     Ok(())
    // }

    pub fn send_bundles_to_network(&self, bundles: usize) -> eyre::Result<()> {
        let orders = AllOrders::gen_many(bundles);
        let num_orders = orders.len();
        tracing::debug!("submitting a angstrom bundle with {num_orders} orders to the network");

        self.tx_strom_handles
            .pool_tx
            .send(NetworkOrderEvent::IncomingOrders { peer_id: self.peer.peer_id(), orders })?;

        tracing::debug!("sent {num_orders} bundles to the network");

        Ok(())
    }

    pub async fn execute_bundles_locally(&self) -> eyre::Result<()> {
        let orders = ContractBundle::gen();
        let hashes = orders.get_filled_hashes();
        tracing::debug!("executing a angstrom bundle with hashes: {:#?}", hashes);

        let tx_hash = self
            .testnet_hub
            .execute(orders.abi_encode().into())
            .send()
            .await?
            .watch()
            .await?;

        tracing::debug!(?tx_hash, "tx hash with angstrom contract sent");

        Ok(())
    }
}
