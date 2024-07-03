use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll}
};

use angstrom_network::{manager::StromConsensusEvent, StromNetworkHandle};
use futures::{Future, FutureExt, Stream, StreamExt};
use order_pool::{AtomicConsensus, OrderPoolHandle};
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_provider::CanonStateNotifications;
use reth_tasks::TaskSpawner;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use validation::BundleValidator;

use crate::{
    core::ConsensusMessage,
    global::GlobalConsensusState,
    round::{Leader, RoundState},
    signer::Signer,
    ConsensusListener, ConsensusUpdater
};

#[allow(unused)]
pub struct ConsensusManager<OrderPool, Validator> {
    // core: ConsensusCore,
    /// keeps track of the current round state
    roundstate:             RoundState,
    globalstate:            Arc<Mutex<GlobalConsensusState>>,
    command:                ReceiverStream<ConsensusCommand>,
    subscribers:            Vec<Sender<ConsensusMessage>>,
    /// Used to trigger new consensus rounds
    canonical_block_stream: CanonStateNotifications,
    /// events from the network,
    strom_consensus_event:  UnboundedMeteredReceiver<StromConsensusEvent>,
    network:                StromNetworkHandle,

    signer:    Signer,
    orderpool: OrderPool,
    validator: Validator
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

impl<OrderPool, Validator> ConsensusManager<OrderPool, Validator>
where
    OrderPool: OrderPoolHandle,
    Validator: BundleValidator
{
    pub fn spawn<TP: TaskSpawner>(
        tp: TP,
        globalstate: Arc<Mutex<GlobalConsensusState>>,
        netdeps: ManagerNetworkDeps,
        signer: Signer,
        orderpool: OrderPool,
        validator: Validator
    ) -> ConsensusHandle {
        let ManagerNetworkDeps { network, canonical_block_stream, strom_consensus_event, tx, rx } =
            netdeps;
        let stream = ReceiverStream::new(rx);

        // This is still a lot of stuff to track that we don't necessarily have to worry
        // about
        let roundstate = RoundState::new(0, Leader::default());

        let this = Self {
            strom_consensus_event,
            validator,
            roundstate,
            globalstate,
            subscribers: Vec::new(),
            command: stream,
            network,
            signer,
            orderpool,
            canonical_block_stream
        };

        tp.spawn_critical("consensus", this.boxed());

        ConsensusHandle { sender: tx }
    }

    fn on_command(&mut self, command: ConsensusCommand) {
        match command {
            ConsensusCommand::Subscribe(sender) => self.subscribers.push(sender)
        }
    }

    fn on_consensus_event(&mut self, command: StromConsensusEvent) {
        match command {
            // A PrePropose message is a set of orders from a node, signed by that node.  If
            // we're the leader, we should be holding on to it because we need to collect at
            // least 2/3rds of these in order to make a Proposal.  If we're not the leader,
            // do we need to hold onto this?
            StromConsensusEvent::PrePropose(..) => todo!(),
            // A Proposal is a backwards-looking data packet that contains all the information
            // needed to validate the actions of the leader in a previous round.  We should
            // validate this information and send out a commit ASAP when we can.
            StromConsensusEvent::Propose(_peer, proposal) => {
                // Validate the proposal itself - not currently checked
                proposal.validate(&[]);
                // Validate that the proposal contains the data we also think is the best
                // Also skipping this for now - maybe do this after we check proposal timing?

                // Otherwise prepare our commit message
                let commit = self
                    .signer
                    .sign_commit(proposal.ethereum_block, &proposal)
                    .unwrap(); // I don't think this can actually fail, validate

                // Broadcast our Commit message to everyone
                self.broadcast(ConsensusMessage::Commit(Box::new(commit)));
            }
            // A Commit message is the validation of a previous proposal by another host.  Right
            // now I'm not sure what we should do with this but overall SOMEONE has to collect
            // these for slashing purposes, no?
            StromConsensusEvent::Commit(_, mut commit) => {
                // Validate the commit itself - not currently checked
                commit.validate(&[]);
                if commit.signed_by(self.signer.validator_id) {
                    // Do nothing I suppose, we're good.  Maybe we want to
                    // rebroadcast?
                } else {
                    // Add our signature to the commit and broadcast
                    commit.add_signature(self.signer.validator_id, &self.signer.key);
                    // Broadcast to our local subscribers
                    self.broadcast(ConsensusMessage::Commit(commit.clone()));
                    // Also broadcast to the Strom network
                    self.network
                        .broadcast_tx(angstrom_network::StromMessage::Commit(commit))
                }
            }
        }
    }

    fn broadcast(&mut self, msg: ConsensusMessage) {
        self.subscribers
            .retain_mut(|sub| sub.try_send(msg.clone()).is_ok());
    }
}

impl<OrderPool, Validator> Future for ConsensusManager<OrderPool, Validator>
where
    OrderPool: OrderPoolHandle,
    Validator: BundleValidator
{
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut work = 10;

        loop {
            while let Poll::Ready(Some(msg)) = self.command.poll_next_unpin(cx) {
                self.on_command(msg)
            }

            // We're handing one event per cycle instead of all events per cycle - might
            // want to change this depending on the priorty level of these
            // events compared to Command events
            if let Poll::Ready(Some(msg)) = self.strom_consensus_event.poll_next_unpin(cx) {
                self.on_consensus_event(msg);
            }

            // Check our core state to see if we have any state changes or other updates to
            // worry about - we're not going to do this for the moment
            // if let Poll::Ready(msg) =
            // self.roundstate.poll_next_unpin(cx).filter_map(|item| item) {
            //     self.broadcast(msg.clone());
            // }

            work -= 1;
            if work == 0 {
                cx.waker().wake_by_ref();
                return Poll::Pending
            }
        }
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
