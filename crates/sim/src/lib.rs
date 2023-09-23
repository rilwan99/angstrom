use std::fmt::Debug;

use client::RevmClient;
use errors::{SimError, SimResult};
use ethers_core::types::{transaction::eip2718::TypedTransaction, I256, U256};
use executor::ThreadPool;
use guard_types::on_chain::{
    CallerInfo, HookSim, RawBundle, RawLvrSettlement, RawUserSettlement, SearcherOrUser,
    SimmedBundle, SimmedLvrSettlement, SimmedUserSettlement
};
use revm_primitives::db::DatabaseRef;
use state::RevmBackend;
use tokio::sync::{mpsc::unbounded_channel, oneshot::Sender};

use crate::revm::Revm;

pub mod anvil;
pub mod client;
pub mod errors;
pub mod executor;
pub mod lru_db;
pub mod revm;
pub mod slot_keeper;
pub mod state;

pub fn spawn_revm_sim(db: lru_db::RevmLRU) -> Result<RevmClient, SimError> {
    let (tx, rx) = unbounded_channel();
    std::thread::spawn(move || {
        let revm_client = Revm::new(rx, db).unwrap();
        let handle = revm_client.get_threadpool_handle();

        handle.block_on(revm_client);
    });

    Ok(RevmClient::new(tx))
}

#[derive(Debug)]
pub enum BundleOrTransactionResult {
    Bundle(SimmedBundle),
    HookSimResult { tx: SearcherOrUser, pre_hook_gas: U256, post_hook_gas: U256 },
    UniswapV4Results { delta: I256, gas: U256 }
}

// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait Simulator: Send + Sync + Clone + Unpin {
    /// executes the swap on the underlying v4 pool in order to see what the
    /// limit price for everyone will be
    async fn simulate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError>;
    /// executes the pre and post hook for the transactions to get the slots
    /// they touched and the cumulative gas that the pre and post hook use.
    /// this also checks to make sure we have enough value to execute on
    /// angstrom given there specifed amount in. we then for post hook give
    /// them there limit price they specifed and simulate that.
    async fn simulate_hooks<T>(
        &self,
        hook_data: T,
        caller_info: CallerInfo
    ) -> Result<SimResult, SimError>
    where
        T: TryInto<HookSim> + Send,
        <T as TryInto<HookSim>>::Error: Debug;

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn simulate_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: RawBundle
    ) -> Result<SimResult, SimError>;
}

/// enum of transaction type
pub enum SimEvent {
    Hook(HookSim, CallerInfo, Sender<SimResult>),
    UniswapV4(TypedTransaction, Sender<SimResult>),
    BundleTx(RawBundle, CallerInfo, Sender<SimResult>),
    NewBlock(Sender<SimResult>)
}
