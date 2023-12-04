use std::{any, collections::HashMap, fmt::Debug};

use alloy_primitives::{Address, U256};
use guard_types::orders::{
    OrderId, OrderLocation, OrderPriorityData, OrderValidationOutcome, PoolOrder, PooledLimitOrder,
    PooledSearcherOrder, SearcherPriorityData, StateValidationError, ValidatedOrder
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
    pub fn new_searcher_order<O: PooledSearcherOrder<ValidationData = SearcherPriorityData>>(
        &mut self,
        order: O,
        deltas: UserAccountDetails
    ) -> OrderValidationOutcome<O> {
        self.basic_order_validation(order, deltas, false, |order| SearcherPriorityData {
            donated: order.donated(),
            volume:  order.volume(),
            gas:     order.gas()
        })
    }

    pub fn new_limit_order<O: PooledLimitOrder<ValidationData = OrderPriorityData>>(
        &mut self,
        order: O,
        deltas: UserAccountDetails
    ) -> OrderValidationOutcome<O> {
        self.basic_order_validation(order, deltas, true, |order| OrderPriorityData {
            price:  order.limit_price(),
            volume: order.limit_price() * order.amount_out_min(),
            gas:    order.gas()
        })
    }

    /// called when a user has a state change on their address. When this
    /// happens we re-evaluate all of there pending orders so we do a
    /// hard-reset here.
    pub fn fresh_state(&mut self, state: HashMap<Address, PendingState>) {
        state.into_iter().for_each(|(k, v)| {
            self.0.insert(k, (v, vec![]));
        });
    }

    fn basic_order_validation<
        O: PoolOrder<ValidationData = Data>,
        Data: Send + Debug + Sync + Clone + Unpin + 'static,
        F: FnOnce(O) -> Data
    >(
        &mut self,
        order: O,
        deltas: UserAccountDetails,
        limit: bool,
        build_priority: F
    ) -> OrderValidationOutcome<O> {
        // always invalid
        if !deltas.is_valid_nonce
            || !deltas.is_valid_pool
            || self.has_nonce_overlap(&order.from(), &order.nonce())
        {
            return OrderValidationOutcome::Error(
                order.hash(),
                StateValidationError::InvalidNonce(order.hash(), order.nonce())
            )
        }

        let user = id.address;
        let (pending_state, ids) = self.0.entry(user).or_default();
        ids.push(id);

        // track which pool this should go into
        let mut has_balances = true;

        // first order so we init instead of apply deltas
        if pending_state.token_balances.is_empty() && pending_state.token_approvals.is_empty() {
            pending_state.token_balances = deltas.token_bals;
            pending_state.token_approvals = deltas.token_approvals;
        } else {
            // subtract token in from approval
            if let Some(token) = pending_state.token_approvals.get_mut(&order.token_in()) {
                if token.clone().checked_sub(order.amount_in()).is_none() {
                    has_balances = false;
                } else {
                    token -= order.amount_in();
                }
            } else {
                has_balances = false;
            }

            if has_balances {
                if let Some(token) = pending_state.token_balances.get_mut(&order.token_in()) {
                    if token.clone().checked_sub(order.amount_in()).is_none() {
                        // add balance back to approval
                        // NOTE: default will never be called here
                        *pending_state
                            .token_approvals
                            .entry(&order.token_in())
                            .or_default() += order.amount_in();

                        has_balances = false;
                    } else {
                        token -= order.amount_in();
                    }
                } else {
                    // NOTE: default will never be called here
                    *pending_state
                        .token_approvals
                        .entry(&order.token_in())
                        .or_default() += order.amount_in();

                    has_balances = false;
                }
            }
            // NOTE: because we can't guarentee the order of execution with
            // these orders, we cannot add the amount out balance to
            // token balances to allow multi hop with different
            // intents within a single transaction
        }

        let data = build_priority(order);

        let res = ValidatedOrder {
            order,
            data,
            is_bid: deltas.is_bid,
            pool_id: deltas.pool_id,
            location: if limit {
                if has_balances {
                    OrderLocation::LimitPending
                } else {
                    OrderLocation::LimitParked
                }
            } else {
                OrderLocation::VanillaSearcher
            }
        };

        Ok(OrderValidationOutcome::Valid { order: res, propagate: true })
    }

    /// Helper function for checking for duplicates when adding orders
    fn has_nonce_overlap(&self, address: &Address, id: &U256) -> bool {
        self.0
            .get(address)
            .map(|inner| inner.1.iter().any(|other_id| other_id == id))
            .unwrap_or(false)
    }
}
