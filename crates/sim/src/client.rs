use std::fmt::Debug;

use anvil_core::eth::transaction::EthTransactionRequest;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use shared::*;
use tokio::sync::{mpsc::UnboundedSender, oneshot::channel};

use crate::{
    errors::{SimError, SimResult},
    SimEvent, Simulator
};

/// clone-able handle to the simulator
#[derive(Clone)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<SimEvent>
}

impl RevmClient {
    pub fn new(transaction_tx: UnboundedSender<SimEvent>) -> Self {
        Self { transaction_tx }
    }
}

#[async_trait::async_trait]
impl Simulator for RevmClient {
    async fn simulate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError> {
        let (sender, rx) = channel();
        self.transaction_tx.send(SimEvent::UniswapV4(tx, sender))?;

        Ok(rx.await.unwrap())
    }

    async fn simulate_hooks<T>(
        &self,
        hook_data: T,
        caller_info: CallerInfo
    ) -> Result<SimResult, SimError>
    where
        T: TryInto<HookSim> + Send,
        <T as TryInto<HookSim>>::Error: Debug
    {
        let (tx, rx) = channel();
        let hook = hook_data.try_into().unwrap();
        self.transaction_tx
            .send(SimEvent::Hook(hook, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }

    async fn simulate_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: RawBundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::BundleTx(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }
}
