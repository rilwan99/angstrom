use std::fmt::Debug;

use ethers_core::types::transaction::eip2718::TypedTransaction;
use guard_types::{
    orders::OrderOrigin,
    primitive::{Angstrom::Bundle, ExternalStateSim},
    rpc::{
        CallerInfo, EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder,
        EcRecoveredLimitOrder, EcRecoveredSearcherOrder
    }
};
use tokio::sync::{mpsc::UnboundedSender, oneshot::channel};

use crate::{
    bundle::{
        errors::{SimError, SimResult},
        BundleSimRequest, BundleValidator
    },
    order::{OrderValidator, ValidationFuture}
};
/// clone-able handle to the simulator
#[derive(Clone, Debug)]
pub struct RevmClient {
    transaction_tx: UnboundedSender<BundleSimRequest>
}

impl RevmClient {
    pub fn new(transaction_tx: UnboundedSender<BundleSimRequest>) -> Self {
        Self { transaction_tx }
    }
}

impl OrderValidator for RevmClient {
    /// The transaction type of the composable limit order pool
    type ComposableLimitOrder = EcRecoveredComposableLimitOrder;
    /// The transaction type of the composable searcher pool
    type ComposableSearcherOrder = EcRecoveredSearcherOrder;
    /// The order type of the limit order pool
    type LimitOrder = EcRecoveredLimitOrder;
    /// The transaction type of the searcher order pool
    type SearcherOrder = EcRecoveredComposableSearcherOrder;

    fn validate_order(
        &self,
        _origin: OrderOrigin,
        _transaction: Self::LimitOrder
    ) -> ValidationFuture<Self::LimitOrder> {
        todo!()
    }

    fn validate_composable_order(
        &self,
        _origin: OrderOrigin,
        _transaction: Self::ComposableLimitOrder
    ) -> ValidationFuture<Self::ComposableLimitOrder> {
        todo!()
    }

    fn validate_searcher_order(
        &self,
        _origin: OrderOrigin,
        _transaction: Self::SearcherOrder
    ) -> ValidationFuture<Self::SearcherOrder> {
        todo!()
    }

    fn validate_composable_searcher_order(
        &self,
        _origin: OrderOrigin,
        _transaction: Self::ComposableSearcherOrder
    ) -> ValidationFuture<Self::ComposableSearcherOrder> {
        todo!()
    }
}

/// Bundle Impl
impl BundleValidator for RevmClient {
    //TODO: Fix this, to whitebox simulate the swap directly, because it isn't a
    // full transaction and should not be validated as such
    async fn validate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError> {
        let (sender, rx) = channel();
        self.transaction_tx
            .send(BundleSimRequest::UniswapV4(tx, sender))?;

        Ok(rx.await.unwrap())
    }

    //TODO:
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
            .send(BundleSimRequest::Hook(hook, caller_info, tx))?;

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
            .send(BundleSimRequest::Bundle(bundle, caller_info, tx))?;

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
            .send(BundleSimRequest::MevBundle(bundle, caller_info, tx))?;

        Ok(rx.await.unwrap())
    }
}
