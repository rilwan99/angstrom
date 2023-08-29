use ethers_core::types::transaction::eip712::Eip712Error;
use revm_primitives::{ExecutionResult, TxEnv};
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;

/// CLEAN THIS UP
#[derive(Debug)]
pub enum SimResult {
    /// error running a sim on the evm
    SimulationError(SimError),
    /// execution result of the sim
    ExecutionResult(ExecutionResult),
    /// successful bundle sim
    SuccessfulBundle,
}

/// errors for sim
/// CHANGE TO EIP712DOMAIN
#[derive(Debug, Error)]
pub enum SimError {
    #[error("No Transactions in Bundle: {0:#?}")]
    NoTransactionsInBundle(TxEnv),
    #[error("Create Transaction Error: {0:#?}")]
    CreateTransaction(TxEnv),
    #[error("EVM Simulation Error: {0:#?}")]
    EVMTransactError(TxEnv),
    #[error("Error Decoding EIP712 Transaction: {0:#?}")]
    Eip712Error(Eip712Error),
}
