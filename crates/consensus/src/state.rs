pub struct ChainMaintainer<T: GuardSetWriter + RewardsWriter + StateWriter> {
    unfinalized_queue: UnfinalizedBlockQueue,
    provider:          T
}

impl<T> ChainMaintainer<T>
where
    T: GuardSetWriter + RewardsWriter + StateWriter
{
    pub fn on_new_finalized_eth_block(&mut self, block: u64) {}

    pub fn on_new_height(&mut self, block: Block) {
        self.unfinalized_queue.push_back(block);
    }
}
