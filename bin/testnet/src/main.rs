use std::time::Duration;

use angstrom_network::StromMessage;
use reth_provider::test_utils::NoopProvider;
use testing_tools::{
    testnet_controllers::AngstromTestnet, types::checked_actions::WithCheckedAction
};
use testnet::cli::Cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
    let config = Cli::build_config();

    let mut testnet = AngstromTestnet::spawn_testnet(NoopProvider::default(), config)
        .await?
        .as_state_machine();

    testnet.send_pooled_orders(vec![]);
    // testnet.send_prepropose(vec![]);

    testnet.run().await;

    Ok(())
}
