use std::{
    future::Future,
    ops::DerefMut,
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    sol_bindings::grouped_orders::AllOrders
};
use futures::Stream;
use reth_rpc_types::PeerId;
use tokio::{
    sync,
    time::{
        Instant, {self}
    }
};

#[derive(Debug, Clone)]
pub(crate) enum DataMsg {
    Order(AllOrders),
    PreProposal(PeerId, PreProposal),
    Proposal(PeerId, Proposal),
    Commit(PeerId, Commit)
}

#[derive(Clone)]
pub enum ConsensusRoundState {
    OrderAccumulator { block_height: BlockNumber, orders: Vec<AllOrders> },
    PrePropose { block_height: BlockNumber, pre_proposal: Vec<PreProposal> } /* Propose,
                                                                              * Commit */
}

impl ConsensusRoundState {
    fn duration(&self) -> Duration {
        match self {
            ConsensusRoundState::OrderAccumulator { .. } => Duration::from_secs(6),
            ConsensusRoundState::PrePropose { .. } => Duration::from_secs(3)
        }
    }

    fn on_data(&mut self, data_msg: DataMsg) {
        match self {
            ConsensusRoundState::OrderAccumulator { orders, .. } => {
                if let DataMsg::Order(order_msg) = data_msg {
                    orders.push(order_msg);
                }
            }
            ConsensusRoundState::PrePropose { .. } => {
                // Handle PreProposal data
            }
        }
    }
}

pub struct RoundState {
    pub current_state: ConsensusRoundState,
    pub timer:         Pin<Box<time::Sleep>>,
    pub data_rx:       sync::mpsc::Receiver<DataMsg>,
    command_tx:        sync::mpsc::Sender<TransitionCommand>,
    command_rx:        sync::mpsc::Receiver<TransitionCommand>
}

impl RoundState {
    pub fn new(initial_state: ConsensusRoundState, data_rx: sync::mpsc::Receiver<DataMsg>) -> Self {
        let duration = initial_state.duration();
        let timer = Box::pin(time::sleep(duration));
        let (tx, rx) = sync::mpsc::channel::<TransitionCommand>(100);
        RoundState { current_state: initial_state, timer, data_rx, command_tx: tx, command_rx: rx }
    }

    pub fn force_transition(&mut self) {
        match self.current_state {
            ConsensusRoundState::OrderAccumulator { .. } => {}
            ConsensusRoundState::PrePropose { .. } => {}
        }
    }

    pub fn reset(&mut self, block_height: BlockNumber, orders: Vec<AllOrders>) {
        self.current_state = ConsensusRoundState::OrderAccumulator { block_height, orders };
    }

    pub fn transition(&mut self) -> ConsensusRoundState {
        match &self.current_state {
            ConsensusRoundState::OrderAccumulator { .. } => {
                self.current_state =
                    ConsensusRoundState::PrePropose { block_height: 0, pre_proposal: vec![] };
                self.current_state.clone()
            }
            ConsensusRoundState::PrePropose { .. } => {
                self.current_state =
                    ConsensusRoundState::OrderAccumulator { block_height: 0, orders: vec![] };
                self.current_state.clone()
            }
        }
    }

    pub async fn on_transition_command(&self, transition_command: TransitionCommand) {
        if let Err(e) = self.command_tx.send(transition_command).await {
            eprintln!("Failed to send transition command: {}", e);
        }
    }
}

enum TransitionCommand {
    OrderAccumulation,
    PrePropose,
    Proposal,
    Commit
}

impl Stream for RoundState {
    type Item = ConsensusRoundState;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.deref_mut();
        if let Poll::Ready(Some(event)) = Pin::new(&mut this.data_rx).poll_recv(cx) {
            this.current_state.on_data(event);
        }

        if this.timer.as_mut().poll(cx).is_ready() {
            let next_state = this.transition();
            this.timer
                .as_mut()
                .reset(Instant::now() + next_state.duration());
            return Poll::Ready(Some(next_state));
        }

        Poll::Pending
    }
}
