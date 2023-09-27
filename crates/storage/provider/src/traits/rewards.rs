use std::ops::RangeBounds;

use guard_types::database::RewardsHeader;

use crate::DatabaseError;

#[auto_impl(&, Arc, Box)]
pub trait RewardsReader: Send + Sync {
    fn rewards_from_range(
        &self,
        range: impl RangeBounds<u64>
    ) -> Result<Vec<RewardsHeader>, DatabaseError>;

    fn rewards_for_block(&self, block: u64) -> Result<Option<RewardsHeader>, DatabaseError>;
}

#[auto_impl(&, Arc, Box)]
pub trait RewardsWriter: Send + Sync {
    fn new_reward(&self, block: u64, reward: RewardsHeader) -> Result<(), DatabaseError>;
}
