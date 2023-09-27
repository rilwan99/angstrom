use guard_types::consensus::GuardSet;

use crate::DatabaseError;

#[auto_impl(&, Arc, Box)]
pub trait GuardSetReader: Send + Sync {
    fn get_head(&self) -> Result<GuardSet, DatabaseError>;
    fn get_set_for_block(&self, block: u64) -> Result<GuardSet, DatabaseError>;
}

#[auto_impl(&, Arc, Box)]
pub trait GuardSetWriter: Send + Sync {
    fn new_guard_set(&self, block: u64, set: GuardSet) -> Result<(), DatabaseError>;
}
