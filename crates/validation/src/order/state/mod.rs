use std::{sync::Arc, task::Poll};

use alloy_primitives::{Address, B256};
use futures::{Stream, StreamExt};
use futures_util::stream::FuturesUnordered;
use angstrom_types::orders::{OrderValidationOutcome, PoolOrder};
use parking_lot::RwLock;
use reth_provider::StateProviderFactory;
use revm::db::BundleState;
use tokio::{
    sync::oneshot::Sender,
    task::{yield_now, JoinHandle}
};

use self::{
    orders::UserOrders,
    upkeepers::{Upkeepers, UserAccountDetails}
};
use super::OrderValidationRequest;
use crate::common::{executor::ThreadPool, lru_db::RevmLRU};

mod orders;
mod upkeepers;

/// State validation is all validation that requires reading from the Ethereum
/// database, these operations are:
/// 1) validating order nonce,
/// 2) checking token balances
/// 3) checking token approvals
/// 4) deals with possible pending state
#[allow(dead_code)]
pub struct StateValidation<DB> {
    db:          Arc<RevmLRU<DB>>,
    /// manage the upkeep of all data needed for validation
    /// all current user orders with the current changed deltas
    user_orders: UserOrders,

    /// upkeeps all state specific checks.
    upkeepers: Arc<RwLock<Upkeepers>>,

    // TODO: this should be a non-async runtime or add yield now points into the calculations
    thread_pool: ThreadPool,
    tasks:       FuturesUnordered<JoinHandle<(OrderValidationRequest, UserAccountDetails)>>
}

impl<DB> StateValidation<DB>
where
    DB: StateProviderFactory + Unpin + 'static
{
    pub fn new(_db: Arc<RevmLRU<DB>>) -> Self {
        todo!()
    }

    pub fn validate_non_composable_order(&mut self, order: OrderValidationRequest) {
        let db = self.db.clone();
        let keeper = self.upkeepers.clone();

        self.tasks
            .push(self.thread_pool.spawn_return_task_as(async move {
                match order {
                    OrderValidationRequest::ValidateLimit(tx, origin, o) => {
                        let (details, order) = keeper.read().verify_order(o, db);
                        (OrderValidationRequest::ValidateLimit(tx, origin, order), details)
                    }
                    OrderValidationRequest::ValidateSearcher(tx, origin, o) => {
                        let (details, order) = keeper.read().verify_order(o, db);
                        (OrderValidationRequest::ValidateSearcher(tx, origin, order), details)
                    }
                    _ => unreachable!()
                }
            }));
    }

    pub fn validate_composable_order(&mut self, order: OrderValidationRequest) {
        todo!()
    }

    pub fn filled_orders(&mut self, orders: Vec<B256>) {
        todo!()
    }

    pub fn eoa_state_changes(&mut self, eoas: Vec<Address>) {
        todo!()
    }

    fn on_task_resolve(&mut self, request: OrderValidationRequest, details: UserAccountDetails) {
        match request {
            OrderValidationRequest::ValidateLimit(tx, origin, order) => {
                let result = self.user_orders.new_limit_order(order, details);
                let _ = tx.send(result);
            }
            OrderValidationRequest::ValidateSearcher(tx, origin, order) => {
                let result = self.user_orders.new_searcher_order(order, details);
                let _ = tx.send(result);
            }
            _ => unreachable!()
        }
    }
}

impl<DB> Stream for StateValidation<DB>
where
    DB: StateProviderFactory + Unpin + 'static
{
    // if the task is a composable order, we stream it up
    type Item = ();

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(Ok((tx, details)))) = self.tasks.poll_next_unpin(cx) {
            self.on_task_resolve(tx, details);
        }

        Poll::Pending
    }
}
