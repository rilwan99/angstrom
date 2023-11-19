use std::marker::PhantomData;

use tokio::sync::mpsc::Sender;

use crate::{limit::LimitOrderPool, PooledComposableOrder, PooledLimitOrder};

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
