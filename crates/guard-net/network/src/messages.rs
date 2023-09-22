use std::{
    sync::Arc,
    task::{ready, Context, Poll}
};

use futures::FutureExt;
use guard_eth_wire::{message::RequestPair, EthMessage};
use guard_types::{
    consensus::{Bundle23Votes, BundleVote, Valid23Bundle},
    on_chain::{
        BundleSignature, SafeTx, SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement,
        TeeAddress
    }
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
    /// k

    /// new simmed user txes
    PropagateUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    /// new simmed searcher txes
    PropagateSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>),
    /// propagates a new bundle
    PropagateBundle(Arc<SimmedBundle>),
    /// leader request to get signatures on a new bundle
    PropagateSignatureRequest(Arc<SafeTx>),
    /// propgating the signature for the send out bundle
    PropagateBundleSignature(Arc<BundleSignature>)
}
