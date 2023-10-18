use std::{collections::HashMap, sync::Arc};

use ethers_core::types::{Block, H256};
use reth_primitives::H512;
use serde::{Deserialize, Serialize};

const ROUND_ROBIN_CACHE: &str = "./";

/// This gets written to a cache location on your local machine
/// in order to avoid having to resync durning downtime.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundRobinAlgo {
    /// this is just a placeholder
    guards_with_score:    HashMap<H512, u64>,
    current_block_height: u64
}

impl RoundRobinAlgo {
    /// also returns what the cache height is up to in order to properly deal
    /// with syncing
    pub fn new() -> (Self, u64) {
        if let Some(cache) = Self::load_cache() {
            let bh = cache.current_block_height;
            (cache, bh)
        } else {
            (Self { guards_with_score: HashMap::new(), current_block_height: 0 }, 0)
        }
    }

    fn load_cache() -> Option<Self> {
        todo!()
    }

    /// who the leader is for this round
    pub fn on_new_block(&mut self, block: Arc<Block<H256>>) -> H512 {
        todo!()
    }
}
