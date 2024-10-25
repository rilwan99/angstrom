use std::{future::Future, pin::Pin};

use angstrom_network::StromMessage;
use angstrom_types::sol_bindings::grouped_orders::AllOrders;
use futures::FutureExt;
use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{
    testnet_controllers::{AngstromTestnet, StateMachineTestnet},
    types::{StateMachineCheckedActionHookFn, StateMachineHook}
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

    fn add_checked_action<F>(&mut self, checked_action_name: &'static str, checked_action: F)
    where
        F: FnOnce(
                &'a mut AngstromTestnet<C>
            )
                -> Pin<Box<dyn Future<Output = eyre::Result<bool>> + Send + Sync + 'a>>
            + 'static;

    // fn add_checked_action2<F, R>(&mut self, checked_action_name: &'static str,
    // checked_action: F) where
    //     F: Fn(&'a mut AngstromTestnet<C>) -> R + 'static,
    //     R: Future<Output = eyre::Result<bool>> + Send + Sync + 'b;

    fn send_bundles(&mut self, orders: Vec<AllOrders>) {
        let f = |testnet: &'a mut AngstromTestnet<C>| {
            Box::pin(testnet.broadcast_orders_message(
                None,
                StromMessage::PropagatePooledOrders(orders.clone()),
                orders
            )) as Pin<Box<dyn Future<Output = eyre::Result<bool>> + Send + Sync + 'a>>
        };
        self.add_checked_action("send bundles", f);
    }
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
    fn add_checked_action<F>(&mut self, checked_action_name: &'static str, checked_action: F)
    where
        F: FnOnce(
                &'a mut AngstromTestnet<C>
            )
                -> Pin<Box<dyn Future<Output = eyre::Result<bool>> + Send + Sync + 'a>>
            + 'static
    {
        self.hooks
            .push((checked_action_name, StateMachineHook::CheckedAction(Box::new(checked_action))))
    }

    // fn add_checked_action2<F, R>(&mut self, checked_action_name: &'static str,
    // checked_action: F) where
    //     F: Fn(&'a mut AngstromTestnet<C>) -> R + 'static,
    //     R: Future<Output = eyre::Result<bool>> + Send + Sync + 'b
    // {
    //     let f = Box::new(move |testnet: &'a mut AngstromTestnet<C>| {
    //         Box::pin(checked_action(testnet))
    //             as Pin<Box<dyn Future<Output = eyre::Result<bool>> + Send + Sync
    // + 'b>>     });

    //     self.add_checked_action(checked_action_name, f);
    // }
}

// fn map_bool<F>(f: F) -> Pin<Box<dyn Future<Output = eyre::Result<bool>> +
// Send + Sync>> where
//     F: Future<Output = bool> + Send + Sync
// {
//     Box::pin(async move { Ok::<_, eyre::ErrReport>(f.await) })
// }
