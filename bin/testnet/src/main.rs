use std::time::Duration;

use angstrom_network::{NetworkOrderEvent, StromMessage};
use reth_primitives::Hardforks;
use reth_provider::{test_utils::NoopProvider, BlockReader, ChainSpecProvider, HeaderProvider};
use testing_tools::testnet_controllers::StromTestnet;
use testnet::cli::Cli;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
    let config = Cli::build_config();

    let network_controller = StromTestnet::spawn_testnet(NoopProvider::default(), config).await?;

    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let peer_count = network_controller
        .run_event(Some(0), |peer| async {
            {
                let v = peer.network.strom_peer_network().peer_count();
                println!("0 - {v}");
                v
            }
        })
        .await;

    let _ = network_controller
        .run_event(Some(1), |peer| async {
            {
                let v = peer.network.strom_peer_network().peer_count();
                println!("1 - {v}");
            }
        })
        .await;

    assert_eq!(peer_count, 1);

    do_thing_other(network_controller).await?;

    Ok(())
}

async fn do_thing(network_controller: StromTestnet<NoopProvider>) -> eyre::Result<()> {
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

async fn do_thing_other(mut network_controller: StromTestnet<NoopProvider>) -> eyre::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        let orders = vec![];
        let passed = network_controller
            .broadcast_message_orders(
                Some(0),
                StromMessage::PropagatePooledOrders(orders.clone()),
                orders
            )
            .await;

        assert!(passed);

        // Ok(())
    }
}
