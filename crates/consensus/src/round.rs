use crate::Signer;
use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution,
    sol_bindings::grouped_orders::AllOrders
};
use futures::{FutureExt, Stream};
use matching_engine::MatchingManager;
use order_pool::order_storage::OrderStorage;
use reth_rpc_types::PeerId;
use std::sync::Arc;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration
};
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

#[derive(Clone, Debug)]
pub enum ConsensusRoundState {
    OrderAccumulator { block_height: BlockNumber, orders: Vec<AllOrders> },
    PrePropose { block_height: BlockNumber, pre_proposals: Vec<PreProposal> },
    Propose { block_height: BlockNumber, proposal: Proposal },
    Commit { block_height: BlockNumber, commits: Vec<Commit> }
}

impl ConsensusRoundState {

    fn as_key(&self) -> &'static str {
        match self {
            ConsensusRoundState::OrderAccumulator { .. } => "OrderAccumulator",
            ConsensusRoundState::PrePropose { .. } => "PrePropose",
            ConsensusRoundState::Propose { .. } => "Propose",
            ConsensusRoundState::Commit { .. } => "Commit",
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
    transition_future: Option<Pin<Box<dyn Future<Output = ConsensusRoundState> + Send>>>,
    signer:            Signer,
    order_storage: Arc<OrderStorage>,
    metrics:           ConsensusMetricsWrapper,
    durations:         HashMap<String, Duration>
}

impl RoundState {
    pub fn new(
        initial_state: ConsensusRoundState,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        metrics: ConsensusMetricsWrapper,
        durations: Option<HashMap<String, Duration>>
    ) -> Self {
        let default_durations = HashMap::from([
            (String::from("OrderAccumulator"), Duration::from_secs(6)),
            (String::from("PrePropose"), Duration::from_secs(3)),
            (String::from("Propose"), Duration::from_secs(3)),
            (String::from("Commit"), Duration::from_secs(3)),
        ]);

        let durations = durations.unwrap_or(default_durations);

        let initial_state_duration = durations.get(initial_state.as_key()).unwrap();
        let timer = Box::pin(time::sleep(*initial_state_duration));
        
        RoundState {
            current_state: initial_state,
            timer,
            transition_future: None,
            order_storage,
            signer,
            metrics,
            durations
        }
    }

    pub fn on_data(&mut self, data_msg: DataMsg) {
        match &mut self.current_state {
            ConsensusRoundState::OrderAccumulator { ref mut orders, .. } => {
                if let DataMsg::Order(order_msg) = data_msg {
                    orders.push(order_msg);
                }
            }
            ConsensusRoundState::PrePropose { ref mut pre_proposals, .. } => {
                if let DataMsg::PreProposal(_, pre_proposal) = data_msg {
                    pre_proposals.push(pre_proposal);
                }
            }
            ConsensusRoundState::Propose { .. } => {
                // Handle Proposal data
            }
            ConsensusRoundState::Commit { ref mut commits, .. } => {
                if let DataMsg::Commit(_, commit) = data_msg {
                    commits.push(commit);
                }
            }
        }
    }

    pub fn duration(&self, state: &ConsensusRoundState) -> Duration {
        // self.state_transition
        //     .force_transition(ConsensusRoundState::Propose {
        //         block_height: self.current_height,
        //         proposal
        //     });

        self.durations.get(state.as_key()).cloned().unwrap()
    }


    pub async fn force_transition(&mut self, new_state: ConsensusRoundState) {
        let new_state = match (&self.current_state, &new_state) {
            (
                ConsensusRoundState::OrderAccumulator { block_height, .. },
                ConsensusRoundState::PrePropose { .. }
            ) => {
                let orders = self.order_storage.get_all_orders();
                let pre_proposal = PreProposal::new(
                    *block_height,
                    &self.signer.key,
                    alloy_primitives::FixedBytes::default(),
                    orders
                );
                ConsensusRoundState::PrePropose { block_height: *block_height, pre_proposals: vec![pre_proposal] }
            }
            (ConsensusRoundState::PrePropose { block_height, pre_proposals, .. }, ConsensusRoundState::Propose { .. }) => {
                let solutions = build_proposal(pre_proposals.clone()).await.unwrap();
                let proposal = self.signer.sign_proposal(*block_height, pre_proposals.clone(), solutions);
                ConsensusRoundState::Propose { block_height:  *block_height, proposal }
            }
            (ConsensusRoundState::Propose { block_height,proposal,..}, ConsensusRoundState::Commit { .. }) => {
                let pre_proposals = proposal.preproposals().clone();
                let height = proposal.ethereum_height;
                let solutions = build_proposal(pre_proposals.clone()).await.unwrap();

                if proposal.solutions != solutions {
                    tracing::warn!(
                        "Proposal for {} failed validation with non-matching signatures",
                        height
                    );
                }

                let commit = self.signer.sign_commit(&proposal);

                ConsensusRoundState::Commit { block_height: *block_height, commits: vec![commit] }
            }
            (ConsensusRoundState::Commit { .. }, ConsensusRoundState::OrderAccumulator { block_height, .. }) => {
                ConsensusRoundState::OrderAccumulator {
                    block_height: *block_height,
                    orders:       vec![]
                }
            }
            _ => {
                tracing::error!(
                    "Invalid state transition from {:?} to {:?}",
                    self.current_state,
                    new_state
                );
                // TODO: panic?
                panic!("Invalid state transition from {:?} to {:?}", self.current_state, new_state);
            }
        };

        self.reset_timer(new_state)
    }
    fn reset_timer(&mut self, new_state: ConsensusRoundState) {
        let duration = self.duration(&new_state);
        self.current_state = new_state;
        self.timer
            .as_mut()
            .reset(Instant::now() + duration);
        self.transition_future = None;
    }

    async fn transition(
        current_state: ConsensusRoundState,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        _metrics: ConsensusMetricsWrapper
    ) -> ConsensusRoundState {
        match current_state {
            ConsensusRoundState::OrderAccumulator { block_height, .. } => {
                let orders = order_storage.get_all_orders();
                    let pre_proposal = PreProposal::new(
                        block_height,
                        &signer.key,
                        alloy_primitives::FixedBytes::default(),
                        orders
                    );
                ConsensusRoundState::PrePropose { block_height, pre_proposals: vec![pre_proposal] }
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

        // if let Poll::Ready(Some(event)) = Pin::new(&mut this.data_rx).poll_recv(cx) {
        //     this.current_state.on_data(event);
        // }

        if let Some(ref mut future) = this.transition_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(new_state) => {
                    this.reset_timer(new_state.clone());
                    return Poll::Ready(Some(new_state));
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }

        if this.timer.as_mut().poll(cx).is_ready() {
            if this.transition_future.is_none() {
                let future = RoundState::transition(
                    this.current_state.clone(),
                    this.order_storage.clone(),
                    this.signer.clone(),
                    this.metrics.clone()
                )
                .boxed();
                this.transition_future = Some(future);
            }
        }

        Poll::Pending
    }
}
