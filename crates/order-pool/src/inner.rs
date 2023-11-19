use std::marker::PhantomData;

use guard_types::orders::{PooledComposableOrder, PooledLimitOrder};
use tokio::sync::mpsc::Sender;

use crate::limit::LimitOrderPool;

pub struct OrderPoolInner<L, LC, S, SC, V>
where
    L: PooledLimitOrder,
    LC: PooledComposableOrder + PooledLimitOrder
{
    /// limit pool
    limit_pool:      LimitOrderPool<L, LC>,
    /// event listeners
    event_listeners: Vec<Sender<()>>,

    validator: V,

    _p: PhantomData<(S, SC, V)>
}
