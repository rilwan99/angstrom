#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GuardInfo {
    pub_key:         [u8; 32],
    voting_power:    u64,
    leader_priority: u64
}
