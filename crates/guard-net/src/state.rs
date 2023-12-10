use std::collections::HashSet;

use reth_primitives::PeerId;

use crate::PeersManager;

#[derive(Debug)]
pub struct StromState<DB> {
    db:            DB,
    active_peers:  HashSet<PeerId>,
    peers_manager: PeersManager
}

impl<DB> StromState<DB> {
    pub fn peers_mut(&mut self) -> &mut PeersManager {
        &mut self.peers_manager
    }

    pub fn poll(&mut self) -> Option<StateEvent> {
        None
    }
}

pub enum StateEvent {}
