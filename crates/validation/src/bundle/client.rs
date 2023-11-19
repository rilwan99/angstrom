use std::fmt::Debug;

use ethers_core::types::transaction::eip2718::TypedTransaction;
use guard_types::{
    primitive::{Angstrom::Bundle, ExternalStateSim},
    rpc::CallerInfo
};
use tokio::sync::{mpsc::UnboundedSender, oneshot::channel};

use crate::{
    bundle::{SimEvent, Simulator},
    bundle::errors::{SimError, SimResult}
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

    async fn simulate_external_state<T>(
        &self,
        hook_data: T,
        caller_info: CallerInfo
    ) -> Result<SimResult, SimError>
    where
        T: TryInto<ExternalStateSim> + Send,
        <T as TryInto<ExternalStateSim>>::Error: Debug
    {
        let (tx, rx) = channel();
        let hook = hook_data.try_into().unwrap();
        self.transaction_tx
            .send(SimEvent::Hook(hook, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn simulate_vanilla_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: Bundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::Bundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn simulate_composable_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: Bundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::MevBundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }
}
