use std::{
    sync::Arc,
    task::{ready, Context, Poll}
};

use futures::FutureExt;
use guard_eth_wire::{message::RequestPair, EthMessage};
use guard_types::{
    consensus::{
        Block, Bundle23Votes, BundleVote, GuardInfo, LeaderProposal, PrePreposeBundle,
        ProposalCommit, SignedLeaderProposal, Valid23Bundle
    },
    database::State,
    on_chain::{
        SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement, SubmittedOrder, VanillaBundle
    }
};
use reth_interfaces::p2p::error::RequestResult;
use tokio::sync::{oneshot, oneshot::Sender as OneSender};

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    // Consensus related messages
    PrePropose(Arc<PrePreposeBundle>),
    Proposal(Arc<LeaderProposal>),
    Commit(Arc<ProposalCommit>),

    // default communication
    PropagateOrder(Arc<SubmittedOrder>),
    PropagateBundle(Arc<VanillaBundle>)
}
