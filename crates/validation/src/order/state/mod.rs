use reth_provider::StateProviderFactory;

use self::orders::UserOrders;

mod orders;

/// State validation is all validation that requires reading from the Ethereum
/// database, these operations are:
/// 1) validating order nonce,
/// 2) checking token balances
/// 3) checking token approvals
#[allow(dead_code)]
pub struct StateValidation<DB> {
    db:          DB,
    /// manage the upkeep of all data needed for validation
    /// all current user orders with the current changed deltas
    user_orders: UserOrders // pending_validation:
}

impl<DB> StateValidation<DB>
where
    DB: StateProviderFactory
{
    pub fn new(_db: DB) -> Self {
        todo!()
    }
}
