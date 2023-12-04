use std::pin::Pin;

use futures::{Stream, StreamExt};
use futures_util::Future;
use guard_eth::manager::EthEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    bundle::{bundle_validator::BundleValidator, BundleSimRequest, BundleValidator},
    common::lru_db::RevmLRU,
    order::{order_validator::OrderValidator, OrderValidationRequest, OrderValidator}
};

pub enum ValidationRequest {
    Bundle(BundleSimRequest),
    Order(OrderValidationRequest)
}

#[derive(Debug, Clone)]
pub struct ValidationClient(UnboundedSender<ValidationRequest>);

/// HeadModule that deals with all validation
#[allow(dead_code)]
pub struct Validator<DB> {
    rx:               UnboundedReceiver<ValidationRequest>,
    /// used to update state
    new_block_stream: Pin<Box<dyn Stream<Item = EthEvent>>>,
    db:               Arc<RevmLRU<DB>>,

    order_validator:  OrderValidator<DB>,
    bundle_validator: BundleValidator
}

impl<DB> Validator<DB> {
    pub fn block_state_changes(&mut self, state: BundleState) {
        // TODO: update the db
    }
}

impl<DB> Future for Validator<DB> {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        Poll::Pending
    }
}
