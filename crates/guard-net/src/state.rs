use std::{collections::HashSet, sync::Arc, task::Context};

use alloy_sol_macro::sol;
use parking_lot::RwLock;
use reth_primitives::PeerId;

use crate::PeersManager;

sol! {
    function validators() public view returns(address[]);
}

#[derive(Debug)]
pub struct StromState<DB> {
    db:           DB,
    active_peers: HashSet<PeerId>,

    validators:    Arc<RwLock<HashSet<PeerId>>>,
    peers_manager: PeersManager
}

impl<DB> StromState<DB> {
    pub fn peers_mut(&mut self) -> &mut PeersManager {
        &mut self.peers_manager
    }

    pub fn update_validators(&mut self) {}

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Option<StateEvent> {
        None
    }
}

pub enum StateEvent {}
