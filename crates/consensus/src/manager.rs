use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll}
};

use alloy_primitives::{private::proptest::collection::vec, BlockNumber};
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::{manager::StromConsensusEvent, Peer, StromMessage, StromNetworkHandle};
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution
};
use futures::{FutureExt, Stream, StreamExt};
use matching_engine::MatchingManager;
use order_pool::{order_storage::OrderStorage, timer::async_time_fn};
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_primitives::transaction::WithEncoded;
use reth_provider::{CanonStateNotification, CanonStateNotifications};
use reth_rpc_types::PeerId;
use reth_tasks::TaskSpawner;
use serde_json::error::Category::Data;
use tokio::{
    select,
    sync::mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver},
    task::{JoinHandle, JoinSet}
};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};
use tracing::{error, warn};

use crate::{
    leader_election::WeightedRoundRobin,
    round::{ConsensusRoundState, DataMsg, RoundState},
    signer::Signer,
    ConsensusListener, ConsensusMessage, ConsensusState as ConsensusStateEnum, ConsensusState,
    ConsensusUpdater
};

enum ConsensusTaskResult {
    BuiltProposal(Proposal),
    ValidationSolutions { height: BlockNumber, solutions: Vec<PoolSolution> }
}

#[allow(unused)]
pub struct ConsensusManager {
    // should we change behavior when we are leader and when we are not
    // those are per round and should probably not be here
    current_height: BlockNumber,
    leader:         PeerId,

    // those are cross round and are changed internally
    leader_selection: WeightedRoundRobin,
    state_transition: RoundState,

    // those are cross-round and immutable
    // signer:           Signer,
    data_tx: Sender<DataMsg>,

    order_storage:          Arc<OrderStorage>,
    // core: ConsensusCore,
    /// keeps track of the current round state
    // state_transition:             RoundState,
    // globalstate:            Arc<Mutex<GlobalConsensusState>>,
    // command:                ReceiverStream<ConsensusCommand>,
    /// Used to trigger new consensus rounds
    canonical_block_stream: BroadcastStream<CanonStateNotification>,
    /// events from the network,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>,
    network:                StromNetworkHandle
    // signer:        Signer,
    // order_storage: Arc<OrderStorage>,
    // cache:         ProposalCache,
    // tasks:   JoinSet<ConsensusTaskResult>,
    // metrics: ConsensusMetricsWrapper
}

pub struct ManagerNetworkDeps {
    network:                StromNetworkHandle,
    canonical_block_stream: CanonStateNotifications,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>
}

impl ManagerNetworkDeps {
    pub fn new(
        network: StromNetworkHandle,
        canonical_block_stream: CanonStateNotifications,
        strom_consensus_event: UnboundedMeteredReceiver<StromConsensusEvent>
    ) -> Self {
        Self { network, canonical_block_stream, strom_consensus_event }
    }
}

impl ConsensusManager {
    fn new(netdeps: ManagerNetworkDeps, signer: Signer, order_storage: Arc<OrderStorage>) -> Self {
        let ManagerNetworkDeps { network, canonical_block_stream, strom_consensus_event } = netdeps;
        let wrapped_broadcast_stream = BroadcastStream::new(canonical_block_stream);
        let (data_tx, data_rx) = channel::<DataMsg>(100);
        let current_height = 0;
        Self {
            strom_consensus_event,
            leader: PeerId::default(),
            current_height,
            leader_selection: WeightedRoundRobin::new(Vec::new(), None),
            data_tx,
            state_transition: RoundState::new(
                ConsensusRoundState::OrderAccumulator {
                    orders:       Vec::new(),
                    block_height: current_height
                },
                data_rx,
                signer,
                ConsensusMetricsWrapper::new()
            ),
            network,
            order_storage,
            canonical_block_stream: wrapped_broadcast_stream
        }
    }

    pub fn spawn<TP: TaskSpawner>(
        tp: TP,
        netdeps: ManagerNetworkDeps,
        signer: Signer,
        order_storage: Arc<OrderStorage>
    ) -> JoinHandle<()> {
        // let tx = netdeps.tx.clone();
        let manager = ConsensusManager::new(netdeps, signer, order_storage);
        let fut = manager.message_loop().boxed();
        // let fut = manager_thread(globalstate, netdeps, signer, order_storage,
        // timings).boxed();
        tp.spawn_critical("consensus", fut)
        // ConsensusHandle { sender: tx }
    }

    fn send_preproposal(&mut self, preproposal: PreProposal) {
        tracing::info!("Sending out preproposal");
        self.network
            .broadcast_tx(StromMessage::PrePropose(preproposal.clone()));
    }

    fn broadcast_proposal(&mut self, proposal: Proposal) {
        self.network
            .broadcast_tx(StromMessage::Propose(proposal.clone()));
    }

    fn on_blockchain_state(&mut self, notification: CanonStateNotification) {
        let new_block = notification.tip();
        let new_block_height = new_block.block.number;

        if self.current_height + 1 == new_block_height {
            self.leader = self.leader_selection.choose_proposer();
            self.current_height = new_block_height;
            let orders = Vec::new();
            self.state_transition
                .force_transition(ConsensusRoundState::OrderAccumulator {
                    block_height: new_block_height,
                    orders
                });
        } else {
            tracing::error!("Block height is not sequential, this breaks round robin!");
            panic!("Unrecoverable consensus error - Block height not sequential");
        }
    }

    fn on_network_event(&mut self, event: StromConsensusEvent) {
        match event {
            StromConsensusEvent::PrePropose(peer_id, pre_proposal) => {
                self.data_tx
                    .send(DataMsg::PreProposal(peer_id, pre_proposal));
                // self.pre_proposals.insert(peer, pre_proposal);
            }
            StromConsensusEvent::Propose(peer_id, proposal) => {
                self.data_tx
                    .send(DataMsg::Proposal(peer_id, proposal.clone()));
                self.state_transition
                    .force_transition(ConsensusRoundState::Propose {
                        block_height: self.current_height,
                        proposal
                    });
                // if !self.start_verify_proposal(&proposal) {
                //     warn!("Proposal failed verification with invalid
                // signatures"); }
            }
            StromConsensusEvent::Commit(peer_id, commit) => {
                self.data_tx.send(DataMsg::Commit(peer_id, *commit.clone()));
                self.state_transition
                    .force_transition(ConsensusRoundState::Commit {
                        block_height: self.current_height,
                        commits:      vec![*commit.clone()]
                    });
                // let block_height = commit.block_height;
                // commit.validate(&[]);
                // if commit.signed_by(self.signer.validator_id) {
                //
                // } else {
                //     commit.add_signature(self.signer.validator_id,
                // &self.signer.bls_key);     self.commits.
                // insert(peer, *commit);     self.network
                //         .broadcast_tx(angstrom_network::StromMessage::Commit(commit.clone()));
                // }
                // self.metrics.set_commit_time(block_height);
            }
        }
    }

    pub fn on_state_transition(&mut self, state: ConsensusRoundState) {}

    pub async fn message_loop(mut self) {
        loop {
            select! {
                Some(msg) = self.canonical_block_stream.next() => {
                    match msg {
                        Ok(notification) => self.on_blockchain_state(notification),
                        Err(e) => tracing::error!("Error receiving chain state notification: {}", e)
                    };
                },
                Some(msg) = self.strom_consensus_event.next() => {
                    self.on_network_event(msg);
                },
                Some(msg) = self.state_transition.next() => {
                    self.on_state_transition(msg);
                },
            }
        }
    }
}
