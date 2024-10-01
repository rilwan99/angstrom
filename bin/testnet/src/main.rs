use std::{sync::Arc, time::Duration};

use alloy::{primitives::Address, providers::Provider, sol_types::SolValue};
use angstrom::cli::{initialize_strom_handles, StromHandles};
use angstrom_eth::handle::Eth;
use angstrom_network::pool_manager::PoolManagerBuilder;
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use angstrom_types::sol_bindings::{sol::ContractBundle, testnet::TestnetHub};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_provider::test_utils::NoopProvider;
use reth_tasks::TokioTaskExecutor;
use testnet::{
    anvil_utils::{spawn_anvil, AnvilEthDataCleanser},
    cli::Cli,
    contract_setup::deploy_contract_and_create_pool,
    eth::RpcStateProviderFactory,
    strom_network::{connect_all_peers, peers::StromPeer}
};
use tracing::{span, Instrument, Level};
use validation::init_validation;

const CACHE_VALIDATION_SIZE: usize = 100_000_000;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli_args = Cli::parse_with_tracing();

    let (_anvil_handle, rpc) = spawn_anvil(cli_args.testnet_block_time_secs).await?;

    tracing::info!("deploying contracts to anvil");
    let addresses = deploy_contract_and_create_pool(rpc.clone()).await?;

    let rpc_wrapper = RpcStateProviderFactory::new(rpc.clone())?;
    let mut network_with_handles = vec![];
    let angstrom_addr = addresses.contract;

    for id in 0..=cli_args.nodes_in_network {
        let span = span!(Level::TRACE, "testnet node", id = id);
        let handles = initialize_strom_handles();
        let peer = StromPeer::new_fully_configed(
            NoopProvider::default(),
            Some(handles.pool_tx.clone()),
            Some(handles.consensus_tx_op.clone())
        )
        .instrument(span)
        .await;
        let pk = peer.get_node_public_key();
        network_with_handles.push((pk, peer, handles));
    }
    connect_all_peers(&mut network_with_handles).await;

    for id in 0..cli_args.nodes_in_network {
        let (_, peer, handles) = network_with_handles.pop().expect("unreachable");
        spawn_testnet_node(rpc_wrapper.clone(), peer, handles, None, angstrom_addr, id).await?;
    }

    let (_, peer, handles) = network_with_handles.pop().expect("unreachable");
    // spawn the node with rpc
    tokio::spawn(async move {
        spawn_testnet_node(
            rpc_wrapper.clone(),
            peer,
            handles,
            Some(cli_args.starting_port),
            angstrom_addr,
            cli_args.nodes_in_network
        )
        .await
        .unwrap();
    });

    let testnet = TestnetHub::new(addresses.contract, rpc.clone());

    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        let orders = ContractBundle::generate_random_bundles(10);
        let hashes = orders.get_filled_hashes();
        tracing::info!("submitting a angstrom bundle with hashes: {:#?}", hashes);
        let tx_hash = testnet
            .execute(orders.abi_encode().into())
            .send()
            .await?
            .watch()
            .await?;

        tracing::info!(?tx_hash, "tx hash with angstrom contract sent");
    }
}

pub async fn spawn_testnet_node(
    rpc_wrapper: RpcStateProviderFactory,
    network: StromPeer<NoopProvider>,
    handles: StromHandles,
    port: Option<u16>,
    contract_address: Address,
    id: u64
) -> eyre::Result<()> {
    let span = span!(Level::ERROR, "testnet node", id = id);
    let pool = handles.get_pool_handle();
    let executor: TokioTaskExecutor = Default::default();

    let rpc_w = rpc_wrapper.clone();
    let state_stream = rpc_wrapper
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
        handles.eth_tx,
        handles.eth_rx,
        state_stream,
        7,
        span
    )
    .await?;

    let validator = init_validation(rpc_wrapper, CACHE_VALIDATION_SIZE);

    let network_handle = network.handle.clone();

    let pool_config = PoolConfig::default();
    let order_storage = Arc::new(OrderStorage::new(&pool_config));

    let _pool_handle = PoolManagerBuilder::new(
        validator.clone(),
        Some(order_storage.clone()),
        network_handle.clone(),
        eth_handle.subscribe_network(),
        handles.pool_rx
    )
    .with_config(pool_config)
    .build_with_channels(
        executor,
        handles.orderpool_tx,
        handles.orderpool_rx,
        handles.pool_manager_tx
    );
    if let Some(port) = port {
        let server = ServerBuilder::default()
            .build(format!("127.0.0.1:{}", port))
            .await?;

        let addr = server.local_addr().unwrap();
        println!("rpc server started on: {}", addr);

        let server_handle = server.start(order_api.into_rpc());
        let _ = server_handle.stopped().await;
    }

    Ok(())
}
