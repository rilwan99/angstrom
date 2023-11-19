mod common;
mod inner;
mod limit;
mod searcher;
mod traits;

pub use common::*;
use guard_types::rpc::{
    EcRecoveredComposableLimitOrder, EcRecoveredComposableSearcherOrder, EcRecoveredLimitOrder,
    EcRecoveredSearcherOrder
};
use inner::OrderPoolInner;
use validation::RevmClient;

pub use crate::traits::*;

pub type DefaultOrderPool = OrderPoolInner<
    EcRecoveredLimitOrder,
    EcRecoveredComposableLimitOrder,
    EcRecoveredSearcherOrder,
    EcRecoveredComposableSearcherOrder,
    RevmClient
>;
