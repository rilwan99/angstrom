use std::collections::VecDeque;

use guard_types::consensus::Block;

pub struct UnfinalizedBlockQueue(pub VecDeque<Block>);

impl UnfinalizedBlockQueue {
    pub fn new() -> Self {
        UnfinalizedBlockQueue(VecDeque::default())
    }

    pub fn on_new_block(&mut self, block: Block) {
        self.0.push_back(block);
    }

    pub fn new_finalized_ethereum_block(&mut self, eth_block: u64) -> Vec<Block> {
        let mut res = Vec::new();
        while self
            .0
            .front()
            .filter(|b| b.header.ethereum_height <= eth_block)
            .is_some()
        {
            res.push(self.0.pop_front().unwrap())
        }

        res
    }
}
