use alloy::primitives::Address;
use angstrom_types::contract_bindings::{angstrom::Angstrom, poolgate::PoolGate::PoolGateInstance};
use tracing::debug;

use super::{uniswap::TestUniswapEnv, TestAnvilEnvironment};
use crate::contracts::DebugTransaction;

pub trait TestAngstromEnv: TestAnvilEnvironment {
    fn angstrom(&self) -> Address;
}

pub struct AngstromEnv<E: TestUniswapEnv> {
    inner:    E,
    angstrom: Address
}

impl<E> AngstromEnv<E>
where
    E: TestUniswapEnv
{
    pub async fn new(inner: E) -> eyre::Result<Self> {
        let provider = inner.provider();
        debug!("Deploying mock rewards manager...");
        let angstrom = *Angstrom::deploy(
            &provider,
            inner.pool_manager(),
            inner.controller(),
            inner.controller()
        )
        .await?
        .address();
        debug!("Angstrom deployed at: {}", angstrom);
        // Set the PoolGate's hook to be our Mock
        debug!("Setting PoolGate hook...");
        let pool_gate_instance = PoolGateInstance::new(inner.pool_gate(), &provider);
        pool_gate_instance
            .setHook(angstrom)
            .from(inner.controller())
            .run_safe()
            .await?;
        debug!("Environment deploy complete!");
        Ok(Self { inner, angstrom })
    }
}
