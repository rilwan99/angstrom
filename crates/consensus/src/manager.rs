use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll}
};

use alloy_primitives::BlockNumber;
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
use tokio::{
    select,
    sync::{
        broadcast::{self, Receiver as BroadcastReceiver, Sender as BroadcastSender},
        mpsc::{channel, unbounded_channel, Receiver, Sender, UnboundedReceiver}
    },
    task::{JoinHandle, JoinSet}
};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};
use tracing::{error, warn};

use crate::{
    leader_election::WeightedRoundRobin,
    round::{ConsensusRoundState, DataMsg, RoundState},
    signer::Signer,
    ConsensusListener, ConsensusMessage, ConsensusState as ConsensusStateEnum, ConsensusUpdater
};

enum ConsensusTaskResult {
    BuiltProposal(Proposal),
    ValidationSolutions { height: BlockNumber, solutions: Vec<PoolSolution> }
}

pub async fn build_proposal_task(
    preproposals: Vec<PreProposal>
) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(preproposals).await
}

#[allow(unused)]
pub struct ConsensusManager {
    // should we change behavior when we are leader and when we are not
    // those are per round and should probably not be here
    pre_proposals:  HashMap<PeerId, PreProposal>,
    proposal:       HashMap<BlockNumber, Proposal>,
    commits:        HashMap<PeerId, Commit>,
    current_height: BlockNumber,
    leader:         PeerId,

    // those are cross round and are changed internally
    leader_selection: WeightedRoundRobin,
    state_transition: RoundState,

    // those are cross-round and immutable
    signer:           Signer,
    data_tx:          Sender<DataMsg>,

    order_storage:    Arc<OrderStorage>,
    // core: ConsensusCore,
    /// keeps track of the current round state
    // state_transition:             RoundState,
    // globalstate:            Arc<Mutex<GlobalConsensusState>>,
    // command:                ReceiverStream<ConsensusCommand>,
    /// Used to trigger new consensus rounds
    canonical_block_stream: BroadcastStream<CanonStateNotification>,
    /// events from the network,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>,
    network:                StromNetworkHandle,

    // signer:        Signer,
    // order_storage: Arc<OrderStorage>,
    // cache:         ProposalCache,
    tasks:   JoinSet<ConsensusTaskResult>,
    metrics: ConsensusMetricsWrapper
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
            pre_proposals: HashMap::new(),
            commits: HashMap::new(),
            proposal: HashMap::new(),
            leader: PeerId::default(),
            current_height,
            leader_selection: WeightedRoundRobin::new(Vec::new(), None),
            signer,
            data_tx,
            state_transition: RoundState::new(
                ConsensusRoundState::OrderAccumulator {
                    orders:       Vec::new(),
                    block_height: current_height
                },
                data_rx
            ),
            network,
            order_storage,
            tasks: JoinSet::new(),
            canonical_block_stream: wrapped_broadcast_stream,
            metrics: ConsensusMetricsWrapper::new()
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

    fn send_preproposal(&mut self) {
        let orders = self.order_storage.get_all_orders();
        let preproposal = PreProposal::new(
            self.current_height,
            &self.signer.key,
            alloy_primitives::FixedBytes::default(),
            orders
        );
        tracing::info!("Sending out preproposal");
        self.network
            .broadcast_tx(StromMessage::PrePropose(preproposal.clone()));
    }

    fn start_build_proposal(&mut self) {
        let pre_proposals: Vec<PreProposal> = self.pre_proposals.values().cloned().collect();
        let ethereum_block = self.current_height;
        let signer = self.signer.clone();
        let metrics = self.metrics.clone();
        let build_handle = self.tasks.spawn(async move {
            let (res, timer) = async_time_fn(|| async move {
                let solutions = build_proposal_task(pre_proposals.clone()).await.unwrap();
                let proposal = signer.sign_proposal(ethereum_block, pre_proposals, solutions);
                ConsensusTaskResult::BuiltProposal(proposal)
            })
            .await;
            metrics.set_proposal_build_time(ethereum_block, timer);
            res
        });
    }

    fn start_verify_proposal(&mut self, proposal: &Proposal) -> bool {
        if proposal.validate() {
            let preproposals = proposal.preproposals().clone();
            let height = proposal.ethereum_height;
            let metrics = self.metrics.clone();
            self.tasks.spawn(async move {
                let (res, timer) = async_time_fn(|| async move {
                    let solutions = build_proposal_task(preproposals).await.unwrap();
                    ConsensusTaskResult::ValidationSolutions { height, solutions }
                })
                .await;
                metrics.set_proposal_verification_time(height, timer);
                res
            });
            return true
        }
        false
    }

    fn broadcast_proposal(&mut self, proposal: Proposal) {
        self.network
            .broadcast_tx(StromMessage::Propose(proposal.clone()));
    }

    fn on_task_complete(&mut self, task_result: ConsensusTaskResult) {
        match task_result {
            ConsensusTaskResult::ValidationSolutions { height, solutions } => {
                let Some(proposal) = self.proposal.get(&height) else {
                    error!(
                        "Built solutions for a proposal that we don't have in our cache: {}",
                        height
                    );
                    return;
                };

                if proposal.solutions != solutions {
                    warn!("Proposal for {} failed validation with non-matching signatures", height);
                    return
                }
                let commit = self.signer.sign_commit(proposal);
                self.network
                    .broadcast_tx(StromMessage::Commit(Box::new(commit.clone())));
            }
            ConsensusTaskResult::BuiltProposal(p) => {
                self.broadcast_proposal(p);
            }
        }
    }

    fn on_blockchain_state(&mut self, notification: CanonStateNotification) {
        let new_block = notification.tip();
        let new_block_height = new_block.block.number;

        if self.current_height + 1 == new_block_height {
            let new_leader = self.leader_selection.choose_proposer();
            let orders = Vec::new();
            self.state_transition.reset(new_block_height, orders);
        } else {
            tracing::error!("Block height is not sequential, this breaks round robin!");
            panic!("Unrecoverable consensus error - Block height not sequential");
        }
    }

    fn on_network_event(&mut self, event: StromConsensusEvent) {
        match event {
            StromConsensusEvent::PrePropose(peer, pre_proposal) => {
                self.pre_proposals.insert(peer, pre_proposal);
            }
            StromConsensusEvent::Propose(_peer, proposal) => {
                if !self.start_verify_proposal(&proposal) {
                    warn!("Proposal failed verification with invalid signatures");
                }
            }
            StromConsensusEvent::Commit(peer, mut commit) => {
                let block_height = commit.block_height;
                commit.validate(&[]);
                if commit.signed_by(self.signer.validator_id) {
                } else {
                    commit.add_signature(self.signer.validator_id, &self.signer.bls_key);
                    self.network
                        .broadcast_tx(angstrom_network::StromMessage::Commit(commit.clone()));
                    self.commits.insert(peer, *commit);
                }
                self.metrics.set_commit_time(block_height);
            }
        }
    }

    fn trigger_next_state(&mut self, new_state: ConsensusRoundState) {
        match new_state {
            ConsensusRoundState::OrderAccumulator { .. } => self.send_preproposal(),
            ConsensusRoundState::PrePropose { .. } => {
                if self.leader == self.signer.my_id {
                    self.start_build_proposal()
                }
            }
            _ => ()
        }
    }

    pub async fn message_loop(mut self) {
        loop {
            select! {
                Some(msg) = self.tasks.join_next() => {
                    match msg {
                        Ok(task_result) => self.on_task_complete(task_result),
                        Err(e) => {
                            if !e.is_cancelled() {
                                tracing::error!("Task error: {}", e)
                            }
                        }
                    }
                },
                Some(msg) = self.canonical_block_stream.next() => {
                    match msg {
                        Ok(notification) => self.on_blockchain_state(notification),
                        Err(e) => tracing::error!("Error receiving chain state notification: {}", e)
                    };
                    self.metrics.set_block_height(self.current_height);
                },
                Some(msg) = self.strom_consensus_event.next() => {
                    self.on_network_event(msg);
                },
                Some(msg) = self.state_transition.next() => {
                    self.trigger_next_state(msg);
                },
            }
        }
    }
}
