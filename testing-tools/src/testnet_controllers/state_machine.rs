use std::{future::Future, pin::Pin};

use super::AngstromTestnet;
use crate::types::{StateMachineActionHookFn, StateMachineCheckHookFn, StateMachineHook};

pub struct StateMachineTestnet<C> {
    testnet: AngstromTestnet<C>,
    hooks:   Vec<StateMachineHook<C>>
}

impl<C> StateMachineTestnet<C> {
    pub async fn run(mut self) -> eyre::Result<()> {
        let hooks = std::mem::take(&mut self.hooks);
        for hook in hooks {
            match hook {
                StateMachineHook::Action(action) => self.run_action(action).await?,
                StateMachineHook::Check(check) => assert!(self.run_check(check).await?)
            }
        }

        Ok(())
    }

    pub fn add_action<F>(&mut self, action: F)
    where
        F: Fn(&mut AngstromTestnet<C>) -> Pin<Box<dyn Future<Output = eyre::Result<()>>>> + 'static
    {
        self.hooks.push(StateMachineHook::Action(Box::new(action)))
    }

    pub fn add_check<F>(&mut self, check: F)
    where
        F: Fn(&AngstromTestnet<C>) -> bool + 'static
    {
        self.hooks.push(StateMachineHook::Check(Box::new(check)))
    }

    async fn run_action(&mut self, action: StateMachineActionHookFn<C>) -> eyre::Result<()> {
        Ok(())
    }

    async fn run_check(&mut self, check: StateMachineCheckHookFn<C>) -> eyre::Result<bool> {
        Ok(false)
    }
}
