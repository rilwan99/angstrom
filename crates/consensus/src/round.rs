use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::{manager::StromConsensusEvent, StromMessage};
use angstrom_types::{
    consensus::{PreProposal, Proposal},
    orders::{OrderSet, PoolSolution},
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use futures::{future::BoxFuture, Future, Stream};
use itertools::Itertools;
use matching_engine::MatchingManager;
use order_pool::order_storage::OrderStorage;
use reth_rpc_types::PeerId;
use secp256k1::SecretKey;
use serde::{Deserialize, Serialize};
use tokio::time::{self};

use crate::{AngstromValidator, Signer};

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

const INITIAL_STATE_DURATION: u64 = 3;

pub struct RoundStateMachine {
    current_state:          ConsensusState,
    signer:                 Signer,
    round_leader:           PeerId,
    validators:             Vec<AngstromValidator>,
    order_storage:          Arc<OrderStorage>,
    initial_state_duration: Duration,
    metrics:                ConsensusMetricsWrapper,
    transition_future:      Option<BoxFuture<'static, ConsensusState>>,
    initial_state_timer:    Option<Pin<Box<time::Sleep>>>,
    waker:                  Option<Waker>
}

impl RoundStateMachine {
    pub fn new(
        block_height: BlockNumber,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
        round_leader: PeerId,
        validators: Vec<AngstromValidator>,
        metrics: ConsensusMetricsWrapper
    ) -> Self {
        let initial_state_duration = Duration::from_secs(INITIAL_STATE_DURATION);
        let timer = Box::pin(time::sleep(initial_state_duration));

        Self {
            current_state: Self::initial_state(block_height),
            round_leader,
            validators,
            initial_state_duration,
            order_storage,
            signer,
            metrics,
            transition_future: None,
            initial_state_timer: Some(timer),
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

    pub fn reset_round(&mut self, block: BlockNumber, leader: PeerId) {
        self.round_leader = leader;
        self.current_state = Self::initial_state(block);
        self.initial_state_timer = Some(Box::pin(time::sleep(self.initial_state_duration)));
        self.transition_future = None;
    }

    pub fn initial_state(block_height: BlockNumber) -> ConsensusState {
        ConsensusState::BidSubmission(BidSubmission { block_height, ..Default::default() })
    }

    pub fn my_pre_proposal(&self, pre_proposals: &HashSet<PreProposal>) -> Option<StromMessage> {
        pre_proposals
            .iter()
            .find(|p| p.source == self.my_id())
            .cloned()
            .map(StromMessage::PrePropose)
    }

    /// Invariants:
    ///  * won't be called with messages with payload from ourselves
    ///  * won't be called with messages for non-current block height
    ///
    /// Returns:
    ///   In general we do not want to return an updated message for broadcast.
    ///   However, during bid aggregation, we need to tell the rest of the
    ///   network what our updated pre_proposal is, so when it's time for the
    ///   leader to propose, it needs to see an agreement on the ToB and RoB
    pub fn on_strom_message(&mut self, strom_msg: StromConsensusEvent) -> Option<StromMessage> {
        let is_leader = self.i_am_leader();
        match strom_msg {
            StromConsensusEvent::PreProposal(_, pre_proposal) => {
                if matches!(self.current_state, ConsensusState::Finalization(_)) {
                    return None
                }

                if !pre_proposal.is_valid() {
                    return None;
                }

                if !is_leader {
                    let mut local_limit_orders = Vec::new();
                    let mut local_searcher_orders = Vec::new();
                    if matches!(self.current_state, ConsensusState::BidSubmission(_)) {
                        let OrderSet { limit, searcher, .. } = self.order_storage.get_all_orders();
                        local_limit_orders.extend(limit);
                        local_searcher_orders.extend(searcher);
                    }

                    let block_height = self.current_state.block_height();
                    let my_id = self.signer.my_id;
                    let key = self.signer.key.clone();
                    let pre_proposals = self.current_state.pre_proposals_mut();
                    let merged_pre_proposal = Self::generate_our_merged_pre_proposal(
                        block_height,
                        local_limit_orders,
                        local_searcher_orders,
                        pre_proposals,
                        my_id,
                        &key
                    );
                    pre_proposals.insert(merged_pre_proposal.clone());

                    return Some(StromMessage::PrePropose(merged_pre_proposal));
                }

                // Leader path
                let pre_proposals = self.current_state.pre_proposals_mut();
                pre_proposals.insert(pre_proposal);

                if self.orders_have_quorum(self.all_searcher_orders(pre_proposals))
                    && self.orders_have_quorum(self.all_limit_orders(pre_proposals))
                {
                    // self.start_bundle_building(bid_submission);
                    return None;
                }
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                let Proposal {
                    source: proposal_sender, block_height: proposal_block_height, ..
                } = proposal;

                let pre_proposals = self.current_state.pre_proposals_mut();
                if proposal.is_valid() && !is_leader {
                    let new_state = ConsensusState::Finalization(Finalization {
                        block_height:  proposal_block_height,
                        proposal:      Some(proposal),
                        pre_proposals: pre_proposals.clone()
                    });
                    self.force_transition(new_state);
                }

                if is_leader {
                    panic!(
                        "Message sender: {}, Proposal block height: {}, Proposal source: {}, \
                         Current state {}, something is terribly wrong",
                        msg_sender,
                        proposal_block_height,
                        proposal_sender,
                        self.current_state.name()
                    );
                }
            }
            StromConsensusEvent::Commit(..) => {}
        };

        None

        // match &mut self.current_state {
        //     ConsensusState::BidSubmission(ref mut bid_submission) => {
        //         bid_submission.pre_proposals.insert(PreProposal::default());
        //         return None
        //         // self.handle_bid_state(bid_submission, strom_msg,
        // is_leader)     }
        //     ConsensusState::BidAggregation(ref mut bid_aggregation) => {
        //         bid_aggregation.pre_proposals.insert(PreProposal::default());
        //         // self.handle_bid_state(bid_aggregation, strom_msg,
        // is_leader)     }
        //     ConsensusState::Finalization(ref mut finalization) => {
        //         self.handle_finalization(finalization, strom_msg)
        //     }
        // }
    }

    fn handle_finalization(
        &mut self,
        finalization: &mut Finalization,
        strom_msg: StromConsensusEvent
    ) -> Option<StromMessage> {
        match strom_msg {
            StromConsensusEvent::PreProposal(..) | StromConsensusEvent::Proposal(..) => {
                // Ignore PreProposal and Proposal messages in Finalization
                // state
            }
            StromConsensusEvent::Commit(..) => {
                // Handle commit messages if needed
            }
        }
        None
    }

    fn generate_bid_aggregation(
        &self,
        block_height: BlockNumber,
        pre_proposals: &HashSet<PreProposal>
    ) -> BidAggregation {
        let OrderSet { limit, searcher } = self.order_storage.get_all_orders();
        let mut pre_proposals = pre_proposals.clone();

        let pre_proposal = Self::generate_our_merged_pre_proposal(
            block_height,
            limit,
            searcher,
            &pre_proposals,
            self.my_id(),
            &self.signer.key.clone()
        );
        pre_proposals.insert(pre_proposal);

        BidAggregation { block_height, pre_proposals }
    }

    fn generate_our_merged_pre_proposal(
        block_height: BlockNumber,
        limit: Vec<OrderWithStorageData<GroupedVanillaOrder>>,
        searcher: Vec<OrderWithStorageData<TopOfBlockOrder>>,
        pre_proposals: &HashSet<PreProposal>,
        my_id: PeerId,
        key: &SecretKey
    ) -> PreProposal {
        let merged_limit_orders = pre_proposals
            .iter()
            .flat_map(|p| p.limit.clone())
            .chain(limit)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        let merged_searcher_orders = pre_proposals
            .iter()
            .flat_map(|p| p.searcher.clone())
            .chain(searcher)
            .group_by(|order| order.pool_id)
            .into_iter()
            .map(|(_, group)| group.max_by_key(|order| order.tob_reward).unwrap())
            .collect();

        PreProposal::generate_pre_proposal(
            block_height,
            my_id,
            merged_limit_orders,
            merged_searcher_orders,
            &key
        )
    }

    fn all_searcher_orders(
        &self,
        pre_proposals: &HashSet<PreProposal>
    ) -> Vec<OrderWithStorageData<TopOfBlockOrder>> {
        pre_proposals
            .iter()
            .flat_map(|pre_proposal| pre_proposal.searcher.clone())
            .collect()
    }

    fn all_limit_orders(
        &self,
        pre_proposals: &HashSet<PreProposal>
    ) -> Vec<OrderWithStorageData<GroupedVanillaOrder>> {
        pre_proposals
            .iter()
            .flat_map(|pre_proposal| pre_proposal.limit.clone())
            .collect()
    }

    fn orders_have_quorum<T: Hash + Eq + Clone>(
        &self,
        orders: Vec<OrderWithStorageData<T>>
    ) -> bool {
        orders.len() == self.filter_quorum_orders(orders).len()
    }

    fn filter_quorum_orders<T: Hash + Eq + Clone>(
        &self,
        input: Vec<OrderWithStorageData<T>>
    ) -> Vec<OrderWithStorageData<T>> {
        input
            .iter()
            .fold(HashMap::new(), |mut acc, order| {
                *acc.entry(order).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .filter(|(_, count)| self.has_quorum(*count))
            .map(|(order, _)| order.clone())
            .collect()
    }

    fn force_transition(&mut self, mut new_state: ConsensusState) {
        let from_state = self.current_state.clone();
        let signer = self.signer.clone();

        self.transition_future = Some(Box::pin(async move {
            if let (
                ConsensusState::BidAggregation(BidAggregation {
                    block_height: pre_proposal_height,
                    pre_proposals
                }),
                ConsensusState::Finalization(ref mut finalization)
            ) = (from_state, &mut new_state)
            {
                let pre_proposals: Vec<PreProposal> = pre_proposals.iter().cloned().collect();
                let (tx, rx) = tokio::sync::oneshot::channel();

                tokio::spawn(async move {
                    let solutions = build_proposal(pre_proposals.clone()).await.unwrap();
                    let proposal =
                        signer.sign_proposal(pre_proposal_height, pre_proposals, solutions);
                    let _ = tx.send(proposal);
                });

                let proposal = rx.await.unwrap();
                finalization.proposal = Some(proposal);

                // TODO: submit the proposal/bundle to the chain
                // Ethereum.submit(proposal).await
            }
            new_state
        }));

        // wake up the task to ensure poll_next is called
        if let Some(waker) = &self.waker {
            waker.wake_by_ref();
        }
    }
}

impl Stream for RoundStateMachine {
    type Item = ConsensusState;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        this.waker = Some(cx.waker().clone());

        if let Some(future) = &mut this.transition_future {
            return match future.as_mut().poll(cx) {
                Poll::Ready(new_state) => Poll::Ready(Some(new_state)),
                Poll::Pending => Poll::Pending
            };
        }

        if let Some(timer) = &mut this.initial_state_timer {
            if timer.as_mut().poll(cx).is_ready() {
                // if the timer ran out and we are not in bid submission, something curious
                // happened
                if let ConsensusState::BidSubmission(BidSubmission {
                    block_height,
                    pre_proposals
                }) = &this.current_state
                {
                    let bid_aggregation =
                        this.generate_bid_aggregation(*block_height, pre_proposals);
                    this.transition_future =
                        Some(Box::pin(async { ConsensusState::BidAggregation(bid_aggregation) }));
                    this.initial_state_timer = None;
                }
            }
        }

        Poll::Pending
    }
}

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
    fn name(&self) -> &'static str {
        match self {
            Self::BidSubmission(_) => "BidSubmission",
            Self::BidAggregation(_) => "BidAggregation",
            Self::Finalization(_) => "Finalization"
        }
    }
}

impl ConsensusState {
    pub fn block_height(&self) -> BlockNumber {
        match self {
            Self::BidSubmission(state) => state.block_height,
            Self::BidAggregation(state) => state.block_height,
            Self::Finalization(state) => state.block_height
        }
    }

    pub fn pre_proposals_mut(&mut self) -> &mut HashSet<PreProposal> {
        match self {
            Self::BidSubmission(state) => &mut state.pre_proposals,
            Self::BidAggregation(state) => &mut state.pre_proposals,
            Self::Finalization(state) => &mut state.pre_proposals
        }
    }
}
