use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use crate::{testnet_controllers::StateMachineTestnet, types::StateMachineCheckHookFn};

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
}

impl<'a, C> WithCheck<C> for StateMachineTestnet<'a, C> where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
}
