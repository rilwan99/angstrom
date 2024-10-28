use std::{future::Future, pin::Pin};

use angstrom_network::StromMessage;
use angstrom_types::sol_bindings::grouped_orders::AllOrders;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{
    testnet_controllers::{AngstromTestnet, StateMachineTestnet},
    types::StateMachineCheckedActionHookFn
};

pub trait WithCheckedAction<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    type FunctionOutput = StateMachineCheckedActionHookFn<'a, C>;

    fn send_bundles(&mut self, orders: Vec<AllOrders>);
}

impl<'a, C> WithCheckedAction<'a, C> for StateMachineTestnet<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    fn send_bundles(&mut self, orders: Vec<AllOrders>) {
        let f = |testnet: &'a mut AngstromTestnet<C>| {
            pin_action(testnet.broadcast_orders_message(
                None,
                StromMessage::PropagatePooledOrders(orders.clone()),
                orders
            ))
        };
        self.add_checked_action("send bundles", f);
    }
}

fn pin_action<'a, F>(fut: F) -> Pin<Box<dyn Future<Output = eyre::Result<bool>> + Send + Sync + 'a>>
where
    F: Future<Output = eyre::Result<bool>> + Send + Sync + 'a
{
    Box::pin(fut)
}
