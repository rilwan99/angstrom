use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{
    testnet_controllers::{AngstromTestnet, StateMachineTestnet},
    types::{StateMachineCheckHookFn, StateMachineHook}
};

pub trait WithCheck<C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    type FunctionOutput = StateMachineCheckHookFn<C>;

    fn add_check<F>(&mut self, checked_action_name: &'static str, checked_action: F)
    where
        F: Fn(&mut AngstromTestnet<C>) -> eyre::Result<bool> + 'static;
}

impl<'a, C> WithCheck<C> for StateMachineTestnet<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    fn add_check<F>(&mut self, check_name: &'static str, check: F)
    where
        F: Fn(&mut AngstromTestnet<C>) -> eyre::Result<bool> + 'static
    {
        self.hooks
            .push((check_name, StateMachineHook::Check(Box::new(check))))
    }
}
