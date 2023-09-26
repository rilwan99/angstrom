use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use reth_codecs::{main_codec, Compact};
use reth_primitives::H512;
use reth_rlp::{Decodable, DecodeError, Encodable, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardSet {
    pub guards:             HashSet<GuardInfo>,
    pub leader:             Option<GuardInfo>,
    pub total_voting_power: u64
}

impl Ord for GuardSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for GuardSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ordering::Equal)
    }
}

impl GuardSet {
    pub fn contains_key(&self, key: H512) -> bool {
        self.guards.contains(&GuardInfo {
            pub_key:         key,
            voting_power:    0,
            leader_priority: 0
        })
    }

    pub fn len(&self) -> usize {
        self.guards.len()
    }

    pub fn get_current_leader(&self) -> Option<&GuardInfo> {
        self.leader.as_ref()
    }
}

#[main_codec]
#[derive(Debug, Clone, RlpDecodable, RlpEncodable, PartialOrd, Ord)]
pub struct GuardInfo {
    pub pub_key:         H512,
    pub voting_power:    u64,
    pub leader_priority: u64
}

impl PartialEq for GuardInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pub_key == other.pub_key
    }
}
impl Eq for GuardInfo {}

impl Hash for GuardInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pub_key.hash(state);
    }
}
