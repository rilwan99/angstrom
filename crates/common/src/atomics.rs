use std::sync::{
    atomic::{AtomicBool, AtomicU8, Ordering},
    Arc
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConsensusState {
    OrderAccumulation = 0,
    PrePropose        = 1,
    Propose           = 2,
    Commit            = 3,
    Submit            = 4
}

pub const ORDER_ACCUMULATION: ConsensusState = ConsensusState::OrderAccumulation;
pub const PRE_PROPOSE: ConsensusState = ConsensusState::PrePropose;
pub const PROPOSE: ConsensusState = ConsensusState::Propose;
pub const COMMIT: ConsensusState = ConsensusState::Commit;
pub const SUBMIT: ConsensusState = ConsensusState::Submit;

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct AtomicConsensus(Arc<AtomicU8>);

impl AtomicConsensus {
    pub fn reset(&self) {
        self.0.store(0, Ordering::SeqCst);
    }

    pub fn get_current_state(&self) -> ConsensusState {
        // this is safe due to the bound on the underlying atomic to the enum
        unsafe { std::mem::transmute(self.0.load(Ordering::SeqCst)) }
    }

    pub fn update_state(&self, state: ConsensusState) {
        // this is safe due to the bound on the underlying atomic to the enum
        self.0
            .store(unsafe { std::mem::transmute(state) }, Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct IsLeader(Arc<AtomicBool>);

impl IsLeader {
    pub fn is_leader(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    pub fn set_leader(&self, is_leader: bool) {
        self.0.store(is_leader, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod test {
    use crate::{AtomicConsensus, ConsensusState};

    #[test]
    fn test_atomic_consensus_state() {
        let atomic = AtomicConsensus::default();
        assert_eq!(atomic.get_current_state(), ConsensusState::OrderAccumulation);
        atomic.update_state(ConsensusState::Submit);
        assert_eq!(atomic.get_current_state(), ConsensusState::Submit);
        atomic.reset();
        assert_eq!(atomic.get_current_state(), ConsensusState::OrderAccumulation);
    }
}
