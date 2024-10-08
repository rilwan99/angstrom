use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration
};

use crate::Signer;
use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_types::orders::OrderSet;
use angstrom_types::primitive::Signature;
use angstrom_types::sol_bindings::grouped_orders::{GroupedComposableOrder, GroupedUserOrder, GroupedVanillaOrder, OrderWithStorageData};
use angstrom_types::sol_bindings::rpc_orders::TopOfBlockOrder;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::PoolSolution,
    sol_bindings::grouped_orders::AllOrders
};
use futures::{FutureExt, Stream};
use matching_engine::MatchingManager;
use order_pool::order_storage::OrderStorage;
use reth_rpc_types::PeerId;
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusState {
    BidSubmission { block_height: BlockNumber, tob_bids: Vec<OrderWithStorageData<TopOfBlockOrder>>, rob_transactions: Vec<OrderWithStorageData<GroupedVanillaOrder>> },
    BidAggregation { block_height: BlockNumber, tob_bids: Vec<OrderWithStorageData<TopOfBlockOrder>>, rob_transactions: Vec<OrderWithStorageData<GroupedVanillaOrder>> },
    // LeaderAction { block_height: BlockNumber, final_bundle: Proposal },
    // PostConsensusVerification { block_height: BlockNumber, all_bids: Vec<AllOrders>, all_transactions: Vec<AllOrders> }
}

impl ConsensusState {
    fn as_key(&self) -> &'static str {
        match self {
            ConsensusState::BidSubmission { .. } => "BidSubmission",
            ConsensusState::BidAggregation { .. } => "BidAggregation",
            // ConsensusState::LeaderAction { .. } => "LeaderAction",
            // ConsensusState::PostConsensusVerification { .. } => "PostConsensusVerification"
        }
    }
}

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

pub struct RoundStateMachine {
    pub current_state: ConsensusState,
    pub timer:         Pin<Box<time::Sleep>>,
    transition_future: Option<Pin<Box<dyn Future<Output = ConsensusState> + Send>>>,
    signer:            Signer,
    order_storage:     Arc<OrderStorage>,
    metrics:           ConsensusMetricsWrapper,
    durations:         HashMap<String, Duration>
}

impl RoundStateMachine {
    pub fn new(
        initial_state: ConsensusState,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        metrics: ConsensusMetricsWrapper,
        durations: Option<HashMap<String, Duration>>
    ) -> Self {
        let default_durations = HashMap::from([
            (String::from("BidSubmission"), Duration::from_secs(6)),
            (String::from("BidAggregation"), Duration::from_secs(3)),
            (String::from("LeaderAction"), Duration::from_secs(3)),
            (String::from("PostConsensusVerification"), Duration::from_secs(3))
        ]);

        let durations = durations.unwrap_or(default_durations);

        let initial_state_duration = durations.get(initial_state.as_key()).unwrap();
        let timer = Box::pin(time::sleep(*initial_state_duration));

        RoundStateMachine {
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
            ConsensusState::BidSubmission { ref mut tob_bids, ref mut rob_transactions, .. } => {
                if let DataMsg::Order(order_msg) = data_msg {
                    // tob_bids.push(order_msg.clone());
                    // rob_transactions.push(order_msg);
                }
            }
            ConsensusState::BidAggregation { ref mut tob_bids, ref mut rob_transactions, .. } => {
                if let DataMsg::Order(order_msg) = data_msg {
                    // tob_bids.push(order_msg.clone());
                    // rob_transactions.push(order_msg);
                }
            }
            // ConsensusState::LeaderAction { .. } => {
            //     // Handle LeaderAction data
            // }
            // ConsensusState::PostConsensusVerification { ref mut all_bids, ref mut all_transactions, .. } => {
                       //     if let DataMsg::Order(order_msg) = data_msg {
            //         all_bids.push(order_msg.clone());
            //         all_transactions.push(order_msg);
            //     }
            // }
        }
    }

    pub fn duration(&self, state: &ConsensusState) -> Duration {
        self.durations.get(state.as_key()).cloned().unwrap()
    }

    pub async fn force_transition(&mut self, new_state: ConsensusState) {
        let new_state = match (&self.current_state, &new_state) {
            (
                ConsensusState::BidSubmission { block_height, .. },
                ConsensusState::BidAggregation { .. }
            ) => {
                let tob_bids = self.order_storage.get_all_orders();
                let rob_transactions = self.order_storage.get_all_orders();
                ConsensusState::BidAggregation {
                    block_height:  *block_height,
                    tob_bids: Vec::new(),
                    rob_transactions: Vec::new(),
                }
            }
            // (
            //     ConsensusState::BidAggregation { block_height, tob_bids, rob_transactions, .. },
            //     ConsensusState::LeaderAction { .. }
            // ) => {
            //     let highest_tob_bid = tob_bids.iter().max_by_key(|bid| *bid.price).unwrap().clone();
            //     let final_bundle = self.signer.sign_proposal(*block_height, vec![highest_tob_bid.clone()], vec![]);
            //     ConsensusState::LeaderAction { block_height: *block_height, final_bundle }
            // }
            // (
            //     ConsensusState::LeaderAction { block_height, final_bundle, .. },
            //     ConsensusState::PostConsensusVerification { .. }
            // ) => {
            //     let all_bids = self.order_storage.get_all_orders();
            //     let all_transactions = self.order_storage.get_all_orders();
            //     ConsensusState::PostConsensusVerification {
            //         block_height: *block_height,
            //         all_bids: all_bids,
            //         all_transactions: all_transactions
            //     }
            // }
            // (
            //     ConsensusState::PostConsensusVerification { .. },
            //     ConsensusState::BidSubmission { block_height, .. }
            // ) => ConsensusState::BidSubmission {
            //     block_height: *block_height + 1,
            //     tob_bids: vec![],
            //     rob_transactions: vec![]
            // },
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

    fn reset_timer(&mut self, new_state: ConsensusState) {
        let duration = self.duration(&new_state);
        self.current_state = new_state;
        self.timer.as_mut().reset(Instant::now() + duration);
        self.transition_future = None;
    }

    async fn transition(
        current_state: ConsensusState,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        _metrics: ConsensusMetricsWrapper
    ) -> ConsensusState {
        match current_state {
            ConsensusState::BidSubmission { block_height, .. } => {
                let OrderSet { limit, searcher } = order_storage.get_all_orders();
                let rob_transactions = order_storage.get_all_orders();
                ConsensusState::BidAggregation {
                    block_height,
                    tob_bids: searcher,
                    rob_transactions: limit,
                }
            }
            ConsensusState::BidAggregation { block_height, tob_bids, rob_transactions } => {
                // let highest_tob_bid = tob_bids.iter().max_by_key(|bid| bid.price).unwrap().clone();
                // let final_bundle = signer.sign_proposal(block_height, vec![highest_tob_bid.clone()], vec![]);
                ConsensusState::BidSubmission {
                    block_height,
                    tob_bids,
                    rob_transactions,
                }
            }
            // ConsensusState::LeaderAction { block_height, final_bundle } => {
            //     let all_bids = order_storage.get_all_orders();
            //     let all_transactions = order_storage.get_all_orders();
            //     ConsensusState::PostConsensusVerification {
            //         block_height,
            //         all_bids: all_bids,
            //         all_transactions: all_transactions
            //     }
            // }
            // ConsensusState::PostConsensusVerification { block_height, .. } => {
            //     ConsensusState::BidSubmission {
            //         block_height: block_height + 1,
            //         tob_bids: vec![],
            //         rob_transactions: vec![]
            //     }
            // }
        }
    }
}

impl Stream for RoundStateMachine {
    type Item = ConsensusState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

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
                let future = RoundStateMachine::transition(
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
