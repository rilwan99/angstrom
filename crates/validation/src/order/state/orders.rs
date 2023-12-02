use std::fmt::Debug;

use alloy_primitives::{Address, U256};
use guard_types::orders::{OrderId, PoolOrder, ValidatedOrder};
use revm::primitives::HashMap;

/// the sum of all pending orders for a given user. This is done
/// so that validation of specific orders is not dependant on all other orders.
pub struct PendingState {
    token_balances:  HashMap<Address, U256>,
    token_approvals: HashMap<Address, U256>
}

pub struct UserOrders(HashMap<Address, (PendingState, Vec<OrderId>)>);

impl UserOrders {
    pub fn new_order<O: PoolOrder, Data: Clone + Debug>(
        &mut self,
        order: ValidatedOrder<O, Data>
    ) -> Result<(), ()> {
        let id: OrderId = order.into();
        let _ = self.check_for_nonce_overlap(&id)?;

        let user = id.address;

        Ok(())
    }

    fn apply_new_order_deltas(
        &mut self,
        token_in: Address,
        amount_in: Address,
        state: &mut PendingState
    ) -> Result<(), ()> {
        Ok(())
    }

    /// Helper function for checking for duplicates when adding orders
    fn check_for_nonce_overlap(&self, id: &OrderId) -> Result<(), ()> {
        if self
            .0
            .get(&id.address)
            .map(|inner| inner.1.iter().any(|other_id| other_id.nonce == id.nonce))
            .unwrap_or(false)
        {
            // return Err(PoolError::DuplicateNonce(id.clone()))
        }

        Ok(())
    }
}
