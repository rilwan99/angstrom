use std::{
    collections::{HashMap, HashSet},
    future::Future
};
pub mod config;

use angstrom::cli::initialize_strom_handles;
use angstrom_network::{NetworkOrderEvent, StromMessage};
use angstrom_types::sol_bindings::grouped_orders::AllOrders;
use config::StromTestnetConfig;
use futures::StreamExt;
use node::TestnetNode;
use rand::Rng;
use reth_chainspec::Hardforks;
use reth_metrics::common::mpsc::{
    metered_unbounded_channel, UnboundedMeteredReceiver, UnboundedMeteredSender
};
use reth_provider::{test_utils::NoopProvider, BlockReader, ChainSpecProvider, HeaderProvider};
use tracing::{instrument, span, Instrument, Level};

use crate::network::peers::TestnetNodeNetwork;

pub mod node;

pub mod strom_internals;

#[derive(Default)]
pub struct StromTestnet<C> {
    peers:               HashMap<u64, TestnetNode<C>>,
    disconnected_peers:  HashSet<u64>,
    dropped_peers:       HashSet<u64>,
    current_max_peer_id: u64,
    config:              StromTestnetConfig
}

impl<C> StromTestnet<C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    pub async fn spawn_testnet(c: C, config: StromTestnetConfig) -> eyre::Result<Self> {
        let mut this = Self {
            peers: Default::default(),
            disconnected_peers: HashSet::new(),
            dropped_peers: HashSet::new(),
            current_max_peer_id: 0,
            config
        };

        tracing::info!("initializing testnet with {} nodes", config.intial_node_count);
        this.spawn_new_nodes(c, config.intial_node_count).await?;

        Ok(this)
    }

    pub async fn spawn_new_nodes(&mut self, c: C, number_nodes: u64) -> eyre::Result<()> {
        for _ in 0..number_nodes {
            self.spawn_new_node(c.clone()).await?;
        }

        Ok(())
    }

    pub async fn spawn_new_node(&mut self, c: C) -> eyre::Result<()> {
        let node_id = self.incr_peer_id();
        self.initialize_new_node(c, node_id).await?;

        Ok(())
    }

    #[instrument(name = "node", skip(self, node_id, c), fields(id = node_id))]
    async fn initialize_new_node(&mut self, c: C, node_id: u64) -> eyre::Result<()> {
        tracing::info!("spawning node");
        let strom_handles = initialize_strom_handles();
        let network = TestnetNodeNetwork::new_fully_configed(
            node_id,
            c,
            Some(strom_handles.pool_tx.clone()),
            Some(strom_handles.consensus_tx_op.clone())
        )
        .await;

        let mut node = TestnetNode::new(node_id, network, strom_handles, self.config).await?;
        node.connect_to_all_peers(&mut self.peers).await;

        if node_id != 0 {
            self.single_peer_update_state(0, node_id).await?;
        }

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

    pub fn get_peer(&self, id: u64) -> &TestnetNode<C> {
        self.peers.get(&id).expect(&format!("peer {id} not found"))
    }

    fn get_peer_mut(&mut self, id: u64) -> &mut TestnetNode<C> {
        self.peers
            .get_mut(&id)
            .expect(&format!("peer {id} not found"))
    }

    pub fn get_random_peer(&self, not_allowed_ids: Vec<u64>) -> &TestnetNode<C> {
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

    /// updates the anvil state of all the peers from a given peer
    pub async fn all_peers_update_state(&self, id: u64) -> eyre::Result<()> {
        let peer = self.get_peer(id);
        let (updated_state, _) = peer.state_provider().execute_and_return_state().await?;

        futures::future::join_all(self.peers.iter().map(|(i, peer)| async {
            if id != *i {
                peer.state_provider()
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

    /// updates the anvil state of all the peers from a given peer
    pub async fn single_peer_update_state(
        &self,
        state_from_id: u64,
        state_to_id: u64
    ) -> eyre::Result<()> {
        let peer_to_get = self.get_peer(state_from_id);
        let state = peer_to_get.state_provider().return_state().await?;

        let peer_to_set = self.peers.get(&state_to_id).expect("peer doesn't exists");
        peer_to_set.state_provider().set_state(state).await?;

        Ok(())
    }

    /// takes a random peer and gets them to broadcast the message. we then
    /// take all other peers and ensure that they received the message.
    pub async fn broadcast_message_orders(
        &mut self,
        id: Option<u64>,
        sent_msg: StromMessage,
        expected_orders: Vec<AllOrders>
    ) -> bool {
        let out = self
            .run_network_order_event_on_all_peers_with_expection(
                id.unwrap_or_else(|| self.random_valid_id()),
                |peer| {
                    let network_handle = peer.strom_network_handle().clone();
                    let peer_id = peer.peer_id();

                    async move {
                        network_handle.broadcast_message(sent_msg.clone());
                        peer_id
                    }
                },
                |other_rxs, peer_id| async move {
                    futures::future::join_all(other_rxs.into_iter().map(|mut rx| {
                        let value = expected_orders.clone();
                        async move {
                            (Some(NetworkOrderEvent::IncomingOrders { peer_id, orders: value })
                                == rx.next().await) as usize
                        }
                    }))
                    .await
                    .into_iter()
                    .sum::<usize>()
                }
            )
            .await;

        out == self.peers.len() - 1
    }

    /// if None, then a random id is used
    pub async fn run_event<'a, 'b, F, O>(&'a self, id: Option<u64>, f: F) -> O::Output
    where
        F: FnOnce(&'b TestnetNode<C>) -> O,
        O: Future,
        'a: 'b
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
        let span = span!(Level::TRACE, "testnet node", ?id);
        f(&peer).instrument(span).await
    }

    /// runs an event that does something with the network orderpool
    /// and compared some expected result against all peers
    pub async fn run_network_order_event_on_all_peers_with_expection<F, P, O, R>(
        &mut self,
        exception_id: u64,
        network_f: F,
        expected_f: P
    ) -> R::Output
    where
        F: FnOnce(&TestnetNode<C>) -> O,
        P: FnOnce(Vec<UnboundedMeteredReceiver<NetworkOrderEvent>>, O::Output) -> R,
        O: Future,
        R: Future
    {
        let (old_peer_channels, rx_channels): (Vec<_>, Vec<_>) = self
            .peers
            .iter_mut()
            .filter(|(id, _)| **id != exception_id)
            .map(|(id, peer)| {
                let (new_tx, new_rx) = metered_unbounded_channel("new orderpool");
                let old_tx = peer.pre_post_network_event_pool_manager_swap(new_tx, true);

                ((*id, old_tx), new_rx)
            })
            .unzip();

        let event_out = self.run_event(Some(exception_id), network_f).await;

        self.peers
            .iter()
            .filter(|(id, _)| **id != exception_id)
            .for_each(|(_, peer)| {
                peer.start_network(true);
            });

        let out = expected_f(rx_channels, event_out).await;

        old_peer_channels.into_iter().for_each(|(id, old_tx)| {
            let peer = self.get_peer_mut(id);
            let _ = peer.pre_post_network_event_pool_manager_swap(old_tx, false);
        });

        out
    }
}
