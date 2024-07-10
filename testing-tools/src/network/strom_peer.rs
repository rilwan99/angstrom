use std::{collections::HashSet, net::SocketAddr, sync::Arc, task::Poll};

use alloy_chains::Chain;
use angstrom_network::{
    manager::StromConsensusEvent, state::StromState, NetworkOrderEvent, StatusState,
    StromNetworkEvent, StromNetworkHandle, StromNetworkManager, StromProtocolHandler,
    StromSessionManager, Swarm, VerificationSidecar
};
use futures::{Future, FutureExt};
use parking_lot::RwLock;
use rand::thread_rng;
use reth_metrics::common::mpsc::{MeteredPollSender, UnboundedMeteredSender};
use reth_network::test_utils::{Peer, PeerConfig};
use reth_network_api::{PeerId, Peers};
use reth_network_peers::pk2id;
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};
use secp256k1::{PublicKey, Secp256k1};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::PollSender;
use tracing::{span, Level};

use crate::network::SecretKey;

pub struct StromPeer<C = NoopProvider> {
    /// the default ethereum network peer
    pub eth_peer:      Peer<C>,
    // strom extensions
    pub strom_network: StromNetworkManager<C>,
    pub handle:        StromNetworkHandle,
    pub secret_key:    SecretKey
}

impl<C: Unpin> StromPeer<C>
where
    C: BlockReader + HeaderProvider + Unpin + Clone
{
    pub async fn new(c: C) -> Self {
        let mut rng = thread_rng();
        let sk = SecretKey::new(&mut rng);
        let peer = PeerConfig::with_secret_key(c.clone(), sk);

        let secp = Secp256k1::default();
        let pub_key = sk.public_key(&secp);

        let state = StatusState {
            version:   0,
            chain:     Chain::mainnet(),
            peer:      pk2id(&pub_key),
            timestamp: 0
        };

        let validators: HashSet<Address> = HashSet::default();
        let validator_set = Arc::new(RwLock::new(validators));

        let verification = VerificationSidecar {
            status:       state,
            has_sent:     false,
            has_received: false,
            secret_key:   sk
        };

        let (session_manager_tx, session_manager_rx) = tokio::sync::mpsc::channel(100);

        let protocol = StromProtocolHandler::new(
            MeteredPollSender::new(PollSender::new(session_manager_tx), "session manager"),
            verification.clone(),
            validator_set.clone()
        );

        let state = StromState::new(c.clone(), validator_set.clone());
        let sessions = StromSessionManager::new(session_manager_rx);
        let swarm = Swarm::new(sessions, state);

        let network = StromNetworkManager::new(swarm, None, None);
        let mut peer = peer.launch().await.unwrap();
        peer.network_mut().add_rlpx_sub_protocol(protocol);
        let handle = network.get_handle();

        Self { strom_network: network, eth_peer: peer, secret_key: sk, handle }
    }

    pub fn new_with_consensus() -> Self {
        todo!("consensus not configured for test peer")
    }

    pub fn new_with_tx_pool() -> Self {
        todo!("tx pool not configured for test peer")
    }

    pub async fn new_fully_configed(
        c: C,
        sk: SecretKey,
        validators: Arc<RwLock<HashSet<Address>>>,
        to_pool_manager: Option<UnboundedMeteredSender<NetworkOrderEvent>>,
        to_consensus_manager: Option<UnboundedMeteredSender<StromConsensusEvent>>
    ) -> Self {
        let peer = PeerConfig::with_secret_key(c.clone(), sk);

        let secp = Secp256k1::default();
        let pub_key = sk.public_key(&secp);

        let state = StatusState {
            version:   0,
            chain:     Chain::mainnet(),
            peer:      pk2id(&pub_key),
            timestamp: 0
        };
        let (session_manager_tx, session_manager_rx) = tokio::sync::mpsc::channel(100);
        let sidecar = VerificationSidecar {
            status:       state,
            has_sent:     false,
            has_received: false,
            secret_key:   sk
        };

        let protocol = StromProtocolHandler::new(
            MeteredPollSender::new(PollSender::new(session_manager_tx), "session manager"),
            sidecar,
            validators.clone()
        );

        let state = StromState::new(c.clone(), validators.clone());
        let sessions = StromSessionManager::new(session_manager_rx);
        let swarm = Swarm::new(sessions, state);

        let network = StromNetworkManager::new(swarm, to_pool_manager, to_consensus_manager);

        let mut peer = peer.launch().await.unwrap();
        peer.network_mut().add_rlpx_sub_protocol(protocol);
        let handle = network.get_handle();

        Self { strom_network: network, eth_peer: peer, secret_key: sk, handle }
    }

    pub fn get_node_public_key(&self) -> PeerId {
        let pub_key = PublicKey::from_secret_key(&Secp256k1::default(), &self.secret_key);
        pk2id(&pub_key)
    }

    pub fn disconnect_peer(&self, id: PeerId) {
        self.handle.remove_peer(id)
    }

    pub fn get_peer_count(&self) -> usize {
        self.handle.peer_count()
    }

    pub fn remove_validator(&self, id: PeerId) {
        let addr = Address::from_raw_public_key(id.as_slice());
        let set = self.get_validator_set();
        set.write().remove(&addr);
    }

    pub fn add_validator(&self, id: PeerId) {
        let addr = Address::from_raw_public_key(id.as_slice());
        let set = self.get_validator_set();
        set.write().insert(addr);
    }

    pub fn connect_to_peer(&self, id: PeerId, addr: SocketAddr) {
        self.eth_peer.peer_handle().network().add_peer(id, addr);
    }

    pub fn socket_addr(&self) -> SocketAddr {
        self.eth_peer.local_addr()
    }

    fn get_validator_set(&self) -> Arc<RwLock<HashSet<Address>>> {
        self.strom_network.swarm().state().validators()
    }

    pub fn sub_network_events(&self) -> UnboundedReceiverStream<StromNetworkEvent> {
        self.handle.subscribe_network_events()
    }

    pub fn manager_mut(&mut self) -> &mut StromNetworkManager<C> {
        &mut self.strom_network
    }
}

impl<C> Future for StromPeer<C>
where
    C: BlockReader + HeaderProvider + Unpin + Clone
{
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        let peer_id = this.get_node_public_key();
        let span = span!(Level::TRACE, "peer_id: {:?}", ?peer_id);
        let e = span.enter();

        if this.strom_network.poll_unpin(cx).is_ready() {
            return Poll::Ready(())
        }
        if this.eth_peer.poll_unpin(cx).is_ready() {
            return Poll::Ready(())
        }

        drop(e);

        Poll::Pending
    }
}