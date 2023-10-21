use std::sync::Arc;

use guard_eth_wire::{message::RequestPair, EthMessage};
use guard_types::{
    consensus::{Commit, PreProposal, Proposal},
    on_chain::{SubmittedOrder, VanillaBundle}
};
use reth_interfaces::p2p::error::RequestResult;

/// General bi-directional messages sent to & from peers
#[derive(Debug, Clone)]
pub enum PeerMessages {
    // Consensus related messages
    PrePropose(Arc<PreProposal>),
    Proposal(Arc<Proposal>),
    Commit(Arc<Commit>),

    // default communication
    PropagateOrder(Arc<SubmittedOrder>),
    PropagateBundle(Arc<VanillaBundle>)
}
