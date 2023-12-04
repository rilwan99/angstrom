use futures_util::Future;
use guard_eth::EthEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    bundle::{bundle_validator::BundleValidator, BundleSimRequest},
    order::{order_validator::OrderValidator, OrderValidationRequest}
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

    order_validator:  OrderValidator<DB>,
    bundle_validator: BundleValidator
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
