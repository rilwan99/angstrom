// use guard_provider::traits::{GuardSetWriter, RewardsWriter, StateWriter};
use guard_types::consensus::Block;
use reth_db::{
    database::Database,
    mdbx::{tx::Tx, Env, WriteMap, RO}
};

use super::unfinalized::UnfinalizedBlockQueue;

pub struct ChainMaintainer<T> {
    reth_db:           Env<WriteMap>,
    unfinalized_queue: UnfinalizedBlockQueue,
    provider:          T
}

impl<T> ChainMaintainer<T>
// where
//     T: GuardSetWriter + RewardsWriter + StateWriter
{
    pub fn on_new_finalized_eth_block(&mut self, block: u64) {
        self.unfinalized_queue
            .new_finalized_ethereum_block(block)
            .into_iter()
            .filter(|possible_block| {
                // let tx = self.reth_db.tx();
                // let block = tx.get_block(possible_block.new_finalized_ethereum_block);
                true
            });
    }

    pub fn on_new_height(&mut self, block: Block) {
        self.unfinalized_queue.0.push_back(block);
    }
}
