use ethers_core::types::transaction::eip712::Eip712Error;
use revm_primitives::{ExecutionResult, TxEnv};
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::TransactionType;

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
    ///
    #[error("Unable to Send Transaction to Sim: {0:#?}")]
    SendToSimError(SendError<TransactionType>),
    #[error("Unable to Create Runtime For ThreadPool")]
    RuntimeCreationError(std::io::Error),
    #[error("Unable to Start libmdbx transaction")]
    LibmdbxTransactionError(reth_db::mdbx::Error),
    #[error("Unable to read Revm-Reth Stateprovider Database")]
    RevmDatabaseError(reth_interfaces::Error),
}

impl From<SendError<TransactionType>> for SimError {
    fn from(value: SendError<TransactionType>) -> Self {
        SimError::SendToSimError(value)
    }
}

impl From<std::io::Error> for SimError {
    fn from(value: std::io::Error) -> Self {
        SimError::RuntimeCreationError(value)
    }
}

impl From<reth_db::mdbx::Error> for SimError {
    fn from(value: reth_db::mdbx::Error) -> Self {
        SimError::LibmdbxTransactionError(value)
    }
}

impl From<reth_interfaces::Error> for SimError {
    fn from(value: reth_interfaces::Error) -> Self {
        SimError::RevmDatabaseError(value)
    }
}
