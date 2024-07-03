use std::sync::RwLock;

#[derive(Debug, Default, Clone, Copy)]
pub enum ConsensusState {
    #[default]
    OrderAccumulation,
    PreProposal,
    PreProposalLaggards,
    Proposal
}

/// GlobalConsensusState contains the information about our consensus state that
/// we are willing to share with other modules during our run
#[derive(Default, Debug)]
pub struct GlobalConsensusState {
    pub(crate) state: RwLock<ConsensusState>
}

impl GlobalConsensusState {
    /// Update the globally shared Consensus state (only for use by the
    /// consensus crate)
    pub(crate) fn set_state(&mut self, new_state: ConsensusState) {
        let mut lock = self.state.write().unwrap();
        *lock = new_state;
    }

    /// Get the current consensus state
    pub fn state(&self) -> ConsensusState {
        let lock = self.state.read().unwrap();
        *lock
    }
}
