use ethers_core::types::transaction::eip712::{Eip712Error, TypedData};
use reth_primitives::Signature;
use revm_primitives::{ExecutionResult, TxEnv};
use shared::{Eip712, UserSettlement};
use tokio::sync::mpsc::error::SendError;

use crate::TransactionType;

#[derive(Debug)]
pub enum SimResult {
    /// error running a sim on the evm
    SimError(SimError),
    /// execution result of the sim
    ExecutionResult(ExecutionResult)
}

/// errors for sim
#[derive(Debug)]
pub enum SimError {
    ///
    //#[error("Unable to Send Transaction to Sim: {0:#?}")]
    SendToSimError(SendError<TransactionType>),
    //#[error("Unable to Create Runtime For ThreadPool")]
    RuntimeCreationError(std::io::Error),
    //#[error("Unable to Start libmdbx transaction")]
    LibmdbxTransactionError(reth_db::mdbx::Error),
    //#[error("Unable to read Revm-Reth StateProvider Database")]
    RevmDatabaseError(reth_interfaces::Error),
    //#[error("No Transactions Decoded from EIP712 Tx: {0:#?}")]
    NoTransactionsInEip712(Eip712),
    //#[error("EVM Simulation Error: {0:#?}")]
    RevmEVMTransactionError(TxEnv),
    //#[error("Call instead of create transaction: {0:#?}")]
    CallInsteadOfCreateError(TxEnv),
    //#[error("Error Encoding EIP712 Transaction: {0:#?}")]
    Eip712EncodingError(Eip712Error),
    //#[error("Error Decoding EIP712 Transaction: {0:#?}")]
    Eip712DecodingError(TypedData),
    //#[error("Error Decoding EIP712 Transaction With Serde: {0:#?}")]
    SerdeEip712DecodingError(serde_json::error::Error),
    //#[error("No verifying contract on EIP712 transaction: {0:#?}")]
    NoVerifyingContract(Eip712),
    //#[error("Error decoding signature: {0:#?}")]
    DecodingSignatureError(UserSettlement),
    // #[error("Error decoding signature: {0:#?}")]
    RecoveringSignerError(Signature)
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

impl From<Eip712Error> for SimError {
    fn from(value: Eip712Error) -> Self {
        SimError::Eip712EncodingError(value)
    }
}

impl From<TypedData> for SimError {
    fn from(value: TypedData) -> Self {
        SimError::Eip712DecodingError(value)
    }
}

impl From<serde_json::error::Error> for SimError {
    fn from(value: serde_json::error::Error) -> Self {
        SimError::SerdeEip712DecodingError(value)
    }
}

impl From<Signature> for SimError {
    fn from(value: Signature) -> Self {
        SimError::RecoveringSignerError(value)
    }
}
