use std::time::Duration;

use testing_tools::testnet_controllers::StromTestnet;
use testnet::cli::Cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
    let config = Cli::build_config();

    let network_controller = StromTestnet::spawn_testnet(config).await?;

    let peer_count = network_controller
        .run_event(Some(1), |peer| async {
            {
                peer.network.strom_peer_network().peer_count()
            }
        })
        .await;

    assert_ne!(peer_count, 1);

    do_thing(network_controller).await?;

    Ok(())
}

async fn do_thing(network_controller: StromTestnet) -> eyre::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        network_controller
            .run_event(None, |peer| async {
                peer.send_bundles_to_network(peer.network.pubkey(), 10)
            })
            .await?;
        // Ok(())
    }
}
