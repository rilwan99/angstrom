use std::sync::Arc;

use alloy::{
    primitives::FixedBytes, providers::Provider, pubsub::PubSubFrontend, sol_types::SolValue
};
use angstrom::cli::{initialize_strom_handles, StromHandles};
use angstrom_eth::{handle::Eth, manager::EthEvent};
use angstrom_network::{pool_manager::PoolHandle, PoolManagerBuilder};
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use angstrom_types::sol_bindings::{
    sol::ContractBundle,
    testnet::TestnetHub::{self, TestnetHubInstance}
};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader};
use reth_tasks::TokioTaskExecutor;
use tracing::{span, Instrument, Level, Span};
use validation::init_validation;

use super::handles::SendingStromHandles;
use crate::{
    eth::{anvil_cleanser::AnvilEthDataCleanser, RpcStateProviderFactory},
    strom_network::peers::StromPeer,
    StromContractInstance, CACHE_VALIDATION_SIZE
};

pub struct StromPeerManagerBuilder<C = NoopProvider> {
    id:            u64,
    port:          u64,
    public_key:    FixedBytes<64>,
    peer:          StromPeer<C>,
    rpc_wrapper:   RpcStateProviderFactory,
    strom_handles: Option<StromHandles>,
    span:          Span
}

impl<C> StromPeerManagerBuilder<C>
where
    C: BlockReader + Clone + Unpin + 'static
{
    pub async fn new(
        id: u64,
        port: u64,
        provider: C,
        rpc_wrapper: RpcStateProviderFactory
    ) -> Self {
        let span = span!(Level::TRACE, "testnet node", id = id);
        let handles = initialize_strom_handles();
        let peer = StromPeer::new_fully_configed(
            provider,
            Some(handles.pool_tx.clone()),
            Some(handles.consensus_tx_op.clone())
        )
        .instrument(span.clone())
        .await;
        let pk = peer.get_node_public_key();

        Self {
            id,
            peer,
            public_key: pk,
            rpc_wrapper,
            strom_handles: Some(handles),
            port: port + id,
            span
        }
    }

    pub async fn build_and_spawn(
        mut self,
        contract_address: Address
    ) -> eyre::Result<StromPeerManager<C>> {
        let span = span!(Level::ERROR, "testnet node", id = self.id);
        let strom_handles = self.strom_handles.take().unwrap();
        let pool = strom_handles.get_pool_handle();
        let executor: TokioTaskExecutor = Default::default();
        let tx_strom_handles = (&strom_handles).into();

        let rpc_w = self.rpc_wrapper.clone();
        let state_stream = self
            .rpc_wrapper
            .provider
            .clone()
            .subscribe_blocks()
            .await?
            .into_stream()
            .map(move |block| {
                let cloned_block = block.clone();
                let rpc = rpc_w.clone();
                async move {
                    let number = cloned_block.header.number;
                    let mut res = vec![];
                    for hash in cloned_block.transactions.hashes() {
                        let Ok(Some(tx)) = rpc.provider.get_transaction_by_hash(hash).await else {
                            continue
                        };
                        res.push(tx);
                    }
                    (number, res)
                }
            })
            .buffer_unordered(10);

        let order_api = OrderApi::new(pool.clone(), executor.clone());

        let eth_handle = AnvilEthDataCleanser::spawn(
            executor.clone(),
            contract_address,
            strom_handles.eth_tx,
            strom_handles.eth_rx,
            state_stream,
            7,
            span
        )
        .await?;

        let validator = init_validation(self.rpc_wrapper.clone(), CACHE_VALIDATION_SIZE);

        let network_handle = self.peer.handle.clone();

        let pool_config = PoolConfig::default();
        let order_storage = Arc::new(OrderStorage::new(&pool_config));

        let pool_handle = PoolManagerBuilder::new(
            validator.clone(),
            Some(order_storage.clone()),
            network_handle.clone(),
            eth_handle.subscribe_network(),
            strom_handles.pool_rx
        )
        .with_config(pool_config)
        .build_with_channels(
            executor,
            strom_handles.orderpool_tx,
            strom_handles.orderpool_rx,
            strom_handles.pool_manager_tx
        );
        let port = self.port;
        let server = ServerBuilder::default()
            .build(format!("127.0.0.1:{}", port))
            .await?;

        let addr = server.local_addr().unwrap();
        tracing::info!("rpc server started on: {}", addr);

        tokio::spawn(async move {
            let server_handle = server.start(order_api.into_rpc());
            let _ = server_handle.stopped().await;
        });

        let testnet_hub = TestnetHub::new(contract_address, self.rpc_wrapper.provider.clone());

        Ok(StromPeerManager {
            id: self.id,
            port: self.port,
            public_key: self.public_key,
            peer: self.peer,
            rpc_wrapper: self.rpc_wrapper.clone(),
            order_storage,
            pool_handle,
            testnet_hub,
            tx_strom_handles,
            span: self.span
        })
    }
}

pub struct StromPeerManager<C = NoopProvider> {
    pub id:               u64,
    pub port:             u64,
    pub public_key:       FixedBytes<64>,
    pub peer:             StromPeer<C>,
    pub rpc_wrapper:      RpcStateProviderFactory,
    pub order_storage:    Arc<OrderStorage>,
    pub pool_handle:      PoolHandle,
    pub tx_strom_handles: SendingStromHandles,
    pub testnet_hub:      StromContractInstance,
    pub span:             Span
}

impl<C> StromPeerManager<C> {
    pub async fn send_bundles(&self, bundles: u64) -> eyre::Result<()> {
        let orders = ContractBundle::generate_random_bundles(bundles);
        let hashes = orders.get_filled_hashes();
        tracing::info!("submitting a angstrom bundle with hashes: {:#?}", hashes);
        let tx_hash = self
            .testnet_hub
            .execute(orders.abi_encode().into())
            .send()
            .await?
            .watch()
            .await?;

        tracing::info!(?tx_hash, "tx hash with angstrom contract sent");

        Ok(())

        // Ok(HookEvents::EthEvent(EthEvent::NewBlockTransitions {
        //     block_number:      orders.,
        //     filled_orders:     hashes,
        //     address_changeset: ()
        // }))
    }
}
