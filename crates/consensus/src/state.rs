use guard_provider::traits::{GuardSetWriter, RewardsWriter, StateWriter};
use reth_db::mdbx::{tx::Tx, Env, WriteMap, RO};

use super::unfinalized::UnfinalizedBlockQueue;

pub struct ChainMaintainer<T: GuardSetWriter + RewardsWriter + StateWriter> {
    reth_db:           Env<WriteMap>,
    unfinalized_queue: UnfinalizedBlockQueue,
    provider:          T
}

impl<T> ChainMaintainer<T>
where
    T: GuardSetWriter + RewardsWriter + StateWriter
{
    pub fn on_new_finalized_eth_block(&mut self, block: u64) {
        let possible_blocks = self.unfinalized_queue.new_finalized_ethereum_block(block);
    }

    pub fn on_new_height(&mut self, block: Block) {
        self.0.unfinalized_queue.push_back(block);
    }
}
