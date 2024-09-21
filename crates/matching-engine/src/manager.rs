use std::collections::{HashMap, HashSet};

use angstrom_types::{
    consensus::PreProposal,
    orders::PoolSolution,
    primitive::PoolId,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        rpc_orders::TopOfBlockOrder
    }
};
use futures_util::FutureExt;
use reth_tasks::TaskSpawner;
use tokio::{
    sync::{
        mpsc::{Receiver, Sender},
        oneshot
    },
    task::JoinSet
};

use crate::{
    book::OrderBook,
    build_book,
    strategy::{MatchingStrategy, SimpleCheckpointStrategy},
    MatchingEngineHandle
};

pub enum MatcherCommand {
    BuildProposal(Vec<PreProposal>, oneshot::Sender<Result<Vec<PoolSolution>, String>>)
}

#[derive(Debug, Clone)]
pub struct MatcherHandle {
    pub sender: Sender<MatcherCommand>
}

impl MatcherHandle {
    async fn send(&self, cmd: MatcherCommand) {
        let _ = self.sender.send(cmd).await;
    }

    async fn send_request<T>(&self, rx: oneshot::Receiver<T>, cmd: MatcherCommand) -> T {
        self.send(cmd).await;
        rx.await.unwrap()
    }
}

impl MatchingEngineHandle for MatcherHandle {
    fn solve_pools(
        &self,
        preproposals: Vec<PreProposal>
    ) -> futures_util::future::BoxFuture<Result<Vec<PoolSolution>, String>> {
        Box::pin(async move {
            let (tx, rx) = oneshot::channel();
            self.send_request(rx, MatcherCommand::BuildProposal(preproposals, tx))
                .await
        })
    }
}

pub struct MatchingManager {}

impl MatchingManager {
    pub fn spawn<TP: TaskSpawner>(tp: TP) -> MatcherHandle {
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        let fut = manager_thread(rx).boxed();
        tp.spawn_critical("matching_engine", fut);

        MatcherHandle { sender: tx }
    }

    pub fn orders_by_pool_id(
        preproposals: &[PreProposal]
    ) -> HashMap<PoolId, HashSet<OrderWithStorageData<GroupedVanillaOrder>>> {
        preproposals
            .iter()
            .flat_map(|p| p.limit.iter())
            .cloned()
            .fold(HashMap::new(), |mut acc, order| {
                acc.entry(order.pool_id).or_default().insert(order);
                acc
            })
    }

    pub fn build_books(preproposals: &[PreProposal]) -> Vec<OrderBook> {
        // Pull all the orders out of all the preproposals and build OrderPools out of
        // them.  This is ugly and inefficient right now
        let book_sources = Self::orders_by_pool_id(preproposals);

        book_sources
            .into_iter()
            .map(|(id, orders)| {
                let amm = None;
                build_book(id, amm, orders)
            })
            .collect()
    }

    pub async fn build_proposal(
        &self,
        preproposals: Vec<PreProposal>
    ) -> Result<Vec<PoolSolution>, String> {
        // Pull all the orders out of all the preproposals and build OrderPools out of
        // them.  This is ugly and inefficient right now
        let books = Self::build_books(&preproposals);

        let searcher_orders: HashMap<PoolId, OrderWithStorageData<TopOfBlockOrder>> = preproposals
            .iter()
            .flat_map(|p| p.searcher.iter())
            .fold(HashMap::new(), |mut acc, order| {
                acc.entry(order.pool_id).or_insert(order.clone());
                acc
            });

        let mut solution_set = JoinSet::new();
        books.into_iter().for_each(|b| {
            let searcher = searcher_orders.get(&b.id()).cloned();
            // Using spawn-blocking here is not BAD but it might be suboptimal as it allows
            // us to spawn many more tasks that the CPu has threads.  Better solution is a
            // dedicated threadpool and some suggest the `rayon` crate.  This is probably
            // not a problem while I'm testing, but leaving this note here as it may be
            // important for future efficiency gains
            solution_set.spawn_blocking(move || {
                SimpleCheckpointStrategy::run(&b).map(|s| s.solution(searcher))
            });
        });
        let mut solutions = Vec::new();
        while let Some(res) = solution_set.join_next().await {
            if let Ok(Some(r)) = res {
                solutions.push(r);
            }
        }

        Ok(solutions)
    }
}

pub async fn manager_thread(mut input: Receiver<MatcherCommand>) {
    let manager = MatchingManager {};

    while let Some(c) = input.recv().await {
        match c {
            MatcherCommand::BuildProposal(p, r) => {
                r.send(manager.build_proposal(p).await).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use alloy::primitives::FixedBytes;
    use angstrom_types::consensus::PreProposal;
    use testing_tools::type_generator::consensus::preproposal::PreproposalBuilder;

    use super::MatchingManager;

    #[tokio::test]
    async fn can_build_proposal() {
        let manager = MatchingManager {};
        let preproposals = vec![];
        let _ = manager.build_proposal(preproposals).await.unwrap();
    }

    #[tokio::test]
    async fn will_combine_preproposals() {
        let manager = MatchingManager {};
        let preproposals: Vec<PreProposal> = (0..3)
            .map(|_| {
                PreproposalBuilder::new()
                    .order_count(10)
                    .for_random_pools(1)
                    .for_block(100)
                    .build()
            })
            .collect();
        let existing_orders: HashSet<FixedBytes<32>> = preproposals
            .iter()
            .flat_map(|p| p.limit.iter().map(|o| o.order_id.hash))
            .collect();
        let res = manager.build_proposal(preproposals).await.unwrap();
        let orders_in_solution: HashSet<FixedBytes<32>> = res
            .iter()
            .flat_map(|p| p.limit.iter().map(|o| o.id.hash))
            .collect();
        // Check to see if we have differences
        let diff = existing_orders
            .difference(&orders_in_solution)
            .collect::<Vec<_>>();
        if !diff.is_empty() {
            println!("Diff is {}", diff.len())
        }
        assert!(existing_orders == orders_in_solution, "Some orders vanished!");
    }
}
