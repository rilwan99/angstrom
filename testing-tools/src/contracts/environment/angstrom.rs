use std::future::Future;

use alloy::primitives::Address;
use angstrom_types::contract_bindings::pool_gate::PoolGate::PoolGateInstance;
use tracing::debug;

use super::{uniswap::TestUniswapEnv, TestAnvilEnvironment};
use crate::contracts::{deploy::angstrom::deploy_angstrom, DebugTransaction};

pub trait TestAngstromEnv: TestAnvilEnvironment + TestUniswapEnv {
    fn angstrom(&self) -> Address;
}

pub struct AngstromEnv<E: TestUniswapEnv> {
    #[allow(dead_code)]
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
        let angstrom = deploy_angstrom(
            &provider,
            inner.pool_manager(),
            inner.controller(),
            Address::default()
        )
        .await;
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

    pub fn angstrom(&self) -> Address {
        self.angstrom
    }
}

impl<E> TestUniswapEnv for AngstromEnv<E>
where
    E: TestUniswapEnv
{
    fn pool_manager(&self) -> Address {
        self.inner.pool_manager()
    }

    fn pool_gate(&self) -> Address {
        self.inner.pool_gate()
    }

    async fn add_liquidity_position(
        &self,
        asset0: Address,
        asset1: Address,
        lower_tick: alloy_primitives::aliases::I24,
        upper_tick: alloy_primitives::aliases::I24,
        liquidity: alloy_primitives::U256
    ) -> eyre::Result<()> {
        self.inner
            .add_liquidity_position(asset0, asset1, lower_tick, upper_tick, liquidity)
            .await
    }
}

impl<E> TestAnvilEnvironment for AngstromEnv<E>
where
    E: TestUniswapEnv
{
    type P = E::P;
    type T = E::T;

    fn provider(&self) -> &Self::P {
        self.inner.provider()
    }

    fn controller(&self) -> Address {
        self.inner.controller()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        time::{Duration, SystemTime, UNIX_EPOCH}
    };

    use alloy::{
        primitives::{Address, Bytes},
        signers::{local::LocalSigner, SignerSync}
    };
    use alloy_primitives::{Uint, U256};
    use angstrom_types::{
        contract_bindings::angstrom::Angstrom::AngstromInstance,
        contract_payloads::angstrom::{AngstromBundle, UserOrder},
        orders::{OrderFillState, OrderOutcome},
        primitive::ANGSTROM_DOMAIN,
        sol_bindings::{
            grouped_orders::{GroupedVanillaOrder, OrderWithStorageData, StandingVariants},
            rpc_orders::OmitOrderMeta
        }
    };
    use pade::PadeEncode;

    use super::{AngstromEnv, DebugTransaction};
    use crate::{
        contracts::environment::{uniswap::UniswapEnv, SpawnedAnvil, TestAnvilEnvironment},
        type_generator::consensus::proposal::ProposalBuilder
    };

    #[tokio::test]
    async fn can_be_constructed() {
        let anvil = SpawnedAnvil::new().await.unwrap();
        let uniswap = UniswapEnv::new(anvil).await.unwrap();
        AngstromEnv::new(uniswap).await.unwrap();
    }

    #[test]
    fn do_a_thing() {
        let user = LocalSigner::random();
        let address = user.address();
        let mut default = angstrom_types::sol_bindings::rpc_orders::ExactStandingOrder {
            exactIn:     true,
            amount:      10,
            minPrice:    U256::from(1u128),
            useInternal: false,
            assetIn:     Address::random(),
            assetOut:    Address::random(),
            recipient:   Address::random(),
            hook:        Address::ZERO,
            hookPayload: alloy::primitives::Bytes::new(),
            nonce:       0,
            deadline:    Uint::<40, 1>::from_be_slice(
                &(SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
                    + Duration::from_secs(1000))
                .as_secs()
                .to_be_bytes()[3..]
            ),
            meta:        Default::default()
        };
        let hash = default.no_meta_eip712_signing_hash(&ANGSTROM_DOMAIN);
        let sig = user.sign_hash_sync(&hash).unwrap();
        default.meta.isEcdsa = true;
        default.meta.from = address;
        default.meta.signature = sig.pade_encode().into();
        // (address, default)

        let mut user_order = OrderWithStorageData {
            order: GroupedVanillaOrder::Standing(StandingVariants::Exact(default)),
            is_currently_valid: true,
            is_bid: true,
            ..Default::default()
        };
        let outcome =
            OrderOutcome { id: user_order.order_id, outcome: OrderFillState::CompleteFill };
        let encode = UserOrder::from_internal_order(&user_order, &outcome, 0).pade_encode();
    }

    #[tokio::test]
    async fn accepts_payload() {
        // Create our anvil environment and grab the nodes and provider
        let anvil = SpawnedAnvil::new().await.unwrap();
        let nodes: Vec<Address> = anvil.anvil.addresses().iter().cloned().collect();
        let controller = nodes[7];
        let provider = anvil.provider().clone();

        let uniswap = UniswapEnv::new(anvil).await.unwrap();
        let env = AngstromEnv::new(uniswap).await.unwrap();
        let angstrom = AngstromInstance::new(env.angstrom(), provider);
        let proposal = ProposalBuilder::new()
            .for_random_pools(1)
            .order_count(1)
            .build();
        println!("Proposal:\n{:?}", proposal);
        let pools = HashMap::new();
        let bundle = AngstromBundle::from_proposal(&proposal, &pools).unwrap();
        let encoded = bundle.pade_encode();

        angstrom.toggleNodes(nodes).run_safe().await.unwrap();
        angstrom
            .execute(Bytes::from(encoded))
            .from(controller)
            .run_safe()
            .await
            .unwrap();
        // angstrom.execute(encoded)
    }
}
