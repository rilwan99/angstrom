use std::{future::Future, pin::Pin};

use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{
    testnet_controllers::{AngstromTestnet, StateMachineTestnet},
    types::{StateMachineActionHookFn, StateMachineHook}
};

pub trait WithAction<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    type FunctionOutput = StateMachineActionHookFn<'a, C>;

    fn add_action<F>(&mut self, action_name: &'static str, action: F)
    where
        F: FnOnce(
                &'a mut AngstromTestnet<C>
            )
                -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + Sync + 'a>>
            + 'static;
}

impl<'a, C> WithAction<'a, C> for StateMachineTestnet<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    fn add_action<F>(&mut self, action_name: &'static str, action: F)
    where
        F: FnOnce(
                &'a mut AngstromTestnet<C>
            )
                -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + Sync + 'a>>
            + 'static
    {
        self.hooks
            .push((action_name, StateMachineHook::Action(Box::new(action))))
    }
}
