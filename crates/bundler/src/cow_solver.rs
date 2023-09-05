use std::{
    collections::{HashMap, HashSet},
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use futures::{stream::FuturesUnordered, Stream, StreamExt};
use revm::primitives::B160;
use shared::{
    CallerInfo, RawBundle, RawLvrSettlement, RawUserSettlement, SimmedBundle, SimmedLvrSettlement,
    SimmedUserSettlement
};
use sim::{
    errors::{SimError, SimResult},
    Simulator
};
use tracing::info;

#[derive(Debug, Clone)]
pub enum CowMsg {
    NewBestBundle(Arc<SimmedBundle>),
    NewUserTransactions(Arc<Vec<SimmedUserSettlement>>),
    NewSearcherTransactions(Arc<Vec<SimmedLvrSettlement>>)
}

pub type SimFut = Pin<Box<dyn Future<Output = Result<SimResult, SimError>> + Send + 'static>>;

/// Handles what is the currently best bundle and tries
/// to beat it
pub struct CowSolver<S: Simulator + 'static> {
    // don't care ab this for now, should be mapped to pool
    _all_user_txes:    HashSet<SimmedUserSettlement>,
    _best_searcher_tx: HashSet<SimmedLvrSettlement>,

    best_simed_bundle:   Option<SimmedBundle>,
    sim:                 S,
    // tmp
    call_info:           CallerInfo,
    pending_simulations: FuturesUnordered<SimFut>
}

impl<S: Simulator + 'static> CowSolver<S> {
    pub fn new(sim: S) -> Self {
        Self {
            sim,
            _best_searcher_tx: HashSet::default(),
            _all_user_txes: HashSet::default(),
            pending_simulations: FuturesUnordered::default(),
            best_simed_bundle: None,
            call_info: CallerInfo {
                address:   B160::default(),
                nonce:     69,
                overrides: HashMap::new()
            }
        }
    }

    pub fn new_block(&mut self) {
        self.best_simed_bundle = None;
    }

    pub fn best_bundle(&self) -> Option<&SimmedBundle> {
        self.best_simed_bundle.as_ref()
    }

    pub fn new_bundle(&mut self, bundle: RawBundle) {
        let handle = self.sim.clone();
        let call_info = self.call_info.clone();
        self.pending_simulations
            .push(Box::pin(async move { handle.simulate_bundle(call_info, bundle).await }));
    }

    pub fn new_searcher_transactions(&mut self, tx: Vec<RawLvrSettlement>) {
        tx.into_iter().for_each(|tx| {
            let handle = self.sim.clone();
            let call_info = self.call_info.clone();
            // self.pending_simulations
            //     .push(Box::pin(async move {
            // handle.simulate_searcher_tx(call_info, tx).await }));
        });
    }

    pub fn new_user_transactions(&mut self, transactions: Vec<RawUserSettlement>) {
        // TODO: Pretty retarded but works for now
        transactions.into_iter().for_each(|tx| {
            let handle = self.sim.clone();
            let call_info = self.call_info.clone();
            // self.pending_simulations
            //     .push(Box::pin(async move {
            // handle.simulate_user_tx(call_info, tx).await }));
        });
    }

    fn on_sim_res(&mut self, sim_results: Vec<Result<SimResult, SimError>>) -> Option<Vec<CowMsg>> {
        info!(?sim_results);
        None
        // let mut new_best = false;
        // TODO: Ugly as shit. fix this
        // let new_transaction = sim_results
        //     .into_iter()
        //     .filter_map(|sim_result| {
        //         match sim_result {
        //             Ok(res) => match res {
        //                 SimResult::ExecutionResult(res) => match res {
        //                     BundleOrTransactionResult::Bundle(bundle) => {
        //                         debug!(?bundle, ?res, "simmed bundle res");
        //                         if let Some(current_best) =
        // self.best_simed_bundle.as_ref() {
        // if bundle.is_more_profitable(current_best) {
        // self.best_simed_bundle = Some(bundle.clone());
        // new_best = true;                             }
        //                         } else if bundle.result.is_success() {
        //                             self.best_simed_bundle =
        // Some(bundle.clone());                             new_best =
        // true                         }
        //                     }
        //
        //                     BundleOrTransactionResult::Transaction(tx) => {
        //                         debug!(?tx, ?res, "simmed transaction res");
        //                         if !self.all_valid_transactions.contains(&tx)
        // {                             let inner =
        // tx.transaction.clone();
        // self.all_valid_transactions.insert(tx);
        // return Some(inner)                         }
        //                     }
        //                 },
        //                 SimResult::SuccessfulRevmBlockUpdate => {
        //                     trace!("simulator updated");
        //                 }
        //                 _ => todo!("placeholder")
        //             },
        //             Err(e) => {
        //                 error!(io_error=?e, "sim had a io error");
        //                 panic!();
        //             }
        //         }
        //         None
        //     })
        //     .collect::<Vec<_>>();
        //
        // // TODO: clean this up
        // if !new_transaction.is_empty() && new_best {
        //     return Some(vec![
        //         CowMsg::NewTransactions(new_transaction.into()),
        //         CowMsg::NewBestBundle(self.best_bundle().unwrap().clone().
        // into()),     ])
        // } else if !new_transaction.is_empty() && new_best {
        //     return
        // Some(vec![CowMsg::NewBestBundle(self.best_bundle().unwrap().clone().
        // into())]) }
        //
        // None
    }
}

impl<S: Simulator + Unpin> Stream for CowSolver<S> {
    type Item = Vec<CowMsg>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut resolved_sims = Vec::new();
        while let Poll::Ready(Some(sim_res)) = self.pending_simulations.poll_next_unpin(cx) {
            resolved_sims.push(sim_res);
        }

        if let Some(res) = self.on_sim_res(resolved_sims) {
            return Poll::Ready(Some(res))
        }

        Poll::Pending
    }
}

#[cfg(feature = "test_harness")]
pub mod test_harness {
    use shared::RawBundle;

    use super::*;

    impl<S: Simulator + 'static> CowSolver<S> {
        /// takes our current best bundle. and then runs it through the sim.
        /// because we don't have a current best. it should then be propagated
        /// through the network since this is our best bundle
        pub fn propagate_bundle(&mut self) {
            let bundle: RawBundle = self.best_simed_bundle.take().unwrap().into();
            self.new_bundle(bundle);
        }

        pub fn propagate_user_transactions(&mut self) {
            let user_txes = self
                ._all_user_txes
                .drain()
                .map(Into::into)
                .collect::<Vec<_>>();

            self.new_user_transactions(user_txes);
        }

        pub fn propagate_searcher_transactions(&mut self) {
            let searcher_txes = self
                ._best_searcher_tx
                .drain()
                .map(Into::into)
                .collect::<Vec<_>>();

            self.new_searcher_transactions(searcher_txes)
        }

        /// does a check on the raw bundle. this is to avoid sim discrepancies
        pub fn check_for_bundle(&self, hash: H256) -> bool {
            // autistic lets not use into trait because it consumes
            let Some(bundle) = self.best_simed_bundle.clone() else { return false };

            return hash == (<SimmedBundle as Into<RawBundle>>::into(bundle)).into()
        }

        pub fn check_for_user_tx(&self, hash: H256) -> bool {
            self._all_user_txes.iter().any(|tx| {
                let raw: RawUserSettlement = tx.clone().into();
                hash == raw.into()
            })
        }

        pub fn check_for_searcher_tx(&self, hash: H256) -> bool {
            self._best_searcher_tx.iter().any(|tx| {
                let raw: RawLvrSettlement = tx.clone().into();
                hash == raw.into()
            })
        }
    }
}
