use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsensusState {
    PrePropose = 0,
    Propose    = 1,
    Commit     = 2,
    Submit     = 3
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct AtomicConsensus(Arc<AtomicU8>);

impl AtomicConsensus {
    pub fn reset(&self) {
        self.0.store(0, Ordering::SeqCst);
    }

    pub fn get_current_state(&self) -> ConsensusState {
        unsafe { std::mem::transmute(self.0.load(Ordering::SeqCst)) }
    }

    pub fn update_state(&self, state: ConsensusState) {
        self.0
            .store(unsafe { std::mem::transmute(state) }, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod test {
    use crate::{AtomicConsensus, ConsensusState};

    #[test]
    fn test_atomic_consensus_state() {
        let atomic = AtomicConsensus::default();
        assert_eq!(atomic.get_current_state(), ConsensusState::PrePropose);
        atomic.update_state(ConsensusState::Submit);
        assert_eq!(atomic.get_current_state(), ConsensusState::Submit);
        atomic.reset();
        assert_eq!(atomic.get_current_state(), ConsensusState::PrePropose);
    }
}
