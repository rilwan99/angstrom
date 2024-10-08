use std::{
    collections::{HashMap, HashSet},
    future::Future
};
pub mod config;

use angstrom::cli::initialize_strom_handles;
use config::StromTestnetConfig;
use node::TestnetNode;
use rand::Rng;
use reth_provider::test_utils::NoopProvider;
use tracing::{instrument, span, Instrument, Level};

use crate::network::peers::TestnetNodeNetwork;

pub mod node;

pub mod strom_internals;

#[derive(Default)]
pub struct StromTestnet {
    peers:               HashMap<u64, TestnetNode>,
    disconnected_peers:  HashSet<u64>,
    dropped_peers:       HashSet<u64>,
    current_max_peer_id: u64,
    config:              StromTestnetConfig
}

impl StromTestnet {
    pub async fn spawn_testnet(config: StromTestnetConfig) -> eyre::Result<Self> {
        let mut this = Self {
            peers: Default::default(),
            disconnected_peers: HashSet::new(),
            dropped_peers: HashSet::new(),
            current_max_peer_id: 0,
            config
        };

        tracing::info!("initializing testnet with {} nodes", config.intial_node_count);
        this.spawn_new_nodes(config.intial_node_count).await?;

        Ok(this)
    }

    pub async fn spawn_new_nodes(&mut self, number_nodes: u64) -> eyre::Result<()> {
        for _ in 0..number_nodes {
            self.spawn_new_node().await?;
        }

        Ok(())
    }

    pub async fn spawn_new_node(&mut self) -> eyre::Result<()> {
        let node_id = self.incr_peer_id();
        self.initialize_new_node(node_id).await?;

        Ok(())
    }

    #[instrument(name = "node", skip(self, node_id), fields(id = node_id))]
    async fn initialize_new_node(&mut self, node_id: u64) -> eyre::Result<()> {
        tracing::info!("spawning node");
        let strom_handles = initialize_strom_handles();
        let network = TestnetNodeNetwork::new_fully_configed(
            node_id,
            NoopProvider::default(),
            Some(strom_handles.pool_tx.clone()),
            Some(strom_handles.consensus_tx_op.clone())
        )
        .await;

        let mut node = TestnetNode::new(node_id, network, strom_handles, self.config).await?;
        node.connect_to_all_peers(&mut self.peers).await;

        self.peers.insert(node_id, node);

        Ok(())
    }

    /// increments the `current_max_peer_id` and returns the previous value
    fn incr_peer_id(&mut self) -> u64 {
        let prev_id = self.current_max_peer_id;
        self.current_max_peer_id += 1;
        prev_id
    }

    fn random_valid_id(&self) -> u64 {
        let ids = self.peers.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        let id_idx = rand::thread_rng().gen_range(0..ids.len());
        ids[id_idx]
    }

    pub fn get_peer(&self, id: u64) -> &TestnetNode {
        self.peers.get(&id).expect(&format!("peer {id} not found"))
    }

    pub fn get_random_peer(&self, not_allowed_ids: Vec<u64>) -> &TestnetNode {
        assert!(self.peers.len() != 0);

        let peer_ids = self
            .peers
            .iter()
            .map(|(id, _)| *id)
            .filter(|id| !not_allowed_ids.contains(&id))
            .collect::<Vec<_>>();

        if peer_ids.len() == 0 {
            panic!("not enough peers")
        }

        let mut random_peer = self.random_valid_id();
        while !peer_ids.contains(&random_peer) {
            random_peer = self.random_valid_id();
        }

        self.peers
            .get(&random_peer)
            .expect(&format!("peer {random_peer} not found"))
    }

    /// updates the anvil state of all the peers from a given peer from a given
    /// peer. Returns the latest block
    pub async fn update_state(&self, id: u64) -> eyre::Result<()> {
        let peer = self.get_peer(id);
        let (updated_state, _) = peer.strom.state_provider.execute_and_return_state().await?;

        futures::future::join_all(self.peers.iter().map(|(i, peer)| async {
            if id != *i {
                peer.strom
                    .state_provider
                    .set_state(updated_state.clone())
                    .await?;
            }
            Ok::<_, eyre::ErrReport>(())
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }

    /// if None, then a random id is used
    pub async fn run_event<'a, F, O, R>(&'a self, id: Option<u64>, f: F) -> R
    where
        F: FnOnce(&'a TestnetNode) -> O,
        O: Future<Output = R>
    {
        let id = if let Some(i) = id {
            assert!(!self.peers.is_empty());
            assert!(self
                .peers
                .iter()
                .map(|(id, _)| *id)
                .collect::<HashSet<_>>()
                .contains(&i));
            i
        } else {
            self.random_valid_id()
        };

        let peer = self.peers.get(&id).unwrap();
        let span = span!(Level::DEBUG, "testnet node", ?id);
        let r = f(&peer).instrument(span).await;
        r
    }
}
