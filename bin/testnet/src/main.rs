use reth_provider::test_utils::NoopProvider;
use testing_tools::{
    testnet_controllers::AngstromTestnet, types::checked_actions::WithCheckedAction
};
use testnet::cli::Cli;
use tracing::{debug, info};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
    let config = Cli::build_config();

    let mut testnet = AngstromTestnet::spawn_testnet(NoopProvider::default(), config)
        .await?
        .as_state_machine();

    info!("deployed state machine");

    testnet.send_pooled_orders(vec![]);
    debug!("added pooled orders to state machine");
    // testnet.send_prepropose(vec![]);

    testnet.run().await;

    Ok(())
}
<<<<<<< HEAD
=======

#[allow(dead_code)]
async fn do_thing(network_controller: AngstromTestnet<NoopProvider>) -> eyre::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        network_controller
            .run_event(None, |peer| async { peer.send_bundles_to_network(peer.peer_id(), 10) })
            .await?;
        // Ok(())
    }
}

async fn send_bundles(mut network_controller: AngstromTestnet<NoopProvider>) -> eyre::Result<()> {
    loop {
        tokio::time::sleep(Duration::from_secs(11)).await;
        let orders = vec![];
        let passed = network_controller
            .broadcast_orders_message(
                Some(0),
                StromMessage::PropagatePooledOrders(orders.clone()),
                orders
            )
            .await;

        assert!(passed);

        // Ok(())
    }
}
>>>>>>> main
