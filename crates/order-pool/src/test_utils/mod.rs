//! Internal helpers for testing.
#![allow(missing_docs, unused, missing_debug_implementations, unreachable_pub)]

mod gen;
mod mock;
mod pool;

use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
pub use gen::*;
pub use mock::*;

use crate::{
    blobstore::InMemoryBlobStore, noop::MockOrderValidator, OrderOrigin, OrderValidator, Pool,
    PoolOrder, TransactionValidationOutcome
};

/// A [Pool] used for testing
pub type TestPool = Pool<MockOrderValidator<MockTransaction>, MockOrdering, InMemoryBlobStore>;

/// Returns a new [Pool] used for testing purposes
pub fn testing_pool() -> TestPool {
    testing_pool_with_validator(MockOrderValidator::default())
}
/// Returns a new [Pool] used for testing purposes
pub fn testing_pool_with_validator(validator: MockOrderValidator<MockTransaction>) -> TestPool {
    Pool::new(validator, MockOrdering::default(), InMemoryBlobStore::default(), Default::default())
}
