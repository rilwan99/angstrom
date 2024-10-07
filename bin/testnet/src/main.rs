use std::time::Duration;

use reth_provider::test_utils::NoopProvider;
use testnet::{cli::Cli, strom_network::controller::StromController};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
    let cli_args = Cli::parse_with_tracing();

    let mut network_controller = StromController::<NoopProvider>::new();

    for id in 0..cli_args.nodes_in_network {
        network_controller
            .spawn_node(id, cli_args.starting_port, cli_args.testnet_block_time_secs)
            .await?;
    }

    network_controller.connect_all_peers().await;

    let is_connected = network_controller
        .run_event(Some(0), |peer| async {
            {
                println!("1 - {:?}", peer.peer.local_addr());
                let peer_b = network_controller.get_peer(1);
                println!("2 - {:?}", peer_b.peer.local_addr());
                peer.peer
                    .strom_network
                    .swarm()
                    .state()
                    .is_active_peer(peer_b.peer.peer_id)
            }
        })
        .await?;

    assert!(is_connected);

    do_thing(network_controller).await?;

    Ok(())
}

async fn do_thing(network_controller: StromController) -> eyre::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        network_controller
            .run_event(None, |peer| async { peer.send_bundles_to_network(10) })
            .await??;
    }
}
