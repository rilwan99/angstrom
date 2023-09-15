use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GuardInfo {
    pub pub_key:         [u8; 32],
    pub voting_power:    u64,
    pub leader_priority: u64
}
