use std::sync::Arc;

use alloy_primitives::Address;
use futures::{Stream, StreamExt};
use futures_util::stream::FuturesUnordered;
use guard_types::orders::{OrderValidationOutcome, PoolOrder};
use parking_lot::RwLock;
use reth_provider::StateProviderFactory;
use tokio::{sync::oneshot::Sender, task::JoinHandle};

use self::{
    orders::UserOrders,
    upkeepers::{Upkeepers, UserAccountDetails}
};
use crate::common::{executor::ThreadPool, lru_db::RevmLRU};

mod orders;
mod upkeepers;

/// State validation is all validation that requires reading from the Ethereum
/// database, these operations are:
/// 1) validating order nonce,
/// 2) checking token balances
/// 3) checking token approvals
#[allow(dead_code)]
pub struct StateValidation<DB> {
    db:          Arc<RevmLRU<DB>>,
    /// manage the upkeep of all data needed for validation
    /// all current user orders with the current changed deltas
    user_orders: UserOrders,

    /// upkeeps all state specific checks.
    upkeepers: Arc<RwLock<Upkeepers>>,

    thread_pool: ThreadPool,
    tasks: FuturesUnordered<
        JoinHandle<(
            Sender<OrderValidationOutcome<Box<dyn PoolOrder>>>,
            Box<dyn PoolOrder>,
            UserAccountDetails
        )>
    >
}

impl<DB> StateValidation<DB>
where
    DB: StateProviderFactory
{
    pub fn new(_db: Arc<RevmLRU<DB>>) -> Self {
        todo!()
    }

    pub fn validate_user_order<O: PoolOrder>(
        &mut self,
        tx: Sender<OrderValidationOutcome<Box<dyn PoolOrder>>>,
        order: O
    ) {
        let db = self.db.clone();
        let keeper = self.upkeepers.clone();
        self.tasks
            .push(self.thread_pool.spawn_return_task_as(async move {
                let (details, order) = keeper.read().verify_order(order.clone(), db);

                (tx, order, details)
            }));
    }

    pub fn validate_searcher_order<O: PoolOrder>(
        &mut self,
        tx: Sender<OrderValidationOutcome<Box<dyn PoolOrder>>>,
        order: O
    ) {
        let db = self.db.clone();
        let keeper = self.upkeepers.clone();
        self.tasks
            .push(self.thread_pool.spawn_return_task_as(async move {
                let (details, order) = keeper.read().verify_order(order.clone(), db);

                (tx, order, details)
            }));
    }

    pub fn new_block(&mut self) {
        todo!()
    }

    pub fn filled_orders(&mut self, orders: Vec<B256>) {
        todo!()
    }

    pub fn eoa_state_changes(&mut self, eoas: Vec<Address>) {
        todo!()
    }

    fn on_task_resolve(
        &mut self,
        tx: Sender<OrderValidationOutcome<Box<dyn PoolOrder>>>,
        order: O,
        details: UserAccountDetails
    ) {
        let res = self.user_orders.new_limit_order(order, details).unwrap();
        let _ = tx.send(res);
    }
}

impl<DB> Stream for StateValidation<DB> {
    type Item = ();

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Option<Self::Item>> {
        while let Poll::Ready(Some(Ok((tx, order, details)))) = self.tasks.poll_next_unpin(cx) {
            self.on_task_resolve(tx, order, details);
        }

        Poll::Pending
    }
}
