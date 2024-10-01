use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution,
    sol_bindings::grouped_orders::AllOrders
};
use futures::{FutureExt, Stream};
use matching_engine::MatchingManager;
use reth_rpc_types::PeerId;
use tokio::{
    sync,
    time::{
        Instant, {self}
    }
};

use crate::Signer;

#[derive(Debug, Clone)]
pub(crate) enum DataMsg {
    Order(AllOrders),
    PreProposal(PeerId, PreProposal),
    Proposal(PeerId, Proposal),
    Commit(PeerId, Commit)
}

#[derive(Clone, Debug)]
pub enum ConsensusRoundState {
    OrderAccumulator { block_height: BlockNumber, orders: Vec<AllOrders> },
    PrePropose { block_height: BlockNumber, pre_proposals: Vec<PreProposal> },
    Propose { block_height: BlockNumber, proposal: Proposal },
    Commit { block_height: BlockNumber, commits: Vec<Commit> }
}

impl ConsensusRoundState {
    fn duration(&self) -> Duration {
        match self {
            ConsensusRoundState::OrderAccumulator { .. } => Duration::from_secs(6),
            ConsensusRoundState::PrePropose { .. } => Duration::from_secs(3),
            ConsensusRoundState::Propose { .. } => Duration::from_secs(3),
            ConsensusRoundState::Commit { .. } => Duration::from_secs(3)
        }
    }

    fn on_data(&mut self, data_msg: DataMsg) {
        match self {
            ConsensusRoundState::OrderAccumulator { orders, .. } => {
                if let DataMsg::Order(order_msg) = data_msg {
                    orders.push(order_msg);
                }
            }
            ConsensusRoundState::PrePropose { pre_proposals, .. } => {
                if let DataMsg::PreProposal(_, pre_proposal) = data_msg {
                    pre_proposals.push(pre_proposal);
                }
            }
            ConsensusRoundState::Propose { .. } => {
                // Handle Proposal data
            }
            ConsensusRoundState::Commit { commits, .. } => {
                if let DataMsg::Commit(_, commit) = data_msg {
                    commits.push(commit);
                }
            }
        }
    }
}

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

pub struct RoundState {
    pub current_state: ConsensusRoundState,
    pub timer:         Pin<Box<time::Sleep>>,
    pub data_rx:       sync::mpsc::Receiver<DataMsg>,
    transition_future: Option<Pin<Box<dyn Future<Output = ConsensusRoundState> + Send>>>,
    signer:            Signer,
    metrics:           ConsensusMetricsWrapper
}

impl RoundState {
    pub fn new(
        initial_state: ConsensusRoundState,
        data_rx: sync::mpsc::Receiver<DataMsg>,
        signer: Signer,
        metrics: ConsensusMetricsWrapper
    ) -> Self {
        let duration = initial_state.duration();
        let timer = Box::pin(time::sleep(duration));
        RoundState {
            current_state: initial_state,
            timer,
            data_rx,
            transition_future: None,
            // command_tx: tx,
            // command_rx: rx,
            // tasks,
            signer,
            metrics
        }
    }

    pub fn force_transition(&mut self, new_state: ConsensusRoundState) {
        match (&self.current_state, &new_state) {
            (
                ConsensusRoundState::OrderAccumulator { .. },
                ConsensusRoundState::PrePropose { .. }
            ) => {
                self.current_state = new_state;
            }
            (ConsensusRoundState::PrePropose { .. }, ConsensusRoundState::Propose { .. }) => {
                self.current_state = new_state;
            }
            (ConsensusRoundState::Propose { .. }, ConsensusRoundState::Commit { .. }) => {
                self.current_state = new_state;
            }
            (ConsensusRoundState::Commit { .. }, ConsensusRoundState::OrderAccumulator { .. }) => {
                self.current_state = new_state;
            }
            _ => {
                tracing::error!(
                    "Invalid state transition from {:?} to {:?}",
                    self.current_state,
                    new_state
                );
            }
        }
    }

    async fn transition(
        current_state: ConsensusRoundState,
        signer: Signer,
        _metrics: ConsensusMetricsWrapper
    ) -> ConsensusRoundState {
        match current_state {
            ConsensusRoundState::OrderAccumulator { block_height, .. } => {
                ConsensusRoundState::PrePropose { block_height, pre_proposals: vec![] }
            }
            ConsensusRoundState::PrePropose { block_height, pre_proposals } => {
                let solutions = build_proposal(pre_proposals.clone()).await.unwrap();
                let proposal = signer.sign_proposal(block_height, pre_proposals.clone(), solutions);
                ConsensusRoundState::Propose { block_height, proposal }
            }
            ConsensusRoundState::Propose { block_height, proposal } => {
                let pre_proposals = proposal.preproposals().clone();
                let height = proposal.ethereum_height;
                let solutions = build_proposal(pre_proposals.clone()).await.unwrap();

                if proposal.solutions != solutions {
                    tracing::warn!(
                        "Proposal for {} failed validation with non-matching signatures",
                        height
                    );
                }

                let commit = signer.sign_commit(&proposal);
                ConsensusRoundState::Commit { block_height, commits: vec![commit] }
            }
            ConsensusRoundState::Commit { block_height, .. } => {
                ConsensusRoundState::OrderAccumulator {
                    block_height: block_height + 1,
                    orders:       vec![]
                }
            }
        }
    }
}

impl Stream for RoundState {
    type Item = ConsensusRoundState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if let Poll::Ready(Some(event)) = Pin::new(&mut this.data_rx).poll_recv(cx) {
            this.current_state.on_data(event);
        }

        if this.timer.as_mut().poll(cx).is_ready() {
            if this.transition_future.is_none() {
                let future = RoundState::transition(
                    this.current_state.clone(),
                    this.signer.clone(),
                    this.metrics.clone()
                )
                .boxed();
                this.transition_future = Some(future);
            }

            if let Some(ref mut future) = this.transition_future {
                match future.as_mut().poll(cx) {
                    Poll::Ready(next_state) => {
                        this.current_state = next_state.clone();
                        this.timer
                            .as_mut()
                            .reset(Instant::now() + next_state.duration());
                        this.transition_future = None;
                        return Poll::Ready(Some(next_state));
                    }
                    Poll::Pending => {
                        return Poll::Pending;
                    }
                }
            }
        }

        Poll::Pending
    }
}
