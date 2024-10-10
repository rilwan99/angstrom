use std::{
    collections::{HashMap, HashSet},
    default::Default,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration
};

use crate::Signer;
use alloy_primitives::BlockNumber;
use angstrom_metrics::ConsensusMetricsWrapper;
use angstrom_network::manager::StromConsensusEvent;
use angstrom_types::sol_bindings::AngstromContract::saveNodeFeeReturn;
use angstrom_types::{
    consensus::{Commit, PreProposal, Proposal},
    orders::{OrderSet, PoolSolution},
    primitive::Signature,
    sol_bindings::{
        grouped_orders::{
            AllOrders, GroupedComposableOrder, GroupedUserOrder, GroupedVanillaOrder,
            OrderWithStorageData
        },
        rpc_orders::TopOfBlockOrder
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
            ConsensusState::BidSubmission(_) => "BidSubmission",
            ConsensusState::BidAggregation(_) => "BidAggregation"
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
        block: BlockNumber,
        order_storage: Arc<OrderStorage>,
        signer: Signer,
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

        let initial_state = RoundStateMachine::initial_state(block);
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

    pub fn reset_state(&mut self, block: BlockNumber) {
        let initial_state = RoundStateMachine::initial_state(block);
        let initial_state_duration = self.durations.get(initial_state.as_key()).unwrap();
        let timer = Box::pin(time::sleep(*initial_state_duration));
        self.transition_future = None;
        self.timer = timer
    }

    pub fn initial_state(block_height: BlockNumber) -> ConsensusState {
        ConsensusState::BidSubmission(BidSubmission {
            block_height,
            pre_proposals: Vec::new(),
            tob_bids: HashSet::new(),
            rob_tx: HashSet::new()
        })
    }

    pub fn on_strom_message(&mut self, strom_msg: StromConsensusEvent) {
        match &mut self.current_state {
            // it's just a timeout to get enough orders from users
            ConsensusState::BidSubmission(BidSubmission {
                block_height,
                mut pre_proposals,
                ..
            }) => {
                match strom_msg {
                    StromConsensusEvent::PreProposal(peer_id, pre_proposal) => {
                        let PreProposal { block_height: pre_proposal_height, source: pre_proposal_sender, limit, searcher, signature } =
                            pre_proposal;
                        if pre_proposal_height < *block_height || pre_proposal_height > *block_height {
                            tracing::warn!(
                                msg_sender = peer_id,
                                ?block_height,
                                ?pre_proposal_sender,
                                ?pre_proposal_height,
                                "received pre_proposal wrong height"
                            );
                            return
                        }
                        if !pre_proposal.validate() {
                            tracing::warn!(
                                msg_sender = peer_id,
                                ?block_height,
                                  ?pre_proposal_sender,
                                ?pre_proposal_height,
                                "received pre_proposal with invalid signature"
                            );
                            return
                        }
                        pre_proposals.push(pre_proposal.clone());
                    }
                    StromConsensusEvent::Proposal(
                        peer_id,
                        Proposal { block_height: ethereum_height, source, preproposals, solutions, signature }
                    ) => {
                        if ethereum_height < *block_height || ethereum_height > *block_height {
                            tracing::warn!(
                                messeg_sender = peer_id,
                                ?block_height,
                                pre_proposal_sender = source,
                                bid_submissino_height = ethereum_height,
                                "received proposal for wrong height"
                            );
                            return
                        }

                        // if it's for old round ignore
                        // Handle BidAggregation event
                        // Example: rob_tx.insert(proposal);
                    }
                    StromConsensusEvent::Commit(peer_id, Commit {block_height: commit_height, .. }) => {
                        if commit_height < *block_height {
                            tracing::debug!(
                                messeg_sender = peer_id,
                                ?block_height,
                                "received commit for wrong height"
                            );
                            return
                        }

                        tracing::warn!("received a commit for the current or future block. That's weird, but I'll ignore")
                    }
                }
            }
            ConsensusState::BidAggregation(BidAggregation { block_height,pre_proposals, .. }) => {
                match strom_msg {
                    StromConsensusEvent::PreProposal(peer_id, pre_proposal) => {
                        let PreProposal { block_height: ethereum_height, source, ..} = pre_proposal;
                        // if it's for old round ignore
                        if ethereum_height < *block_height || ethereum_height > *block_height {
                            tracing::warn!(
                                messeg_sender = peer_id,
                                ?block_height,
                                pre_proposal_sender = source,
                                bid_submissino_height = ethereum_height,
                                "received pre_proposal for wrong height"
                            );
                            return
                        }

                        pre_proposals.push(pre_proposal);
                    }
                    StromConsensusEvent::Proposal(peer_id, proposal) => {
                        // if it's for old round ignore

                        // Handle BidAggregation event
                        // Example: pre_proposals.push(proposal);
                    }
                    StromConsensusEvent::Commit(peer_id, commit) => {
                        // if it's for old round ignore

                        // Handle Commit event
                        // Example: process_commit(commit);
                    }
                }
            } /* ConsensusState::LeaderAction { .. } => {
               *     match strom_msg {
               *         StromConsensusEvent::BidSubmission(peer_id, pre_proposal) => {
               *             // Handle BidSubmission event
               *         }
               *         StromConsensusEvent::BidAggregation(peer_id, proposal) => {
               *             // Handle BidAggregation event
               *         }
               *         StromConsensusEvent::Commit(peer_id, commit) => {
               *             // Handle Commit event
               *         }
               *     }
               * }
               * ConsensusState::PostConsensusVerification { ref mut all_bids, ref mut
               * all_transactions, .. } => {     match strom_msg {
               *         StromConsensusEvent::BidSubmission(peer_id, pre_proposal) => {
               *             // Handle BidSubmission event
               *         }
               *         StromConsensusEvent::BidAggregation(peer_id, proposal) => {
               *             // Handle BidAggregation event
               *         }
               *         StromConsensusEvent::Commit(peer_id, commit) => {
               *             // Handle Commit event
               *         }
               *     }
               * } */
        }
    }

    pub fn duration(&self, state: &ConsensusState) -> Duration {
        self.durations.get(state.as_key()).cloned().unwrap()
    }

    pub async fn force_transition(&mut self, new_state: ConsensusState) {
        let new_state = match (&self.current_state, &new_state) {
            (
                ConsensusState::BidSubmission(BidSubmission { block_height, .. }),
                ConsensusState::BidAggregation(_)
            ) => {
                let tob_bids = self.order_storage.get_all_orders();
                let rob_transactions = self.order_storage.get_all_orders();
                ConsensusState::BidAggregation(BidAggregation {
                    block_height:  *block_height,
                    proposal: None,
                    pre_proposals: Vec::new()
                })
            }
            // (
            //     ConsensusState::BidAggregation { block_height, tob_bids, rob_transactions, .. },
            //     ConsensusState::LeaderAction { .. }
            // ) => {
            //     let highest_tob_bid = tob_bids.iter().max_by_key(|bid|
            // *bid.price).unwrap().clone();     let final_bundle =
            // self.signer.sign_proposal(*block_height, vec![highest_tob_bid.clone()], vec![]);
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
                ConsensusState::BidAggregation(BidAggregation { block_height, pre_proposals, proposal: None })
            }
            ConsensusState::BidAggregation(BidAggregation { block_height, pre_proposals,proposal }) => {
                ConsensusState::BidSubmission(BidSubmission::default())
            }
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
