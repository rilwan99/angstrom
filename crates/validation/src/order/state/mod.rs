use reth_provider::StateProviderFactory;

mod upkeepers;

/// State validation is all validation that requires reading from the Ethereum
/// database, these operations are:
/// 1) validating order nonce,
/// 2) checking token balances
/// 3) checking token approvals
pub struct StateValidation<DB> {
    db: DB,
}

impl<DB> StateValidation<DB>
where
    DB: StateProviderFactory
{
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}
