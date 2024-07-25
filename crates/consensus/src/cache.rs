use std::collections::HashMap;

use angstrom_types::consensus::Proposal;

pub struct ProposalCache {
    proposals: HashMap<u64, Proposal>
}

impl ProposalCache {
    pub fn new() -> Self {
        Self { proposals: HashMap::new() }
    }

    pub fn set(&mut self, ethereum_height: u64, proposal: Proposal) -> bool {
        if self.proposals.contains_key(&ethereum_height) {
            return false;
        }
        self.proposals.insert(ethereum_height, proposal);
        true
    }

    pub fn get(&self, ethereum_height: u64) -> Option<&Proposal> {
        self.proposals.get(&ethereum_height)
    }
}
