use std::collections::HashMap;

use alloy_sol_types::SolValue;
use angstrom::cli::StromHandles;
use angstrom_network::NetworkOrderEvent;
use angstrom_types::{
    primitive::PeerId,
    sol_bindings::{grouped_orders::AllOrders, sol::ContractBundle, testnet::random::RandomValues}
};

use super::{config::StromTestnetConfig, strom_internals::StromTestnetNodeInternals};
use crate::network::peers::TestnetNodeNetwork;

pub struct TestnetNode {
    pub testnet_node_id: u64,
    pub network:         TestnetNodeNetwork,
    pub strom:           StromTestnetNodeInternals
}

impl TestnetNode {
    pub async fn new(
        testnet_node_id: u64,
        network: TestnetNodeNetwork,
        strom_handles: StromHandles,
        config: StromTestnetConfig
    ) -> eyre::Result<Self> {
        let strom = StromTestnetNodeInternals::new(
            testnet_node_id,
            strom_handles,
            network.strom_peer_network().network_handle.clone(),
            config
        )
        .await?;

        Ok(Self { testnet_node_id, network, strom })
    }

    pub async fn connect_to_all_peers(&mut self, other_peers: &HashMap<u64, Self>) {
        self.network.start_network();
        other_peers.iter().for_each(|(_, peer)| {
            self.network
                .strom_peer_network()
                .add_validator(peer.network.pubkey());
            self.network.eth_peer_handle().connect_to_peer(
                peer.network.pubkey(),
                peer.network.eth_peer_handle().socket_addr()
            );
        });

        self.network.initialize_connections(other_peers.len()).await;
    }

    pub fn send_bundles_to_network(&self, peer_id: PeerId, bundles: usize) -> eyre::Result<()> {
        let orders = AllOrders::gen_many(bundles);
        let num_orders = orders.len();
        tracing::debug!("submitting a angstrom bundle with {num_orders} orders to the network");

        self.strom
            .tx_strom_handles
            .network_tx
            .send(NetworkOrderEvent::IncomingOrders { peer_id, orders })?;

        tracing::debug!("sent {num_orders} bundles to the network");

        Ok(())
    }

    pub async fn execute_bundles_locally(&self) -> eyre::Result<()> {
        let orders = ContractBundle::gen();
        let hashes = orders.get_filled_hashes();
        tracing::debug!("executing a angstrom bundle with hashes: {:#?}", hashes);

        let tx_hash = self
            .strom
            .testnet_hub
            .execute(orders.abi_encode().into())
            .send()
            .await?
            .watch()
            .await?;

        tracing::debug!(?tx_hash, "tx hash with angstrom contract sent");

        Ok(())
    }
}
