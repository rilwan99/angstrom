use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    time::Duration
};

use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::{OrderSet, PoolSolution},
    sol_bindings::{
        grouped_orders::{
            AllOrders, GroupedComposableOrder, GroupedUserOrder, GroupedVanillaOrder,
            OrderWithStorageData
        },
        rpc_orders::TopOfBlockOrder,
        AngstromContract::saveNodeFeeReturn
    }
};
use futures::{FutureExt, Stream};
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
    pub pre_proposals: Vec<PreProposal>,
    pub tob_bids:      HashSet<OrderWithStorageData<TopOfBlockOrder>>,
    pub rob_tx:        HashSet<OrderWithStorageData<GroupedVanillaOrder>>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BidAggregation {
    pub block_height:  BlockNumber,
    pub proposal:      Option<Proposal>,
    pub pre_proposals: Vec<PreProposal>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusState {
    BidSubmission(BidSubmission),
    BidAggregation(BidAggregation)
}

impl ConsensusState {
    fn as_key(&self) -> &'static str {
        match self {
            Self::BidSubmission(_) => "BidSubmission",
            Self::BidAggregation(_) => "BidAggregation"
        }
    }
}

async fn build_proposal(pre_proposals: Vec<PreProposal>) -> Result<Vec<PoolSolution>, String> {
    let matcher = MatchingManager {};
    matcher.build_proposal(pre_proposals).await
}

pub struct RoundStateMachine {
    // get rid of the mutex
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
        ConsensusState::BidSubmission(BidSubmission {
            block_height,
            pre_proposals: Vec::new(),
            tob_bids: HashSet::new(),
            rob_tx: HashSet::new()
        })
    }

    /// Invariants:
    ///  * won't be called with messages from ourselves
    ///  * won't be called with messages for non-current block height
    pub async fn on_strom_message(&mut self, strom_msg: StromConsensusEvent) {
        let mut current_state = self.current_state.clone();
        let new_state: Option<ConsensusState> = match current_state {
            ConsensusState::BidSubmission(bid_submission) => {
                let (updated_submission, new_state) =
                    self.handle_in_bid_submission(bid_submission, strom_msg);
                self.current_state = ConsensusState::BidSubmission(updated_submission);
                new_state
            }
            ConsensusState::BidAggregation(bid_aggregation) => {
                let (updated_aggregation, new_state) = self
                    .handle_in_bid_aggregation(bid_aggregation, strom_msg)
                    .await;
                self.current_state = ConsensusState::BidAggregation(updated_aggregation);
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
                bid_submission.pre_proposals.push(pre_proposal.clone());
                (bid_submission, None)
            }
            StromConsensusEvent::Proposal(msg_sender, proposal) => {
                // TODO: we seem to be lagging
                let Proposal { source, .. } = proposal;
                if proposal.is_valid() && self.is_leader(source) {
                    // move to a commit phase
                    // ConsensusState::BidAggregation(BidAggregation{
                    //
                    // })
                    return (bid_submission, None);
                }
                (bid_submission, None)
            }
            StromConsensusEvent::Commit(msg_sender, commit) => {
                // TODO: we seem to be lagging
                // let Commit { source, .. } = commit;
                // if commit.is_valid() && self.is_leader(source) {
                //     // we seem to be lagging force transition
                // }
                (bid_submission, None)
            }
            _ => (bid_submission, None)
        }
    }

    async fn handle_in_bid_aggregation(
        &self,
        mut bid_aggregation: BidAggregation,
        strom_msg: StromConsensusEvent
    ) -> (BidAggregation, Option<ConsensusState>) {
        match strom_msg {
            StromConsensusEvent::PreProposal(msg_sender, pre_proposal) => {
                let PreProposal { block_height, source, .. } = pre_proposal;
                bid_aggregation.pre_proposals.push(pre_proposal);
                if self.i_am_leader() && self.has_quorum(bid_aggregation.pre_proposals.len()) {
                    let solutions = build_proposal(bid_aggregation.pre_proposals.to_vec())
                        .await
                        .unwrap();
                    let proposal = self.signer.sign_proposal(
                        block_height,
                        bid_aggregation.pre_proposals.clone(),
                        solutions
                    );
                    (
                        bid_aggregation.clone(),
                        Some(ConsensusState::BidAggregation(BidAggregation {
                            pre_proposals: bid_aggregation.pre_proposals.to_vec(),
                            block_height,
                            proposal: Some(proposal)
                        }))
                    )
                } else {
                    (bid_aggregation, None)
                }
            }
            StromConsensusEvent::Proposal(msg_sender, ..) => {
                // we are in bid aggregation and got a proposal
                if self.i_am_leader() {
                    todo!();
                }
                (bid_aggregation, None)
            }
            StromConsensusEvent::Commit(msg_sender, ..) => {
                // Handle Commit event
                // Example: process_commit(commit);
                (bid_aggregation, None)
            }
            _ => (bid_aggregation, None)
        }
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
                pre_proposals.push(pre_proposal);
                ConsensusState::BidAggregation(BidAggregation {
                    block_height,
                    pre_proposals,
                    proposal: None
                })
            }
            ConsensusState::BidAggregation(BidAggregation {
                block_height,
                pre_proposals,
                proposal
            }) => ConsensusState::BidSubmission(BidSubmission::default())
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
