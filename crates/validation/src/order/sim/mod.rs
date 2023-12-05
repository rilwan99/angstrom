use std::sync::Arc;

use crate::common::lru_db::RevmLRU;

/// The SimValidation module is responsible for dealing with orders that
/// need to be partly simulated to prove their possible validity.
/// These are composable orders where pre and post hooks add uncertainty in
/// execution validity
/// 1) This will Sim the pre-hook, do state validation with the updated pre-hook
/// data (Approvals transfers etc)
/// 2) Then normal State sim will occur
/// 3) post-hook will be simmed with updated data to ensure success
pub struct SimValidation<DB> {
    db: Arc<RevmLRU<DB>>
}
