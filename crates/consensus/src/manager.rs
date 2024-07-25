use std::{
    pin::Pin,
    sync::{Arc, Mutex}
};

use angstrom_network::{manager::StromConsensusEvent, StromMessage, StromNetworkHandle};
use angstrom_types::{
    consensus::{PreProposal, Proposal},
    orders::PoolSolution
};
use futures::{FutureExt, Stream, StreamExt};
use matching_engine::{MatchingEngineHandle, MatchingManager};
use order_pool::order_storage::OrderStorage;
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_provider::{CanonStateNotification, CanonStateNotifications};
use reth_tasks::TaskSpawner;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinSet
};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};
use tracing::{error, warn};
use validation::BundleValidator;

use crate::{
    cache::ProposalCache,
    global::GlobalConsensusState,
    round::{Leader, RoundState, RoundStateTimings},
    round_robin_algo::RoundRobinAlgo,
    signer::Signer,
    ConsensusListener, ConsensusMessage, ConsensusState, ConsensusUpdater
};

enum ConsensusTaskResult {
    BuiltProposal(Proposal),
    ValidationSolutions { height: u64, solutions: Vec<PoolSolution> }
}

pub async fn manager_thread(
    globalstate: Arc<Mutex<GlobalConsensusState>>,
    netdeps: ManagerNetworkDeps,
    signer: Signer,
    order_storage: Arc<OrderStorage>
) {
    let ManagerNetworkDeps { network, canonical_block_stream, strom_consensus_event, rx, .. } =
        netdeps;
    let stream = ReceiverStream::new(rx);

    // This is still a lot of stuff to track that we don't necessarily have to worry
    // about
    let timings = RoundStateTimings::new(6, 2, 6);
    let roundstate = RoundState::new(0, 1, Leader::default(), Some(timings));

    let wrapped_broadcast_stream = BroadcastStream::new(canonical_block_stream);
    // Use our default round robin algo
    let (roundrobin, _cacheheight) = RoundRobinAlgo::new();
    let tasks: JoinSet<ConsensusTaskResult> = JoinSet::new();
    let cache = ProposalCache::new();

    let mut manager = ConsensusManager {
        strom_consensus_event,
        roundstate,
        globalstate,
        subscribers: Vec::new(),
        command: stream,
        network,
        roundrobin,
        signer,
        order_storage,
        tasks,
        cache,
        canonical_block_stream: wrapped_broadcast_stream
    };

    // Start message loop
    loop {
        select! {
            Some(msg) = manager.tasks.join_next() => {
                match msg {
                    Ok(task_result) => manager.on_task_complete(task_result),
                    Err(e) => {
                        // Don't log an error if we cancelled the task, that error is expected
                        if !e.is_cancelled() {
                            tracing::error!("Task error: {}", e)
                        }
                    }
                }
            },
            Some(msg) = manager.command.next() => {
                manager.on_command(msg);
            },
            Some(msg) = manager.canonical_block_stream.next() => {
                match msg {
                    Ok(notification) => manager.on_blockchain_state(notification).await,
                    Err(e) => tracing::error!("Error receiving chain state notification: {}", e)
                }
            },
            Some(msg) = manager.strom_consensus_event.next() => {
                manager.on_network_event(msg);
            },
            Some(msg) = manager.roundstate.next() => {
                manager.on_new_globalstate(msg).await;
            }
        }
    }
}

pub async fn build_proposal_task(
    preproposals: Vec<PreProposal>
) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(preproposals).await
}

#[allow(unused)]
pub struct ConsensusManager {
    // core: ConsensusCore,
    /// keeps track of the current round state
    roundstate:             RoundState,
    globalstate:            Arc<Mutex<GlobalConsensusState>>,
    command:                ReceiverStream<ConsensusCommand>,
    subscribers:            Vec<Sender<ConsensusMessage>>,
    /// Used to trigger new consensus rounds
    canonical_block_stream: BroadcastStream<CanonStateNotification>,
    /// events from the network,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>,
    network:                StromNetworkHandle,

    roundrobin:    RoundRobinAlgo,
    signer:        Signer,
    order_storage: Arc<OrderStorage>,
    cache:         ProposalCache,
    tasks:         JoinSet<ConsensusTaskResult>
}

pub struct ManagerNetworkDeps {
    network:                StromNetworkHandle,
    canonical_block_stream: CanonStateNotifications,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>,
    tx:                     Sender<ConsensusCommand>,
    rx:                     Receiver<ConsensusCommand>
}

impl ManagerNetworkDeps {
    pub fn new(
        network: StromNetworkHandle,
        canonical_block_stream: CanonStateNotifications,
        strom_consensus_event: UnboundedMeteredReceiver<StromConsensusEvent>,
        tx: Sender<ConsensusCommand>,
        rx: Receiver<ConsensusCommand>
    ) -> Self {
        Self { network, canonical_block_stream, strom_consensus_event, tx, rx }
    }
}

impl ConsensusManager {
    pub fn spawn<TP: TaskSpawner>(
        tp: TP,
        globalstate: Arc<Mutex<GlobalConsensusState>>,
        netdeps: ManagerNetworkDeps,
        signer: Signer,
        order_storage: Arc<OrderStorage>
    ) -> ConsensusHandle {
        let tx = netdeps.tx.clone();
        let fut = manager_thread(globalstate, netdeps, signer, order_storage).boxed();

        tp.spawn_critical("consensus", fut);

        ConsensusHandle { sender: tx }
    }

    fn send_preproposal(&mut self) {
        let orders = self.order_storage.get_all_orders();
        let preproposal = PreProposal::new(
            self.roundstate.current_height(),
            &self.signer.key,
            alloy_primitives::FixedBytes::default(),
            orders
        );
        tracing::info!("Sending out preproposal");
        self.network
            .broadcast_tx(StromMessage::PrePropose(preproposal.clone()));
        self.broadcast(ConsensusMessage::PrePropose(preproposal));
    }

    /// Start the Build Proposal job to produce a signed Proposal out of this
    /// round's preproposals
    fn start_build_proposal(&mut self) {
        let preproposals = self.roundstate.get_preproposals();
        let signer = self.signer.clone();
        let ethereum_block = self.roundstate.current_height();
        let build_handle = self.tasks.spawn(async move {
            let solutions = build_proposal_task(preproposals.clone()).await.unwrap();
            let proposal = signer
                .sign_proposal(ethereum_block, preproposals, solutions)
                .unwrap();
            ConsensusTaskResult::BuiltProposal(proposal)
        });
        self.roundstate.on_proposal(build_handle);
    }

    /// Start to verify our proposal.  This method will return `false` if the
    /// proposal fails signature verification.  Presuming signature verification
    /// is passed, this proposal will be stored in our Proposal Store and a task
    /// will be launched to verify the solutions produced within the proposal
    fn start_verify_proposal(&mut self, proposal: &Proposal) -> bool {
        if proposal.validate() {
            let preproposals = proposal.preproposals().clone();
            let height = proposal.ethereum_height;
            self.tasks.spawn(async move {
                let solutions = build_proposal_task(preproposals).await.unwrap();
                ConsensusTaskResult::ValidationSolutions { height, solutions }
            });
            // We've passed basic validation so we can return true here
            return true;
        }
        false
    }

    /// Send a built proposal out over the network
    fn broadcast_proposal(&mut self, proposal: Proposal) {
        self.network
            .broadcast_tx(StromMessage::Propose(proposal.clone()));
        self.broadcast(ConsensusMessage::Proposal(proposal));
    }

    fn on_command(&mut self, command: ConsensusCommand) {
        match command {
            ConsensusCommand::Subscribe(sender) => self.subscribers.push(sender)
        }
    }

    /// Respond to the messages our spawned tasks pass back when complete
    fn on_task_complete(&mut self, task_result: ConsensusTaskResult) {
        match task_result {
            ConsensusTaskResult::ValidationSolutions { height, solutions } => {
                warn!("Finished validation job");
                // Get the proposal for that height
                let Some(proposal) = self.cache.get(height) else {
                    error!(
                        "Built solutions for a proposal that we don't have in our cache: {}",
                        height
                    );
                    return;
                };

                // Match the solutions
                if proposal.solutions != solutions {
                    warn!("Proposal for {} failed validation with non-matching signatures", height);
                    return;
                }
                // Prepare our commit messge
                let commit = self.signer.sign_commit(height, proposal).unwrap(); // I don't think this can actually fail, validate

                // Then, broadcast our Commit message to all local subscribers and the network
                self.network
                    .broadcast_tx(StromMessage::Commit(Box::new(commit.clone())));
                self.broadcast(ConsensusMessage::Commit(Box::new(commit)));
            }
            ConsensusTaskResult::BuiltProposal(p) => {
                self.broadcast_proposal(p);
            }
        }
    }

    async fn on_new_globalstate(&mut self, new_state: ConsensusState) {
        // First let's update our global state
        match self.globalstate.lock() {
            Ok(mut lock) => {
                lock.set_state(new_state);
            }
            Err(_) => {
                tracing::error!("Global state mutex poisoned!");
            }
        }
        // Depending on what state we moved into, we might have some work to do
        match new_state {
            // Send out our preproposal if we entered PreProposal state
            ConsensusState::PreProposal => self.send_preproposal(),
            // Send out our Proposal if we're the leader and we entered Proposal state
            ConsensusState::Proposal => {
                if self.roundstate.is_leader() {
                    // This now sits and waits on a pretty long computational task - do we want the
                    // Consensus Manager thread to be effectively blocked on awaiting this or do we
                    // want to spawn it and figure out how to sync it back up here presuming it
                    // completes in time?
                    self.start_build_proposal()
                }
            }
            _ => ()
        }
    }

    async fn on_blockchain_state(&mut self, notification: CanonStateNotification) {
        // Get the newest block height
        let new_block = notification.tip();
        let new_block_height = new_block.block.number;

        if self.roundstate.current_height() + 1 == new_block_height {
            // We should immediately start a new round and drop our current round
            let new_leader = self.roundrobin.on_new_block(&new_block.block);
            // TODO:  Figure out the best way to get our node count
            let new_round_state =
                RoundState::new(new_block_height, 1, new_leader, Some(self.roundstate.timings()));
            // Update our round state to the new round state
            self.roundstate = new_round_state;
            // Update the global state to show that we're back in OrderAccumulation
            self.on_new_globalstate(ConsensusState::OrderAccumulation)
                .await;
            // Broadcast that we have a new block
            self.broadcast(ConsensusMessage::NewBlock(new_block_height));
        } else {
            // We should figure out what to do here....while we're testing let's make this a
            // panic
            tracing::error!("Block height is not sequential, this breaks round robin!");
            panic!("Unrecoverable consensus error - Block height not sequential");
        }
    }

    fn on_network_event(&mut self, event: StromConsensusEvent) {
        match event {
            // A PrePropose message is a set of orders from a node, signed by that node.  If
            // we're the leader, we should be holding on to it because we need to collect at
            // least 2/3rds of these in order to make a Proposal.  If we're not the leader,
            // do we need to hold onto this?
            StromConsensusEvent::PrePropose(peer, pre_proposal) => {
                // When I get a prepropose from another peer we should do a few things
                // Validate the signature for the prepropose
                // Validate that the prepropose is for the current block and we're in prepropose
                // state Store the prepropose as part of the current round state
                // Rebroadcast the prepropose to all other hosts?  Maybe...very chatty.
                // We should now check to see if we have gotten enough pre-proposals to step
                // forward
                let _ = self.roundstate.on_pre_propose(peer, pre_proposal.clone());
                // Alert all subscribers to our preproposal received
                self.broadcast(ConsensusMessage::PrePropose(pre_proposal))
            }
            // A Proposal is a backwards-looking data packet that contains all the information
            // needed to validate the actions of the leader in a previous round.  We should
            // validate this information and send out a commit ASAP when we can.
            StromConsensusEvent::Propose(_peer, proposal) => {
                // Put the proposal in the cache
                self.cache.set(proposal.ethereum_height, proposal.clone());

                // Start a verification task
                if !self.start_verify_proposal(&proposal) {
                    warn!("Proposal failed verification with invalid signatures");
                }
            }
            // A Commit message is the validation of a previous proposal by another host.  Right
            // now I'm not sure what we should do with this but overall SOMEONE has to collect
            // these for slashing purposes, no?
            StromConsensusEvent::Commit(peer, mut commit) => {
                // Validate the commit itself - not currently checked
                commit.validate(&[]);
                if commit.signed_by(self.signer.validator_id) {
                    // Do nothing I suppose, we're good.  Maybe we want to
                    // rebroadcast?
                } else {
                    // Add our signature to the commit and broadcast
                    commit.add_signature(self.signer.validator_id, &self.signer.bls_key);
                    // Broadcast to our local subscribers
                    self.broadcast(ConsensusMessage::Commit(commit.clone()));
                    // Also broadcast to the Strom network
                    self.network
                        .broadcast_tx(angstrom_network::StromMessage::Commit(commit.clone()));
                    // Store it locally as a commit we've seen
                    let _ = self.roundstate.on_commit(peer, *commit.clone());
                    // Broadcast to all our internal subscribers
                    self.broadcast(ConsensusMessage::Commit(commit))
                }
            }
        }
    }

    fn broadcast(&mut self, msg: ConsensusMessage) {
        self.subscribers
            .retain_mut(|sub| sub.try_send(msg.clone()).is_ok());
    }
}

pub enum ConsensusCommand {
    Subscribe(Sender<ConsensusMessage>)
}

#[derive(Debug, Clone)]
pub struct ConsensusHandle {
    pub sender: Sender<ConsensusCommand>
}

impl ConsensusUpdater for ConsensusHandle {
    fn new_block(&self, _block: ()) {
        todo!()
    }
}

impl ConsensusListener for ConsensusHandle {
    fn subscribe_messages(&self) -> Pin<Box<dyn Stream<Item = ConsensusMessage>>> {
        let (tx, rx) = channel(10);
        let _ = self.sender.try_send(ConsensusCommand::Subscribe(tx));

        Box::pin(ReceiverStream::new(rx))
    }
}
