use std::{future::Future, pin::Pin};

use reth_chainspec::Hardforks;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider};

use super::AngstromTestnet;
use crate::types::{HookResult, StateMachineHook};

pub struct StateMachineTestnet<'a, C> {
    pub(crate) testnet: AngstromTestnet<C>,
    pub(crate) hooks:   Vec<(&'static str, StateMachineHook<'a, C>)>
}

impl<'a, C> StateMachineTestnet<'a, C>
where
    C: BlockReader
        + HeaderProvider
        + ChainSpecProvider
        + Unpin
        + Clone
        + ChainSpecProvider<ChainSpec: Hardforks>
        + 'static
{
    pub(crate) fn new(testnet: AngstromTestnet<C>) -> Self {
        Self { testnet, hooks: Vec::new() }
    }

    pub async fn run(&'a mut self) {
        let hooks = std::mem::take(&mut self.hooks);
        for (i, (name, hook)) in hooks.into_iter().enumerate() {
            self.run_hook(i, name, hook).await;
        }
    }

    async fn run_hook(&mut self, i: usize, name: &'static str, hook: StateMachineHook<'_, C>) {
        match hook {
            StateMachineHook::Action(action) => action(&mut self.testnet).await.fmt_result(i, name),
            StateMachineHook::Check(check) => check(&mut self.testnet).fmt_result(i, name),
            StateMachineHook::CheckedAction(checked_action) => {
                checked_action(&mut self.testnet).await.fmt_result(i, name)
            }
        }
    }
}
