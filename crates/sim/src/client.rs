use std::fmt::Debug;

use anvil_core::eth::transaction::EthTransactionRequest;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use guard_types::on_chain::{ComposableBundle, VanillaBundle};
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

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn simulate_vanilla_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: VanillaBundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::VanillaBundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }

    /// simulates the full bundle in order to make sure it is valid and passes
    async fn simulate_composable_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: ComposableBundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::ComposableBundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }
}
