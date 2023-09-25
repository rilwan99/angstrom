use std::collections::HashSet;

use guard_types::{
    consensus::{GuardSet, LeaderProposal, SignedLeaderProposal},
    get_public_key
};
use tracing::info;

pub struct ProposalManager {
    current_proposal: Option<LeaderProposal>,
    votes:            HashSet<SignedLeaderProposal>,

    height: u64,
    round:  u64
}

impl ProposalManager {
    pub fn new_proposal(&mut self, proposal: LeaderProposal) {}

    pub fn new_proposal_vote(&mut self, vote: SignedLeaderProposal, guards: &GuardSet) -> bool {
        let Some(proposal) = self.current_proposal.as_ref() else { return false };
        let message = proposal.bundle_hash();
        let public_key = get_public_key(&vote.0, message);

        if guards.contains_key(public_key) {
            info!(?vote, ?public_key, "got vote for proposal");
            self.votes.insert(vote);

            return true
        }
        false
    }

    pub fn has_proposal(&self) -> bool {
        self.current_proposal.is_some()
    }
}
