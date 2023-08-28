use ethers_core::types::transaction::eip2718::TypedTransaction;
use thiserror::Error;
use revm_primitives::ExecutionResult;


/// CLEAN THIS UP
pub enum SimResult {
    /// error running a sim on the evm
    SimulationError(SimError),
    /// execution result of the sim
    ExecutionResult(ExecutionResult),
    /// successful bundle sim
    SuccessfulBundle
}


/// errors for sim
/// CHANGE TO EIP712DOMAIN
#[derive(Debug, Error)]
pub enum SimError {
    #[error("No Transactions in Bundle: {0:#?}")]
    NoTransactionsInBundle(TypedTransaction),
    #[error("Create Transaction Error: {0:#?}")]
    CreateTransaction(TypedTransaction),
    #[error("EVM Simulation Error: {0:#?}")]
    EVMTransactError(TypedTransaction),
}

