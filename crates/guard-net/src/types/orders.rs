use alloy_rlp::{RlpDecodableWrapper, RlpEncodableWrapper};
use guard_types::rpc::{
    SignedComposableLimitOrder, SignedComposableSearcherOrder, SignedLimitOrder,
    SignedSearcherOrder
};
use reth_codecs::derive_arbitrary;
use reth_primitives::B256;
use serde::{Deserialize, Serialize};

/// a list of hashs a peer wants orders for
#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct GetLimitOrders(pub Vec<B256>);

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct LimitOrders(pub Vec<SignedLimitOrder>);

/// a list of hashs a peer wants orders for
#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct GetComposableLimitOrders(pub Vec<B256>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct ComposableLimitOrders(pub Vec<SignedComposableLimitOrder>);

/// a list of hashs a peer wants orders for
#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct GetSearcherOrders(pub Vec<B256>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct SearcherOrders(pub Vec<SignedSearcherOrder>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct GetComposableSearcherOrders(pub Vec<B256>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct ComposableSearcherOrders(pub Vec<SignedComposableSearcherOrder>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    RlpEncodableWrapper,
    RlpDecodableWrapper,
    Default,
    Serialize,
    Deserialize,
)]
pub struct GetOrders(pub Vec<B256>);

#[derive_arbitrary(rlp)]
#[derive(
    Clone, Debug, PartialEq, Eq, RlpEncodableWrapper, RlpDecodableWrapper, Serialize, Deserialize,
)]
pub struct Orders(pub Vec<Orders>);

pub enum Order {
    Limit(SignedLimitOrder),
    ComposableLimit(SignedComposableLimitOrder),
    Searcher(SignedSearcherOrder),
    ComposableSearcher(SignedComposableSearcherOrder)
}
