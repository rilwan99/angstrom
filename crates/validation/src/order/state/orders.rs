use std::fmt::Debug;

use alloy_primitives::{Address, U256};
use guard_types::orders::{
    OrderId, OrderLocation, OrderPriorityData, OrderValidationOutcome, PoolOrder,
    SearcherPriorityData, ValidatedOrder
};
use revm::primitives::HashMap;

use super::upkeepers::UserAccountDetails;

/// the sum of all pending orders for a given user. This is done
/// so that validation of specific orders is not dependant on all other orders.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PendingState {
    token_balances:  HashMap<Address, U256>,
    token_approvals: HashMap<Address, U256>
}

pub struct UserOrders(HashMap<Address, (PendingState, Vec<U256>)>);

impl UserOrders {
    #[allow(dead_code)]
    pub fn new_searcher_order<O: PoolOrder<ValidationData = SearcherPriorityData>>(
        &mut self,
        order: O,
        deltas: UserAccountDetails
    ) -> Result<OrderValidationOutcome<O>, ()> {
        Ok(self.basic_order_validation(order, deltas, false, |order, deltas| todo!()))
    }

    pub fn new_limit_order<O: PoolOrder<ValidationData = OrderPriorityData>>(
        &mut self,
        order: O,
        deltas: UserAccountDetails
    ) -> Result<OrderValidationOutcome<O>, ()> {
        Ok(self.basic_order_validation(order, deltas, true, |order, deltas| todo!()))
    }

    fn basic_order_validation<
        O: PoolOrder<ValidationData = Data>,
        Data: Send + Debug + Sync + Clone + Unpin + 'static
    >(
        &mut self,
        order: O,
        deltas: UserAccountDetails,
        limit: bool,
        build_priority: FnOnce(O, UserAccountDetails) -> Data
    ) -> OrderValidationOutcome<O> {
        let _ = self.check_for_nonce_overlap(&order.from(), &order.nonce())?;

        // bad order
        if !deltas.is_valid_nonce || !deltas.is_valid_pool {
            return Err(())
        }

        let user = id.address;
        let (pending_state, ids) = self.0.entry(user).or_default();
        ids.push(id);

        // first order so we init instead of apply deltas
        if pending_state.token_balances.is_empty() && pending_state.token_approvals.is_empty() {
            pending_state.token_balances = deltas.token_bals;
            pending_state.token_approvals = deltas.token_approvals;
        } else {
            // subtract token in from approval
            if let Some(token) = pending_state.token_approvals.get_mut(&order.token_in()) {
                if token.clone().checked_sub(order.amount_in()).is_none() {
                    // TODO: not enough balance
                }
                token -= order.amount_in();
            } else {
                //TODO: error, no approval
            }

            if let Some(token) = pending_state.token_balances.get_mut(&order.token_in()) {
                if token.clone().checked_sub(order.amount_in()).is_none() {
                    // add balance back to approval
                    // NOTE: default will never be called here
                    *pending_state
                        .token_approvals
                        .entry(&order.token_in())
                        .or_default() += order.amount_in();
                    //TODO: error, not enough balance,
                }
                token -= order.amount_in();
            } else {
                // NOTE: default will never be called here
                *pending_state
                    .token_approvals
                    .entry(&order.token_in())
                    .or_default() += order.amount_in();
                // TODO: error, no balance, add amount back to approval
            }

            // NOTE: because we can't guarentee the order of execution with
            // these orders, we cannot add the amount out balance to
            // token balances to allow multi hop with different
            // intents within a single transaction
        }

        let data = build_priority(order, deltas);

        let res = ValidatedOrder {
            order,
            data,
            is_bid: deltas.is_bid,
            pool_id: deltas.pool_id,
            location: if limit {
                OrderLocation::LimitPending
            } else {
                OrderLocation::VanillaSearcher
            }
        };

        Ok(OrderValidationOutcome::Valid { order: res, propagate: true })
    }

    /// Helper function for checking for duplicates when adding orders
    #[allow(dead_code)]
    fn check_for_nonce_overlap(&self, address: &Address, id: &U256) -> Result<(), ()> {
        if self
            .0
            .get(address)
            .map(|inner| inner.1.iter().any(|other_id| other_id == id))
            .unwrap_or(false)
        {
            // return Err(PoolError::DuplicateNonce(id.clone()))
        }

        Ok(())
    }
}
