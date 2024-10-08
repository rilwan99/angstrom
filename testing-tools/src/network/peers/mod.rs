mod eth_peer;
use futures::FutureExt;

mod network_future;
mod strom_peer;
use std::{
    collections::HashSet,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
    task::{Context, Poll}
};

use alloy_chains::Chain;
use alloy_primitives::Address;
use angstrom_network::{
    manager::StromConsensusEvent, state::StromState, NetworkOrderEvent, StatusState,
    StromNetworkManager, StromProtocolHandler, StromSessionManager, Swarm, VerificationSidecar
};
pub use eth_peer::*;
use network_future::TestnetPeerFuture;
use parking_lot::RwLock;
use rand::thread_rng;
use reth_chainspec::Hardforks;
use reth_metrics::common::mpsc::{MeteredPollSender, UnboundedMeteredSender};
use reth_network::test_utils::PeerConfig;
use reth_network_peers::{pk2id, PeerId};
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
pub use strom_peer::*;
use tokio::task::JoinHandle;
use tokio_util::sync::PollSender;
use tracing::{span, Level};

use crate::network::peers::StromNetworkPeer;

pub struct TestnetNodeNetwork {
    // eth components
    eth_handle:   EthNetworkPeer,
    // strom components
    strom_handle: StromNetworkPeer,
    secret_key:   SecretKey,
    pubkey:       PeerId,
    running:      Arc<AtomicBool>,
    network_futs: JoinHandle<()>
}

impl TestnetNodeNetwork {
    pub async fn new_fully_configed<C>(
        testnet_node_id: u64,
        c: C,
        to_pool_manager: Option<UnboundedMeteredSender<NetworkOrderEvent>>,
        to_consensus_manager: Option<UnboundedMeteredSender<StromConsensusEvent>>
    ) -> Self
    where
        C: BlockReader
            + HeaderProvider
            + ChainSpecProvider
            + Unpin
            + Clone
            + ChainSpecProvider<ChainSpec: Hardforks>
            + 'static
    {
        let mut rng = thread_rng();
        let sk = SecretKey::new(&mut rng);
        let peer = PeerConfig::with_secret_key(c.clone(), sk);

        let secp = Secp256k1::default();
        let pub_key = sk.public_key(&secp);

        let peer_id = pk2id(&pub_key);
        let state = StatusState {
            version:   0,
            chain:     Chain::mainnet().id(),
            peer:      peer_id,
            timestamp: 0
        };
        let (session_manager_tx, session_manager_rx) = tokio::sync::mpsc::channel(100);
        let sidecar = VerificationSidecar {
            status:       state,
            has_sent:     false,
            has_received: false,
            secret_key:   sk
        };

        let validators: HashSet<Address> = HashSet::default();
        let validators = Arc::new(RwLock::new(validators));

        let protocol = StromProtocolHandler::new(
            MeteredPollSender::new(PollSender::new(session_manager_tx), "session manager"),
            sidecar,
            validators.clone()
        );

        let state = StromState::new(c.clone(), validators.clone());
        let sessions = StromSessionManager::new(session_manager_rx);
        let swarm = Swarm::new(sessions, state);

        let strom_network = StromNetworkManager::new(swarm, to_pool_manager, to_consensus_manager);

        let mut eth_peer = peer.launch().await.unwrap();
        eth_peer.network_mut().add_rlpx_sub_protocol(protocol);

        let strom_handle = StromNetworkPeer::new(&strom_network);
        let eth_handle = EthNetworkPeer::new(&eth_peer);

        let span = span!(Level::DEBUG, "testnet node", testnet_node_id);

        let running = Arc::new(AtomicBool::new(false));
        let futs = TestnetPeerFuture::new(eth_peer, strom_network, running.clone(), span);

        Self {
            strom_handle,
            secret_key: sk,
            pubkey: peer_id,
            network_futs: tokio::spawn(futs),
            eth_handle,
            running
        }
    }

    pub fn new_with_consensus() -> Self {
        todo!("consensus not configured for test peer")
    }

    pub fn new_with_tx_pool() -> Self {
        todo!("tx pool not configured for test peer")
    }

    pub fn pubkey(&self) -> PeerId {
        self.pubkey
    }

    pub fn strom_peer_network(&self) -> &StromNetworkPeer {
        &self.strom_handle
    }

    pub fn eth_peer_handle(&self) -> &EthNetworkPeer {
        &self.eth_handle
    }

    pub fn stop_network(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn start_network(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    pub fn is_network_off(&self) -> bool {
        self.running.load(Ordering::Relaxed) == false
    }

    pub fn is_network_on(&self) -> bool {
        self.running.load(Ordering::Relaxed) == true
    }

    pub(crate) async fn initialize_connections(&mut self, connections_needed: usize) {
        tracing::debug!("attempting connections to {connections_needed} peers");
        let mut last_peer_count = 0;
        std::future::poll_fn(|cx| loop {
            if self.poll_fut_to_initialize(cx).is_ready() {
                tracing::error!("peer failed");
            }

            let peer_cnt = self.strom_peer_network().peer_count();
            if last_peer_count != peer_cnt {
                tracing::trace!("connected to {peer_cnt}/{connections_needed} peers");
                last_peer_count = peer_cnt;
            }

            if connections_needed == peer_cnt {
                return Poll::Ready(())
            }
        })
        .await
    }

    fn poll_fut_to_initialize(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.network_futs.poll_unpin(cx).map(|_| ())
    }
}

impl Drop for TestnetNodeNetwork {
    fn drop(&mut self) {
        self.network_futs.abort();
    }
}
