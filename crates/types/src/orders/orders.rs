use alloy_rlp::{
    Buf, BufMut, Decodable, Encodable, Error, RlpDecodableWrapper, RlpEncodableWrapper
};
use reth_codecs::derive_arbitrary;
use reth_primitives::B256;
use serde::{Deserialize, Serialize};

use crate::rpc::{
    SignedComposableLimitOrder, SignedComposableSearcherOrder, SignedLimitOrder,
    SignedSearcherOrder
};

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
pub struct Orders(pub Vec<PooledOrder>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PooledOrder {
    Limit(SignedLimitOrder),
    ComposableLimit(SignedComposableLimitOrder),
    Searcher(SignedSearcherOrder),
    ComposableSearcher(SignedComposableSearcherOrder)
}

impl PooledOrder {
    pub fn order_type(&self) -> OrderType {
        match self {
            PooledOrder::Limit(_) => OrderType::Limit,
            PooledOrder::ComposableLimit(_) => OrderType::Composable,
            PooledOrder::Searcher(_) => OrderType::Searcher,
            PooledOrder::ComposableSearcher(_) => OrderType::ComposableSearcher
        }
    }
}

impl Encodable for PooledOrder {
    fn encode(&self, out: &mut dyn BufMut) {
        out.put_u8(self.order_type() as u8);
        match self {
            PooledOrder::Limit(order) => order.encode(out),
            PooledOrder::ComposableLimit(order) => order.encode(out),
            PooledOrder::Searcher(order) => order.encode(out),
            PooledOrder::ComposableSearcher(order) => order.encode(out)
        }
    }

    /// Returns the length of the encoding of this type in bytes.
    #[inline]
    fn length(&self) -> usize {
        let mut out = Vec::new();
        self.encode(&mut out);
        out.len()
    }
}

impl Decodable for PooledOrder {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        let order_type = *buf.first().ok_or(Error::InputTooShort)?;
        buf.advance(1);
        match order_type.into() {
            OrderType::Limit => Ok(PooledOrder::Limit(SignedLimitOrder::decode(buf)?)),
            OrderType::Composable => {
                Ok(PooledOrder::ComposableLimit(SignedComposableLimitOrder::decode(buf)?))
            }
            OrderType::Searcher => Ok(PooledOrder::Searcher(SignedSearcherOrder::decode(buf)?)),
            OrderType::ComposableSearcher => {
                Ok(PooledOrder::ComposableSearcher(SignedComposableSearcherOrder::decode(buf)?))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OrderType {
    /// Limit order
    Limit              = 0,
    /// Composable limit order
    Composable         = 1,
    /// Searcher Order
    Searcher           = 2,
    /// Composable searcher order
    ComposableSearcher = 3
}

impl From<u8> for OrderType {
    fn from(value: u8) -> Self {
        match value {
            0 => OrderType::Limit,
            1 => OrderType::Composable,
            2 => OrderType::Searcher,
            3 => OrderType::ComposableSearcher,
            _ => panic!("invalid order type")
        }
    }
}
