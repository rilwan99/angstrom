use std::collections::HashSet;

use reth_network::peers::PeersManager;
use reth_primitives::PeerId;

pub struct StromState<DB> {
    db:           DB,
    active_peers: HashSet<PeerId>,
    peers:        PeersManager
}

impl<DB> StromState<DB> {
    pub fn poll(&mut self) -> Option<StateEvent> {
        None
    }
}

pub enum StateEvent {}
