use std::sync::{atomic::AtomicBool, Arc};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    providers::Provider,
    pubsub::PubSubFrontend
};
use alloy_primitives::aliases::{I24, U24};
use angstrom::cli::StromHandles;
use angstrom_eth::handle::Eth;
use angstrom_network::{pool_manager::PoolHandle, PoolManagerBuilder, StromNetworkHandle};
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use angstrom_types::{
    contract_payloads::angstrom::{AngstromPoolConfigStore, UniswapAngstromRegistry},
    matching::{
        uniswap::{LiqRange, PoolSnapshot},
        SqrtPriceX96
    },
    primitive::PoolKey,
    sol_bindings::testnet::TestnetHub
};
use consensus::{AngstromValidator, ConsensusManager, ManagerNetworkDeps, Signer};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_provider::CanonStateSubscriptions;
use reth_tasks::TokioTaskExecutor;
use secp256k1::SecretKey;
use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

use crate::{
    anvil_state_provider::{
        utils::StromContractInstance, AnvilEthDataCleanser, AnvilStateProvider,
        AnvilStateProviderWrapper
    },
    contracts::environment::{
        angstrom::AngstromEnv, mockreward::MockRewardEnv, uniswap::UniswapEnv
    },
    network::TestnetConsensusFuture,
    testnet_controllers::AngstromTestnetConfig,
    types::SendingStromHandles,
    validation::TestOrderValidator
};

pub struct AngstromTestnetNodeInternals {
    pub rpc_port:         u64,
    pub state_provider:   AnvilStateProviderWrapper,
    pub order_storage:    Arc<OrderStorage>,
    pub pool_handle:      PoolHandle,
    pub tx_strom_handles: SendingStromHandles,
    pub testnet_hub:      StromContractInstance,
    pub validator:        TestOrderValidator<AnvilStateProvider>,
    consensus:            TestnetConsensusFuture<PubSubFrontend>,
    consensus_running:    Arc<AtomicBool>
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
        let state_provider = AnvilStateProviderWrapper::spawn_new(config, testnet_node_id).await?;
        tracing::info!("connected to state provider");

        tracing::debug!("deploying contracts to anvil");
        let uni_env = UniswapEnv::with_anvil(state_provider.provider()).await?;
        let angstrom_env = AngstromEnv::new(uni_env).await?;
        // let rewards_env =
        // MockRewardEnv::with_anvil(state_provider.provider()).await?;

        // let sqrt_price_x96 =
        // SqrtPriceX96::from(get_sqrt_ratio_at_tick(100020).unwrap());
        // let tick_spacing = I24::unchecked_from(60);
        // let pool_fee = U24::ZERO;
        // let snapshot = PoolSnapshot::new(
        //     vec![LiqRange::new(99900, 100140,
        // 5_000_000_000_000_000_000_000_u128).unwrap()],     sqrt_price_x96
        // )?;
        // let pool_key = rewards_env
        //     .create_pool_and_tokens_from_snapshot(tick_spacing, pool_fee, snapshot)
        //     .await?;

        tracing::info!("deployed contracts to anvil");

        let angstrom_addr = angstrom_env.angstrom();
        // let pools = vec![pool_key];
        let pools = vec![];
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
        let block_id = state_provider
            .provider()
            .provider()
            .get_block_number()
            .await
            .unwrap();
        // let pool_config_store = AngstromPoolConfigStore::load_from_chain(
        //     angstrom_addr,
        //     BlockId::Number(BlockNumberOrTag::Number(block_id)),
        //     &state_provider.provider().provider()
        // )
        // .await
        // .unwrap();
        let pool_config_store = AngstromPoolConfigStore::default();
        let pool_registry = UniswapAngstromRegistry::new(pools.into(), pool_config_store);

        let consensus_handle = ConsensusManager::new(
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
                .await?,
            pool_registry,
            state_provider.provider().provider()
        );

        let consensus_running = Arc::new(AtomicBool::new(false));

        let consensus = TestnetConsensusFuture::new(
            testnet_node_id,
            consensus_handle,
            consensus_running.clone()
        );

        Ok(Self {
            rpc_port,
            state_provider,
            order_storage,
            pool_handle,
            tx_strom_handles,
            testnet_hub,
            validator,
            consensus,
            consensus_running
        })
    }
}
