use std::{collections::HashMap, future::Future, task::Context};

use ethers_core::types::{transactions::eip712::TypedData, Address, U256};
use futures::FuturesUnordered;
use shared::{SealedBundle, UserOrder};
use sim::Simulator;

pub struct SimulatedTransaction {
    typed_data:    TypedData,
    details:       UserOrder,
    /// true if we are swapping from pools token0 to token1
    direction:     bool,
    slots_touched: HashMap<B160, DbAccount>
}

impl SimulatedTransaction {
    // TODO: we need to choose a rational library
    pub fn limit_price(&self) -> u128 {
        self.details.amount_in / self.details.amount_out_min
    }

    pub fn gas_bid(&self) -> U256 {
        self.details.gas_bid
    }

    /// true if we are swapping from pools token0 to token1
    pub fn direction(&self) -> bool {
        self.direction
    }

    pub fn token_out(&self) -> Address {
        self.details.token_out
    }
}

#[derive(Debug, Clone)]
pub enum CowMsg {
    NewBestBundle(SealedBundle),
    NewValidTransactions(Vec<SimulatedTransaction>),
}


pub type SimFut = Pin<Box<dyn Future<Output = ()> + Send + Sync +'static>>;

pub struct CowSolver<S: Simulator + 'static> {
    all_valid_transactions: HashMap<Address, Vec<SimulatedTransaction>>,
    best_simed_bundle:      Option<SealedBundle>,
    sim:                    S,

    pending_simulations: FuturesUnordered<SimFut>
}

impl CowSolver {
    pub fn best_bundle(&self) -> &SealedBundle {
        &self.best_simed_bundle
    }

    // pub fn prune_included_transactions(&mut self, transactions: Vec<

    /// NOTICE: you need to verify that the sealed bundle already
    /// passed the simulation before trying to compare it. this
    /// is to guarantee no memory slot collusion
    pub fn new_bundle(&mut self, bundle: SealedBundle) -> bool {
        // TODO: this tech works because all we care is having the best gas bid
        if bundle.gas_bid_sum() > self.best_simed_bundle.gas_bid_sum() {
            self.best_simed_bundle = Some(bundle);

            return true
        }

        false
    }

    pub fn new_simmed_transaction(
        &mut self,
        transactions: HashMap<Address, Vec<SimulatedTransaction>>
    ) {
        /// TODO: does this deal with conflicts
        // self.all_valid_transactions.extend(transactions);
        transactions.into_iter().for_each(|(addr, txes)| {
            self.all_valid_transactions
                .entry(addr)
                .or_default()
                .extend(txes);
        });
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

impl<S: Simulator+ Unpin> Stream for CowSolver<S> {
    type Item = CowMsg;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}