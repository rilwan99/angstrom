use std::{
    sync::Arc,
    task::{ready, Context, Poll}
};

use futures::FutureExt;
use guard_eth_wire::{message::RequestPair, EthMessage};
use guard_types::{
    consensus::{
        Block, Bundle23Votes, BundleVote, GuardInfo, LeaderProposal, SignedLeaderProposal,
        Valid23Bundle
    },
    database::State,
    on_chain::{SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement}
};
use reth_interfaces::p2p::error::RequestResult;
use tokio::sync::{oneshot, oneshot::Sender as OneSender};

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    // Consensus related messages
    /// new vote for a bundle
    BundleVote(Arc<BundleVote>),
    /// bundle that has 2/3
    Bundle23Vote(Arc<Valid23Bundle>),
    /// proposer block
    LeaderProposal(Arc<LeaderProposal>),
    /// signed leader proposal
    SignedLeaderProposal(Arc<SignedLeaderProposal>),
    /// new block that the network finalized on
    NewBlock(Arc<Block>),
    /// the state correlated to the underlying block
    NewState(Arc<State>),

    // default propagation messages
    /// new simmed user txes
    PropagateUserTransaction(Arc<SimmedUserSettlement>),
    /// new simmed searcher txes
    PropagateSearcherTransaction(Arc<SimmedLvrSettlement>),
    /// propagates a new bundle
    PropagateBundle(Arc<SimmedBundle>)
}
