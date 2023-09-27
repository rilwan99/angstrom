use std::ops::RangeBounds;

use guard_types::database::State;

use crate::DatabaseError;

#[auto_impl(&, Arc, Box)]
pub trait StateReader: Send + Sync {
    fn get_newest_state(&self) -> Result<State, DatabaseError>;
    fn get_state_range(&self, range: impl RangeBounds<u64>) -> Result<Vec<State>, DatabaseError>;
    fn get_state_for_block(&self, block: u64) -> Result<Option<State>, DatabaseError>;
}

#[auto_impl(&, Arc, Box)]
pub trait StateWriter: Send + Sync {
    fn write_state(&self, block: u64, state: State) -> Result<(), DatabaseError>;
}
