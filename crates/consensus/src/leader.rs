use guard_types::consensus::{LeaderProposal, SignedLeaderProposal};

pub struct ProposalManager {
    current_proposal: Option<LeaderProposal>,
    votes:            Vec<SignedLeaderProposal>,

    height: u64,
    round:  u64
}

impl ProposalManager {
    pub fn new_proposal(&mut self, proposal: LeaderProposal) {}
}
