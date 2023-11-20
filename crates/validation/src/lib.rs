pub mod bundle;
pub mod common;
pub mod order;

use std::fmt::Debug;

use bundle::BundleValidator;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use guard_types::{
    orders::OrderOrigin,
    primitive::{Angstrom::Bundle, ExternalStateSim},
    rpc::{
        CallerInfo, EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder,
        EcRecoveredLimitOrder, EcRecoveredSearcherOrder
    }
};
use order::OrderValidator;
use tokio::sync::{mpsc::UnboundedSender, oneshot::channel};

use crate::{
    bundle::{
        errors::{SimError, SimResult},
        SimEvent
    },
    order::OrderValidationOutcome
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
impl OrderValidator for RevmClient {
    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder = EcRecoveredComposableLimitOrder;
    /// The transaction type of the composable searcher order pool
    type ComposableSearcherOrder = EcRecoveredComposableSearcherOrder;
    /// The order type of the limit order pool
    type LimitOrder = EcRecoveredLimitOrder;
    /// The transaction type of the searcher order pool
    type SearcherOrder = EcRecoveredSearcherOrder;

    async fn validate_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::LimitOrder
    ) -> OrderValidationOutcome<Self::LimitOrder> {
        todo!()
    }

    async fn validate_composable_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableLimitOrder
    ) -> OrderValidationOutcome<Self::ComposableLimitOrder> {
        todo!()
    }

    async fn validate_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::SearcherOrder
    ) -> OrderValidationOutcome<Self::SearcherOrder> {
        todo!()
    }

    async fn validate_composable_searcher_order(
        &self,
        origin: OrderOrigin,
        transaction: Self::ComposableSearcherOrder
    ) -> OrderValidationOutcome<Self::ComposableSearcherOrder> {
        todo!()
    }
}

/// Bundle Impl
#[async_trait::async_trait]
impl BundleValidator for RevmClient {
    //TODO: Fix this, to whitebox simulate the swap directly, because it isn't a
    // full transaction and should not be validated as such
    async fn validate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError> {
        let (sender, rx) = channel();
        self.transaction_tx.send(SimEvent::UniswapV4(tx, sender))?;

        Ok(rx.await.unwrap())
    }

    async fn validate_external_state<T>(
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

    /// validates the full bundle in order to make sure it is valid and passes
    async fn validate_vanilla_bundle(
        &self,
        caller_info: CallerInfo,
        bundle: Bundle
    ) -> Result<SimResult, SimError> {
        let (tx, rx) = channel();
        self.transaction_tx
            .send(SimEvent::Bundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }

    /// validates the full bundle in order to make sure it is valid and passes
    async fn validate_composable_bundle(
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
