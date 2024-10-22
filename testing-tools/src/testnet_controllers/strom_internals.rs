use std::sync::Arc;

use alloy::providers::Provider;
use angstrom::cli::StromHandles;
use angstrom_eth::handle::Eth;
use angstrom_network::{pool_manager::PoolHandle, PoolManagerBuilder, StromNetworkHandle};
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use angstrom_types::sol_bindings::testnet::TestnetHub;
use consensus::{AngstromValidator, ConsensusManager, ManagerNetworkDeps, Signer};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_tasks::TokioTaskExecutor;
use secp256k1::SecretKey;

use super::config::AngstromTestnetConfig;
use crate::{
    anvil_state_provider::{
        utils::StromContractInstance, AnvilEthDataCleanser, RpcStateProviderFactory,
        RpcStateProviderFactoryWrapper
    },
    contracts::deploy_contract_and_create_pool,
    types::SendingStromHandles,
    validation::TestOrderValidator
};

pub struct AngstromTestnetNodeInternals {
    pub rpc_port:         u64,
    pub state_provider:   RpcStateProviderFactoryWrapper,
    pub order_storage:    Arc<OrderStorage>,
    pub pool_handle:      PoolHandle,
    pub tx_strom_handles: SendingStromHandles,
    pub testnet_hub:      StromContractInstance,
    pub validator:        TestOrderValidator<RpcStateProviderFactory> //pub consensus:
}

impl AngstromTestnetNodeInternals {
    pub async fn new(
        testnet_node_id: u64,
        strom_handles: StromHandles,
        strom_network_handle: StromNetworkHandle,
        secret_key: SecretKey,
        config: AngstromTestnetConfig,
        initial_validators: Vec<AngstromValidator>
    ) -> eyre::Result<Self> {
        tracing::debug!("connecting to state provider");
        let state_provider = RpcStateProviderFactoryWrapper::spawn_new(
            config.testnet_block_time_secs,
            testnet_node_id
        )
        .await?;
        tracing::info!("connected to state provider");

        tracing::debug!("deploying contracts to anvil");
        let addresses =
            deploy_contract_and_create_pool(state_provider.provider().provider()).await?;
        tracing::info!("deployed contracts to anvil");

        let angstrom_addr = addresses.contract;

        let pool = strom_handles.get_pool_handle();
        let executor: TokioTaskExecutor = Default::default();
        let tx_strom_handles = (&strom_handles).into();

        let rpc_w = state_provider.provider();
        let state_stream = state_provider
            .provider()
            .provider()
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
                        let Ok(Some(tx)) = rpc.provider().get_transaction_by_hash(hash).await
                        else {
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
            testnet_node_id,
            executor.clone(),
            angstrom_addr,
            strom_handles.eth_tx,
            strom_handles.eth_rx,
            state_stream,
            7
        )
        .await?;

        // let (_, rx) = tokio::sync::broadcast::channel(1000);
        //let validator = init_validation(state_provider.provider(), rx,
        // CACHE_VALIDATION_SIZE);

        let validator = TestOrderValidator::new(state_provider.provider());

        let pool_config = PoolConfig::default();
        let order_storage = Arc::new(OrderStorage::new(&pool_config));

        let pool_handle = PoolManagerBuilder::new(
            validator.client.clone(),
            Some(order_storage.clone()),
            strom_network_handle.clone(),
            eth_handle.subscribe_network(),
            strom_handles.pool_rx
        )
        .with_config(pool_config)
        .build_with_channels(
            executor.clone(),
            strom_handles.orderpool_tx,
            strom_handles.orderpool_rx,
            strom_handles.pool_manager_tx
        );

        let rpc_port = config.rpc_port_with_node_id(testnet_node_id);
        let server = ServerBuilder::default()
            .build(format!("127.0.0.1:{}", rpc_port))
            .await?;

        let addr = server.local_addr().unwrap();

        tokio::spawn(async move {
            let server_handle = server.start(order_api.into_rpc());
            tracing::info!("rpc server started on: {}", addr);
            let _ = server_handle.stopped().await;
        });

        let testnet_hub = TestnetHub::new(angstrom_addr, state_provider.provider().provider());

        let _consensus_handle = ConsensusManager::spawn(
            executor,
            ManagerNetworkDeps::new(
                strom_network_handle.clone(),
                state_provider.provider().subscribe_to_canonical_state(),
                strom_handles.consensus_rx_op
            ),
            Signer::new(secret_key),
            initial_validators,
            order_storage.clone(),
            state_provider
                .provider()
                .provider()
                .get_block_number()
                .await?
        );

        Ok(Self {
            rpc_port,
            state_provider,
            order_storage,
            pool_handle,
            tx_strom_handles,
            testnet_hub,
            validator
        })
    }
}
