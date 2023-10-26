use alloy_rlp::{RlpDecodableWrapper, RlpEncodableWrapper};
use guard_types::rpc::{SignedLimitOrder, SignedSearcherOrder};
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
pub struct GetUsersOrders(pub Vec<B256>);

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
pub struct UserOrders(pub Vec<SignedSearcherOrder>);

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
