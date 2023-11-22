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
pub struct Validator<DB> {
    rx: UnboundedReceiver<ValidationRequest>,

    order_validator:  OrderValidator<DB>,
    bundle_validator: BundleValidator
}
