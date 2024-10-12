use std::{
    collections::{HashMap, HashSet},
    default::Default,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::{OrderSet, PoolSolution},
    primitive::Signature,
    sol_bindings::{
        grouped_orders::{
            AllOrders, GroupedComposableOrder, GroupedUserOrder, GroupedVanillaOrder,
            OrderWithStorageData
        },
        rpc_orders::TopOfBlockOrder,
        AngstromContract::saveNodeFeeReturn
    }
};
use blsful::vsss_rs::elliptic_curve::rand_core::block;
use futures::{FutureExt, Stream, StreamExt};
use matching_engine::MatchingManager;
use order_pool::order_storage::OrderStorage;
use reth_rpc_types::{mev::BundleItem::Hash, PeerId};
use serde::{Deserialize, Serialize};
use tokio::{
    sync,
    time::{self, Instant}
};
use validation::validator::Validator;

use crate::{AngstromValidator, Signer};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BidSubmission {
    pub block_height:  BlockNumber,
    // this is used mostly for early messages
    pub pre_proposals: HashSet<PreProposal>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BidAggregation {
    pub block_height:  BlockNumber,
    pub pre_proposals: HashSet<PreProposal>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Finalization {
    pub block_height:  BlockNumber,
    pub pre_proposals: HashSet<PreProposal>,
    pub proposal:      Option<Proposal>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusState {
    BidSubmission(BidSubmission),
    BidAggregation(BidAggregation),
    Finalization(Finalization)
}

impl ConsensusState {
    fn as_key(&self) -> &'static str {
        match self {
            Self::BidSubmission(_) => "BidSubmission",
            Self::BidAggregation(_) => "BidAggregation",
            Self::Finalization(_) => "Finalization"
        }
    }
}

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

pub struct RoundStateMachine {
    current_state:     ConsensusState,
    timer:             Pin<Box<time::Sleep>>,
    transition_future: Option<Pin<Box<dyn Future<Output = ConsensusState> + Send>>>,
    signer:            Signer,
    round_leader:      PeerId,
    validators:        Vec<AngstromValidator>,
    order_storage:     Arc<OrderStorage>,
    metrics:           ConsensusMetricsWrapper,
    durations:         HashMap<String, Duration>,
    waker:             Option<Waker>
}

impl RoundStateMachine {
    pub fn new(
        block_height: BlockNumber,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        round_leader: PeerId,
        validators: Vec<AngstromValidator>,
        metrics: ConsensusMetricsWrapper,
        durations: Option<HashMap<String, Duration>>
    ) -> Self {
        let default_durations = HashMap::from([
            (
                ConsensusState::BidSubmission(BidSubmission::default())
                    .as_key()
                    .to_string(),
                Duration::from_secs(3)
            ),
            (
                ConsensusState::BidAggregation(BidAggregation::default())
                    .as_key()
                    .to_string(),
                Duration::from_secs(6)
            ),
            (
                ConsensusState::Finalization(Finalization::default())
                    .as_key()
                    .to_string(),
                Duration::from_secs(9)
            )
        ]);

        let durations = durations.unwrap_or(default_durations);

        let current_state = Self::initial_state(block_height);
        let initial_state_duration = durations.get(current_state.as_key()).unwrap().clone();
        let timer = Box::pin(time::sleep(initial_state_duration));

        Self {
            current_state,
            round_leader,
            validators,
            timer,
            order_storage,
            signer,
            metrics,
            durations,
            transition_future: None,
            waker: None
        }
    }

    pub fn my_id(&self) -> PeerId {
        self.signer.my_id
    }

    pub fn is_leader(&self, node: PeerId) -> bool {
        self.round_leader == node
    }

    pub fn i_am_leader(&self) -> bool {
        self.is_leader(self.my_id())
    }

    pub fn has_quorum(&self, voters: usize) -> bool {
        voters >= (self.validators.len() * 2) / 3 + 1
    }

    pub fn reset_state(&mut self, block: BlockNumber, leader: PeerId) {
        self.round_leader = leader;
        self.reset_timer(Self::initial_state(block));
    }

    pub fn initial_state(block_height: BlockNumber) -> ConsensusState {
        ConsensusState::BidSubmission(BidSubmission { block_height, pre_proposals: HashSet::new() })
    }

    /// Invariants:
    ///  * won't be called with messages from ourselves
    ///  * won't be called with messages for non-current block height
    pub fn on_strom_message(&mut self, strom_msg: StromConsensusEvent) {
        let current_state = self.current_state.clone();
        let new_state: Option<ConsensusState> = match current_state {
            ConsensusState::BidSubmission(bid_submission) => {
                let (updated_submission, new_state) =
                    self.handle_in_bid_submission(bid_submission, strom_msg);
                self.current_state = ConsensusState::BidSubmission(updated_submission);
                new_state
            }
            ConsensusState::BidAggregation(bid_aggregation) => {
                let (updated_aggregation, new_state) =
                    self.handle_in_bid_aggregation(bid_aggregation, strom_msg);
                self.current_state = ConsensusState::BidAggregation(updated_aggregation);
                new_state
            }
            ConsensusState::Finalization(finalization) => {
                let (updated_finalization, new_state) =
                    self.handle_in_finalization(finalization, strom_msg);
                self.current_state = ConsensusState::Finalization(updated_finalization);
                new_state
            }
        };

        if let Some(new_state) = new_state {
            self.force_transition(new_state);
        }
    }

    fn handle_in_bid_submission(
        &self,
        mut bid_submission: BidSubmission,
        strom_msg: StromConsensusEvent
    ) -> (BidSubmission, Option<ConsensusState>) {
        match strom_msg {
            // we are lagging, we should transition
            StromConsensusEvent::PreProposal(msg_sender, pre_proposal) => {
                let PreProposal {
                    block_height: pre_proposal_height,
                    source: pre_proposal_sender,
                    ..
                } = pre_proposal;
                if !pre_proposal.is_valid() {
                    tracing::warn!(
                        %msg_sender,
                        %bid_submission.block_height,
                        %pre_proposal_sender,
                        %pre_proposal_height,
                        "received pre_proposal with invalid signature",
                    );
                    return (bid_submission, None);
                }
                bid_submission.pre_proposals.insert(pre_proposal.clone());

                let merged_limit_orders = self.merge_limit_orders(
                    bid_submission
                        .pre_proposals
                        .iter()
                        .flat_map(|p| p.limit.clone())
                        .collect()
                );

                let merged_searcher_orders = self.merge_searcher_orders(
                    bid_submission
                        .pre_proposals
                        .iter()
                        .flat_map(|p| p.searcher.clone())
                        .collect()
                );

                let pre_proposal = PreProposal::generate_pre_proposal(
                    pre_proposal_height,
                    self.my_id(),
                    merged_limit_orders,
                    merged_searcher_orders,
                    &self.signer.key
                );
                bid_submission.pre_proposals.insert(pre_proposal);

                // TODO: do we want to skip bid aggregation if we have quorum here ?
                (
                    bid_submission.clone(),
                    Some(ConsensusState::BidAggregation(BidAggregation {
                        block_height:  pre_proposal_height,
                        pre_proposals: bid_submission.pre_proposals.clone()
                    }))
                )
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    block_height: proposal_block_height, source: proposal_sender, ..
                } = proposal;
                // TODO: we got an invalid proposal; start slashing ?
                if !proposal.is_valid() || !self.is_leader(proposal_sender) {
                    return (bid_submission, None)
                }

                // we are following and lagging a lot
                if !self.i_am_leader() {
                    let pre_proposals = bid_submission.pre_proposals.clone();
                    return (
                        bid_submission,
                        Some(ConsensusState::Finalization(Finalization {
                            block_height: proposal_block_height,
                            proposal: Some(proposal),
                            pre_proposals
                        }))
                    )
                }

                // that should not be possible. it means we got our own proposal from another
                // node, while we are in bid submission state
                panic!(
                    "Message sender: {}, Proposal block height: {}, Proposal source: {}, Current \
                     state {}, something is terribly wrong",
                    msg_sender,
                    proposal_block_height,
                    proposal_sender,
                    self.current_state.as_key()
                );
            }
            // TODO: this could be used  by the leader to gossip the 2/3 + 1 pre_proposals that were
            // used for the bundle
            StromConsensusEvent::Commit(msg_sender, commit) => (bid_submission, None)
        }
    }

    fn handle_in_bid_aggregation(
        &self,
        mut bid_aggregation: BidAggregation,
        strom_msg: StromConsensusEvent
    ) -> (BidAggregation, Option<ConsensusState>) {
        match strom_msg {
            StromConsensusEvent::PreProposal(msg_sender, pre_proposal) => {
                let PreProposal { block_height, source, .. } = pre_proposal;

                if !pre_proposal.is_valid() {
                    return (bid_aggregation, None);
                }

                let merged_limit_orders = self.merge_limit_orders(
                    bid_aggregation
                        .pre_proposals
                        .iter()
                        .flat_map(|p| p.limit.clone())
                        .collect()
                );

                let merged_searcher_orders = self.merge_searcher_orders(
                    bid_aggregation
                        .pre_proposals
                        .iter()
                        .flat_map(|p| p.searcher.clone())
                        .collect()
                );

                let self_proposal = PreProposal {
                    block_height,
                    source: self.my_id(),
                    limit: merged_limit_orders,
                    searcher: merged_searcher_orders,
                    ..Default::default() // Assuming other fields can be defaulted
                };

                bid_aggregation.pre_proposals.insert(self_proposal);

                if self.has_quorum(bid_aggregation.pre_proposals.len()) {
                    return (
                        bid_aggregation.clone(),
                        Some(ConsensusState::Finalization(Finalization {
                            block_height,
                            proposal: None,
                            pre_proposals: bid_aggregation.pre_proposals
                        }))
                    )
                }

                (bid_aggregation, None)
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    block_height: proposal_block_height, source: proposal_sender, ..
                } = proposal;
                // TODO: we got an invalid proposal; start slashing ?
                if !proposal.is_valid() || !self.is_leader(proposal_sender) {
                    return (bid_aggregation, None)
                }

                // we are following and lagging a bit
                if !self.i_am_leader() {
                    let pre_proposals = bid_aggregation.pre_proposals.clone();
                    return (
                        bid_aggregation,
                        Some(ConsensusState::Finalization(Finalization {
                            block_height: proposal_block_height,
                            proposal: Some(proposal),
                            pre_proposals
                        }))
                    )
                }

                // that should not be possible. it means we got our own proposal from another
                // node, while we are in bid aggregation state
                panic!(
                    "Message sender: {}, Proposal block height: {}, Proposal source: {}, Current \
                     state {}, something is terribly wrong",
                    msg_sender,
                    proposal_block_height,
                    proposal_sender,
                    self.current_state.as_key()
                );
            }
            // TODO: used to broadcast after the
            StromConsensusEvent::Commit(msg_sender, ..) => (bid_aggregation, None),
            _ => (bid_aggregation, None)
        }
    }

    fn handle_in_finalization(
        &self,
        mut finalization: Finalization,
        strom_msg: StromConsensusEvent
    ) -> (Finalization, Option<ConsensusState>) {
        match strom_msg {
            // that is a very delayed message; ignore it maybe
            StromConsensusEvent::PreProposal(..) => (finalization, None),
            StromConsensusEvent::Proposal(..) => (finalization, None),
            // TODO: this could be used  by the leader to gossip the 2/3 + 1 pre_proposals that were
            // used for the bundle
            StromConsensusEvent::Commit(msg_sender, commit) => (finalization, None)
        }
    }

    fn merge_limit_orders(
        &self,
        left_limit: Vec<OrderWithStorageData<GroupedVanillaOrder>>
    ) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
        let mut unique_limit_per_pool: HashMap<_, HashSet<GroupedVanillaOrder>> = HashMap::new();

        for order_with_data in left_limit.into_iter() {
            unique_limit_per_pool
                .entry(order_with_data.pool_id)
                .or_insert_with(HashSet::new)
                .insert(order_with_data.order.clone());
        }

        unique_limit_per_pool
            .into_iter()
            .flat_map(|(pool_id, orders)| {
                orders.into_iter().map(move |order| OrderWithStorageData {
                    pool_id,
                    order,
                    ..Default::default() // Assuming other fields can be defaulted
                })
            })
            .collect()
    }

    fn merge_searcher_orders(
        &self,
        left_searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>
    ) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        let mut top_searcher_per_pool: HashMap<_, OrderWithStorageData<TopOfBlockOrder>> =
            HashMap::new();
        for order_with_data in left_searcher.into_iter() {
            let pool_id = order_with_data.pool_id;
            top_searcher_per_pool
                .entry(pool_id)
                .and_modify(|existing_order| {
                    if order_with_data.tob_reward > existing_order.tob_reward {
                        *existing_order = order_with_data.clone();
                    }
                })
                .or_insert(order_with_data);
        }
        top_searcher_per_pool.into_values().collect()
    }

    pub fn duration(&self, state: &ConsensusState) -> Duration {
        self.durations.get(state.as_key()).cloned().unwrap()
    }

    fn reset_timer(&mut self, new_state: ConsensusState) {
        let duration = self.duration(&new_state);
        self.current_state = new_state;
        self.timer.as_mut().reset(Instant::now() + duration);
        self.transition_future = None;
    }

    fn force_transition(&mut self, new_state: ConsensusState) {
        self.transition_future = Some(Box::pin(async { new_state }));

        // wake up the task to ensure poll_next is called
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }

    async fn transition(
        current_state: ConsensusState,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        _metrics: ConsensusMetricsWrapper
    ) -> ConsensusState {
        match current_state {
            ConsensusState::BidSubmission(BidSubmission {
                block_height,
                mut pre_proposals,
                ..
            }) => {
                let OrderSet { limit, searcher } = order_storage.get_all_orders();
                let pre_proposal = PreProposal::generate_pre_proposal(
                    block_height,
                    signer.my_id,
                    limit,
                    searcher,
                    &signer.key
                );
                pre_proposals.insert(pre_proposal);
                ConsensusState::BidAggregation(BidAggregation { block_height, pre_proposals })
            }
            ConsensusState::BidAggregation(BidAggregation { block_height, pre_proposals }) => {
                ConsensusState::Finalization(Finalization {
                    block_height,
                    pre_proposals,
                    proposal: None
                })
            }
            ConsensusState::Finalization(Finalization { block_height, .. }) => {
                ConsensusState::BidSubmission(BidSubmission { block_height, ..Default::default() })
            }
        }
    }
}

impl Stream for RoundStateMachine {
    type Item = ConsensusState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        this.waker = Some(cx.waker().clone());

        if let Some(ref mut future) = this.transition_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(new_state) => {
                    this.reset_timer(new_state.clone());
                    return Poll::Ready(Some(new_state));
                }
                Poll::Pending => return Poll::Pending
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
