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
}

impl<'a, C> WithAction<'a, C> for StateMachineTestnet<'a, C> where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
}

fn pin_action<'a, F>(fut: F) -> Pin<Box<dyn Future<Output = eyre::Result<()>> + Send + Sync + 'a>>
where
    F: Future<Output = eyre::Result<()>> + Send + Sync + 'a
{
    Box::pin(fut)
}
