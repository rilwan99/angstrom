use std::fmt::Debug;

use errors::{SimError, SimResult};
use ethers_core::types::{transaction::eip2718::TypedTransaction, I256, U256};
use guard_types::{
    primitive::{Angstrom::Bundle, ExternalStateSim},
    rpc::{CallerInfo, SignedLimitOrder}
};
use tokio::sync::{mpsc::unbounded_channel, oneshot::Sender};

pub mod bundle_validator;
pub mod errors;

#[derive(Debug)]
pub enum BundleOrTransactionResult {
    /// We just return the bundle as we don't care about gas usage but rather
    /// it finishes execution
    Bundle(Bundle),
    /// We just return the bundle as we don't care about gas usage but rather
    /// it finishes execution
    MevBundle(Bundle),
    HookSimResult {
        tx:            SignedLimitOrder,
        pre_hook_gas:  U256,
        post_hook_gas: U256
    },
    UniswapV4Results {
        delta: I256,
        gas:   U256
    }
}

// the simulator is a handle that we use to simulate transactions.
#[async_trait::async_trait]
pub trait BundleValidator: Send + Sync + Clone + Unpin {
    /// executes the swap on the underlying v4 pool in order to see what the
    /// limit price for everyone will be
    async fn validate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError>;
    /// executes the pre and post hook for the transactions to get the slots
    /// they touched and the cumulative gas that the pre and post hook use.
    /// this also checks to make sure we have enough value to execute on
    /// angstrom given there specifed amount in. we then for post hook give
    /// them there limit price they specifed and simulate that.
    async fn validate_external_state<T>(
        &self,
        hook_data: T,
        caller_info: CallerInfo
    ) -> Result<SimResult, SimError>
    where
        T: TryInto<ExternalStateSim> + Send,
        <T as TryInto<ExternalStateSim>>::Error: Debug;

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn validate_vanilla_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: Bundle
    ) -> Result<SimResult, SimError>;

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn validate_composable_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: Bundle
    ) -> Result<SimResult, SimError>;
}

/// enum of transaction type
pub enum BundleSimRequest {
    Hook(ExternalStateSim, CallerInfo, Sender<SimResult>),
    UniswapV4(TypedTransaction, Sender<SimResult>),
    Bundle(Bundle, CallerInfo, Sender<SimResult>),
    MevBundle(Bundle, CallerInfo, Sender<SimResult>),
    NewBlock(Sender<SimResult>)
}
