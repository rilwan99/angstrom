use std::time::Duration;

use alloy::sol_types::SolValue;
use angstrom_types::sol_bindings::{sol::ContractBundle, testnet::TestnetHub};
use reth_provider::test_utils::NoopProvider;
use testnet::{
    anvil_utils::spawn_anvil,
    cli::Cli,
    contract_setup::deploy_contract_and_create_pool,
    eth::RpcStateProviderFactory,
    strom_network::{controller::StromController, manager::StromPeerManager}
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli_args = Cli::parse_with_tracing();

    let (_anvil_handle, rpc) = spawn_anvil(cli_args.testnet_block_time_secs).await?;

    tracing::info!("deploying contracts to anvil");
    let addresses = deploy_contract_and_create_pool(rpc.clone()).await?;

    let rpc_wrapper = RpcStateProviderFactory::new(rpc.clone())?;
    let mut network_controller = StromController::new();
    let angstrom_addr = addresses.contract;

    for id in 0..=cli_args.nodes_in_network {
        let peer = StromPeerManager::new(
            id,
            cli_args.starting_port as u64,
            NoopProvider::default(),
            rpc_wrapper.clone()
        )
        .await;
        network_controller.add_peer(peer)
    }

    network_controller.connect_all_peers().await;
    network_controller
        .spawn_all_testnet_nodes(angstrom_addr)
        .await?;

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
