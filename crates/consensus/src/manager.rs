use std::{
    pin::Pin,
    sync::{Arc, Mutex}
};

use angstrom_network::{manager::StromConsensusEvent, StromMessage, StromNetworkHandle};
use angstrom_types::consensus::PreProposal;
use futures::{FutureExt, Stream, StreamExt};
use matching_engine::MatchingEngineHandle;
use order_pool::order_storage::OrderStorage;
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_provider::{CanonStateNotification, CanonStateNotifications};
use reth_tasks::TaskSpawner;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender}
};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};
use validation::BundleValidator;

use crate::{
    global::GlobalConsensusState,
    round::{Leader, RoundState},
    round_robin_algo::RoundRobinAlgo,
    signer::Signer,
    ConsensusListener, ConsensusMessage, ConsensusState, ConsensusUpdater
};

pub async fn manager_thread<Matcher, Validator>(
    globalstate: Arc<Mutex<GlobalConsensusState>>,
    netdeps: ManagerNetworkDeps,
    signer: Signer,
    order_storage: Arc<OrderStorage>,
    matcher: Matcher,
    validator: Validator
) where
    Matcher: MatchingEngineHandle,
    Validator: BundleValidator
{
    let ManagerNetworkDeps { network, canonical_block_stream, strom_consensus_event, rx, .. } =
        netdeps;
    let stream = ReceiverStream::new(rx);

    // This is still a lot of stuff to track that we don't necessarily have to worry
    // about
    let roundstate = RoundState::new(0, 1, Leader::default());

    let wrapped_broadcast_stream = BroadcastStream::new(canonical_block_stream);
    // Use our default round robin algo
    let (roundrobin, _cacheheight) = RoundRobinAlgo::new();

    let mut manager = ConsensusManager {
        strom_consensus_event,
        validator,
        roundstate,
        globalstate,
        subscribers: Vec::new(),
        command: stream,
        network,
        roundrobin,
        signer,
        order_storage,
        matcher,
        canonical_block_stream: wrapped_broadcast_stream
    };

    // Start message loop
    loop {
        select! {
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

#[allow(unused)]
pub struct ConsensusManager<Matcher, Validator> {
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
    matcher:       Matcher,
    validator:     Validator
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

impl<Matcher, Validator> ConsensusManager<Matcher, Validator>
where
    Matcher: MatchingEngineHandle,
    Validator: BundleValidator
{
    pub fn spawn<TP: TaskSpawner>(
        tp: TP,
        globalstate: Arc<Mutex<GlobalConsensusState>>,
        netdeps: ManagerNetworkDeps,
        signer: Signer,
        order_storage: Arc<OrderStorage>,
        matcher: Matcher,
        validator: Validator
    ) -> ConsensusHandle {
        let tx = netdeps.tx.clone();
        let fut =
            manager_thread(globalstate, netdeps, signer, order_storage, matcher, validator).boxed();

        tp.spawn_critical("consensus", fut);

        ConsensusHandle { sender: tx }
    }

    fn send_preproposal(&mut self) {
        let orders = self.order_storage.get_all_orders();
        let preproposal =
            PreProposal::new(0, &self.signer.key, alloy_primitives::FixedBytes::default(), orders);
        tracing::info!("Sending out preproposal");
        self.network
            .broadcast_tx(StromMessage::PrePropose(preproposal.clone()));
        self.broadcast(ConsensusMessage::PrePropose(preproposal));
    }

    async fn send_proposal(&mut self) {
        let preproposals = self.roundstate.get_preproposals();
        let solutions = self
            .matcher
            .solve_pools(preproposals.clone())
            .await
            .unwrap();
        let proposal = self
            .signer
            .sign_proposal(self.roundstate.current_height(), preproposals, solutions)
            .unwrap();
        tracing::info!("Sending out proposal");
        self.network
            .broadcast_tx(StromMessage::Propose(proposal.clone()));
        self.broadcast(ConsensusMessage::Proposal(proposal));
    }

    fn on_command(&mut self, command: ConsensusCommand) {
        match command {
            ConsensusCommand::Subscribe(sender) => self.subscribers.push(sender)
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
                    self.send_proposal().await
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
            let new_round_state = RoundState::new(new_block_height, 1, new_leader);
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
                // When I get a Proposal, I should send it off to the proposal validation
                // pipeline

                // Validate the proposal itself - not currently checked
                proposal.validate(&[]);

                // Given that the proposal has passed validation, prepare our commit message
                let commit = self
                    .signer
                    .sign_commit(proposal.ethereum_height, &proposal)
                    .unwrap(); // I don't think this can actually fail, validate

                // Store the current proposal in our Round State as the Proposal for our round
                // I probably don't have to do this in the future because Proposal validation is
                // going to be backwards-looking
                let _ = self.roundstate.on_proposal(proposal.clone());
                // Update local subscribers to the fact that a proposal was received
                self.broadcast(ConsensusMessage::Proposal(proposal));
                // Then, broadcast our Commit message to all local subscribers and the network
                self.network
                    .broadcast_tx(StromMessage::Commit(Box::new(commit.clone())));
                self.broadcast(ConsensusMessage::Commit(Box::new(commit)));
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
