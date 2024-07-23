use std::collections::{HashMap, HashSet};

use angstrom_types::{
    consensus::PreProposal,
    orders::PoolSolution,
    sol_bindings::{
        grouped_orders::{GroupedVanillaOrder, OrderWithStorageData},
        sol::TopOfBlockOrder
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

    pub async fn build_proposal(
        &self,
        preproposals: Vec<PreProposal>
    ) -> Result<Vec<PoolSolution>, String> {
        // Pull all the orders out of all the preproposals and build OrderPools out of
        // them.  This is ugly and inefficient right now
        let book_sources: HashMap<usize, HashSet<OrderWithStorageData<GroupedVanillaOrder>>> =
            preproposals
                .iter()
                .flat_map(|p| p.limit.iter())
                .cloned()
                .fold(HashMap::new(), |mut acc, order| {
                    acc.entry(order.pool_id).or_default().insert(order);
                    acc
                });

        let searcher_orders: HashMap<usize, OrderWithStorageData<TopOfBlockOrder>> = preproposals
            .iter()
            .flat_map(|p| p.searcher.iter())
            .fold(HashMap::new(), |mut acc, order| {
                acc.entry(order.pool_id).or_insert(order.clone());
                acc
            });

        let books: Vec<OrderBook> = book_sources
            .into_iter()
            .map(|(id, orders)| {
                let amm = None;
                build_book(id, amm, orders)
            })
            .collect();
        let mut solution_set = JoinSet::new();
        books.into_iter().for_each(|b| {
            let searcher = searcher_orders.get(&b.id()).unwrap().clone();
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
