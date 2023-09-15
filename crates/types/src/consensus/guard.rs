use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardSet {
    pub guards:             HashSet<GuardInfo>,
    pub leader:             Option<GuardInfo>,
    pub total_voting_power: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GuardInfo {
    pub pub_key:         [u8; 32],
    pub voting_power:    u64,
    pub leader_priority: u64
}
