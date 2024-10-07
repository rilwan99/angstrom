use std::{
    collections::HashSet,
    marker::PhantomData,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
    task::{Context, Poll}
};

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
use reth_network::test_utils::{Peer, PeerConfig, PeerHandle};
use reth_network_api::Peers;
use reth_network_peers::{pk2id, PeerId};
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};
use reth_transaction_pool::{
    blobstore::InMemoryBlobStore, noop::MockTransactionValidator, test_utils::MockTransaction,
    CoinbaseTipOrdering, Pool
};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tokio::task::JoinHandle;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::PollSender;
use tracing::{span, Instrument, Level, Span};

pub struct StromPeer {
    strom_network_handle: StromNetworkHandle,
    strom_validator_set:  Arc<RwLock<HashSet<Address>>>
}

impl StromPeer {
    pub fn new<C>(strom_network: &StromNetworkManager<C>) -> Self {
        Self {
            strom_network_handle: strom_network.get_handle(),
            strom_validator_set:  strom_network.swarm().state().validators().clone()
        }
    }
}
