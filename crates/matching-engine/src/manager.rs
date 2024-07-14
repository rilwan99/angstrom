use std::collections::{hash_map::Entry, HashMap, HashSet};

use angstrom_types::consensus::{PreProposal, Proposal};
use futures_util::FutureExt;
use reth_tasks::TaskSpawner;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot
};

use crate::{book::OrderBook, build_book, MatchingEngineHandle};

pub enum MatcherCommand {
    BuildProposal(Vec<PreProposal>, oneshot::Sender<Result<Proposal, String>>)
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
    fn build_proposal(
        &self,
        preproposals: Vec<PreProposal>
    ) -> futures_util::future::BoxFuture<Result<Proposal, String>> {
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

    pub fn build_proposal(&self, preproposals: Vec<PreProposal>) -> Result<Proposal, String> {
        let mut book_sources = HashMap::new();
        // Pull all the orders out of all the preproposals and build OrderPools out of
        // them This is ugly and inefficient right now
        preproposals
            .iter()
            .flat_map(|p| p.limit.iter())
            .for_each(|o| match book_sources.entry(o.pool_id) {
                Entry::Vacant(e) => {
                    e.insert(HashSet::from([o.clone()]));
                }
                Entry::Occupied(mut v) => {
                    v.get_mut().insert(o.clone());
                }
            });

        let books: Vec<OrderBook> = book_sources
            .into_iter()
            .map(|(id, orders)| {
                let amm = None;
                build_book(id, amm, orders)
            })
            .collect();

        // let solutions = books
        //     .iter()
        //     .filter_map(|b| SimpleCheckpointStrategy::run(&b))
        //     .collect();
        // Use an AMM starting position and get to where we know the AMM is
        // Run our matching algo on all the pools
        // Put together a proposal of what we think should happen
        Ok(Proposal::default())
    }
}

pub async fn manager_thread(mut input: Receiver<MatcherCommand>) {
    let manager = MatchingManager {};

    while let Some(c) = input.recv().await {
        match c {
            MatcherCommand::BuildProposal(p, r) => {
                r.send(manager.build_proposal(p)).unwrap();
            }
        }
    }
}
