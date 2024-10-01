use std::time::Duration;

use alloy::sol_types::SolValue;
use angstrom_types::sol_bindings::{sol::ContractBundle, testnet::TestnetHub};
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};
use testnet::{
    anvil_utils::spawn_anvil,
    cli::Cli,
    contract_setup::deploy_contract_and_create_pool,
    eth::RpcStateProviderFactory,
    strom_network::{
        controller::StromController,
        manager::{StromPeerManager, StromPeerManagerBuilder}
    }
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cli_args = Cli::parse_with_tracing();

    let mut network_controller = StromController::<NoopProvider>::new();

    for id in 0..=cli_args.nodes_in_network {
        network_controller
            .spawn_node(id, cli_args.starting_port, cli_args.testnet_block_time_secs)
            .await?;
    }

    network_controller.connect_all_peers().await;

    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        network_controller
            .run_event(None, |peer| peer.send_bundles(10))
            .await??;
    }

    Ok(())
}
