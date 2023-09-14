use std::collections::HashSet;

use guard_types::consensus::GuardInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuardSet {
    guards:             HashSet<GuardInfo>,
    leader:             Option<GuardInfo>,
    total_voting_power: u64
}
