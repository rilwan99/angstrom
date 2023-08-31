use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_core::types::{Address, U256};
use futures::{stream::FuturesUnordered, Stream};
use revm::db::DbAccount;
use revm_primitives::*;
use shared::{Batch, Eip712, UserSettlement};
use sim::Simulator;

#[derive(Debug, Clone)]
pub struct SimulatedTransaction {
    pub transaction:   Eip712,
    pub details:       UserSettlement,
    /// true if we are swapping from pools token0 to token1
    pub direction:     bool,
    pub slots_touched: HashMap<B160, DbAccount>
}

impl SimulatedTransaction {
    // TODO: we need to choose a rational library
    pub fn limit_price(&self) -> u128 {
        self.details.order.amount_in / self.details.order.amount_out_min
    }

    pub fn gas_bid(&self) -> U256 {
        self.details.order.gas_cap
    }

    /// true if we are swapping from pools token0 to token1
    pub fn direction(&self) -> bool {
        self.direction
    }

    pub fn token_out(&self) -> Address {
        self.details.order.token_out
    }
}

#[derive(Debug, Clone)]
pub enum CowMsg {
    NewBestBundle(Batch),
    NewTransactions(Arc<Vec<Eip712>>)
}

pub type SimFut = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>;

pub struct CowSolver<S: Simulator + 'static> {
    all_valid_transactions: HashMap<Address, Vec<SimulatedTransaction>>,
    best_simed_bundle:      Option<Batch>,
    sim:                    S,

    pending_simulations: FuturesUnordered<SimFut>
}

impl<S: Simulator + 'static> CowSolver<S> {
    pub fn new(sim: S) -> Self {
        Self {
            sim,
            all_valid_transactions: HashMap::default(),
            pending_simulations: FuturesUnordered::default(),
            best_simed_bundle: None
        }
    }

    pub fn best_bundle(&self) -> Option<&Batch> {
        self.best_simed_bundle.as_ref()
    }

    /// NOTICE: you need to verify that the sealed bundle already
    /// passed the simulation before trying to compare it. this
    /// is to guarantee no memory slot collusion
    pub fn new_bundle(&mut self, bundle: Batch) -> bool {
        // TODO: this tech works because all we care is having the best gas bid
        false
    }

    /// TODO: does this deal with conflicts
    /// self.all_valid_transactions.extend(transactions);
    pub fn new_transactions(&mut self, transactions: Vec<Eip712>) {
        // transactions.into_iter().for_each(|(addr, txes)| {
        //     self.all_valid_transactions
        //         .entry(addr)
        //         .or_default()
        //         .extend(txes);
        // });
    }

    /// TODO: this assumes full cow of transactions and doesn't account for
    /// reverting down to cfmm
    /// on commit reveal we will know what the base price will be for every
    /// pool and can prune transactions where there limit price was violated
    pub fn prune_reveal(&mut self, prices: HashMap<Address, (u128, u128)>) {
        prices
            .into_iter()
            .for_each(|(address, (zto_price, otz_price))| {
                if let Some(simed_txes) = self.all_valid_transactions.get_mut(&address) {
                    simed_txes.retain(|tx| {
                        if tx.direction() {
                            tx.limit_price() > zto_price
                        } else {
                            tx.limit_price() > otz_price
                        }
                    });
                }
            })
    }
}

impl<S: Simulator + Unpin> Stream for CowSolver<S> {
    type Item = CowMsg;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Pending
    }
}
