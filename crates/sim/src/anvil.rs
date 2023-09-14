use std::{fmt::Debug, sync::Arc};

use anvil::{eth::EthApi, server::error::NodeResult, spawn, NodeConfig};
use anvil_core::eth::{state::StateOverride, transaction::EthTransactionRequest};
use ethers_core::{
    abi::AbiEncode,
    types::{
        transaction::eip2718::TypedTransaction, Address, GethDebugTracingOptions, H256, I256, U256,
        U64
    }
};
use ethers_providers::{Ipc, Provider};
use guard_types::{contract_bindings::ERC20, on_chain::*};
use hex_literal::hex;
use tokio::task::{JoinError, JoinHandle};

use crate::{BundleOrTransactionResult, SimError, SimResult, Simulator};

/// What we do here is we will deploy and run all transactions
/// to setup initial state when it comes to the test environment.
/// we then will take a snapshot. for commands, we then will
/// have a regular simulations on this overrided state
///
/// NOTE: This is only for testing. this is not optimzed at all
/// pls don't use this in prod
#[derive(Clone)]
pub struct AnvilSimulator {
    eth_api:                  EthApi,
    anvil_handle:             Arc<Provider<Ipc>>,
    _node_handle:             Arc<JoinHandle<Result<NodeResult<()>, JoinError>>>,
    /// this is an option because the bytecode of our spoof contract
    /// relys on our angstrom deployed code
    spoof_contract_overrides: Option<StateOverride>,
    angstrom_address:         Option<Address>,
    pool_manager_addr:        Option<Address>,
    default_state:            U256
}

impl AnvilSimulator {
    pub async fn new(config: NodeConfig) -> anyhow::Result<Self> {
        let (eth_api, node_handle) = spawn(config).await;
        let ipc = node_handle.ipc_provider().await.unwrap();
        let node_handle = tokio::spawn(node_handle);

        eth_api.anvil_auto_impersonate_account(true).await?;
        let default_state = eth_api.evm_snapshot().await?;

        Ok(Self {
            anvil_handle: ipc.into(),
            eth_api,
            _node_handle: Arc::new(node_handle),
            spoof_contract_overrides: None,
            angstrom_address: None,
            pool_manager_addr: None,
            default_state
        })
    }

    pub async fn update_default_snapshot(&mut self) -> anyhow::Result<()> {
        let default_state = self.eth_api.evm_snapshot().await?;
        self.default_state = default_state;

        Ok(())
    }

    /// deploys a contract and then returns the deploy address
    pub async fn deploy_contract(&self, tx: EthTransactionRequest) -> anyhow::Result<Address> {
        let tx_hash = self.eth_api.send_transaction(tx).await?;
        self.eth_api.mine_one().await;
        let tx = self.eth_api.transaction_receipt(tx_hash).await?.unwrap();

        Ok(tx.logs.first().unwrap().address.into())
    }

    /// used to move liquidity into pools
    pub async fn add_contract(&self, tx: EthTransactionRequest) -> anyhow::Result<H256> {
        Ok(self.eth_api.send_transaction(tx).await?)
    }

    pub async fn mine_one(&self) {
        self.eth_api.mine_one().await;
    }

    pub fn set_angstrom_contract(&mut self, addr: Address) {
        self.angstrom_address = Some(addr);
    }

    pub fn set_pool_manager_addr(&mut self, addr: Address) {
        self.pool_manager_addr = Some(addr);
    }
}

#[async_trait::async_trait]
impl Simulator for AnvilSimulator {
    async fn simulate_v4_tx(&self, tx: TypedTransaction) -> Result<SimResult, SimError> {
        let mut eth_tx = EthTransactionRequest::default();
        eth_tx.data = tx.data().cloned();
        eth_tx.from = tx.from().cloned();
        eth_tx.to = tx.to_addr().cloned();

        if let Some(spoof) = self.spoof_contract_overrides.as_ref() {
            for (addr, overrides) in spoof {
                if let Some(code) = overrides.code.as_ref() {
                    let _ = self
                        .eth_api
                        .anvil_set_code(*addr, code.clone())
                        .await
                        .unwrap();
                }
                if let Some(balance) = overrides.balance.as_ref() {
                    let _ = self
                        .eth_api
                        .anvil_set_balance(*addr, balance.clone())
                        .await
                        .unwrap();
                }
                if let Some(state) = overrides.state.as_ref() {
                    for (slot, val) in state {
                        let _ = self
                            .eth_api
                            .anvil_set_storage_at(
                                *addr,
                                U256::from_big_endian(&slot.encode()),
                                *val
                            )
                            .await
                            .unwrap();
                    }
                }
            }
        }

        let frame = self
            .eth_api
            .debug_trace_call(eth_tx, None, GethDebugTracingOptions::default())
            .await
            .unwrap();

        if frame.failed {
            return Err(SimError::V4Failed)
        }

        self.eth_api.evm_revert(self.default_state).await.unwrap();
        let delta = I256::from_raw(U256::from_big_endian(&frame.return_value));

        Ok(SimResult::ExecutionResult(BundleOrTransactionResult::UniswapV4Results {
            delta,
            gas: frame.gas
        }))
    }

    async fn simulate_hooks<T>(
        &self,
        hook_data: T,
        _caller_info: CallerInfo
    ) -> Result<SimResult, SimError>
    where
        T: TryInto<HookSim> + Send,
        <T as TryInto<HookSim>>::Error: Debug
    {
        let hook: HookSim = hook_data.try_into().unwrap();
        let (call_addr, data) = hook.pre_hook();

        // send prehook and then check if we have enough now to swap on our pool
        let mut pre_hook_tx = EthTransactionRequest::default();
        pre_hook_tx.data = Some(data.into());
        pre_hook_tx.from = Some(hook.addr);
        pre_hook_tx.to = Some(call_addr);
        let pre_tx_hash = self.eth_api.send_transaction(pre_hook_tx).await.unwrap();

        let slot =
            find_slot(hook.amount_out_token, hook.addr, &self.eth_api, self.anvil_handle.clone())
                .await;

        let value = self
            .eth_api
            .storage_at(hook.amount_out_token, slot, None)
            .await
            .unwrap();

        let mut decoded = U256::from_big_endian(&value.as_bytes());
        decoded += U256::from(hook.amount_out_lim);

        self.eth_api
            .anvil_set_storage_at(hook.amount_out_token, slot, H256::from_slice(&decoded.encode()))
            .await
            .unwrap();

        let (call_addr, data) = hook.post_hook();
        let mut post_hook_tx = EthTransactionRequest::default();

        post_hook_tx.data = Some(data.into());
        post_hook_tx.from = Some(hook.addr);
        post_hook_tx.to = Some(call_addr);
        let post_tx_hash = self.eth_api.send_transaction(post_hook_tx).await.unwrap();
        self.eth_api.mine_one().await;

        let post_tx = self
            .eth_api
            .transaction_receipt(post_tx_hash)
            .await
            .unwrap()
            .unwrap();
        if post_tx.status.unwrap() != U64::from(1) {
            return Err(SimError::HookFailed)
        }
        let post_hook_gas = post_tx.cumulative_gas_used;

        let erc = ERC20::new(hook.amount_in_token, self.anvil_handle.clone());
        let amount_in = erc.balance_of(hook.addr).call().await.unwrap().as_u128();
        if amount_in < hook.amount_in_req {
            return Err(SimError::InvalidAmountIn(hook.amount_in_req, amount_in))
        }

        let pre_tx = self
            .eth_api
            .transaction_receipt(pre_tx_hash)
            .await
            .unwrap()
            .unwrap();
        if pre_tx.status.unwrap() != U64::from(1) {
            return Err(SimError::HookFailed)
        }
        let pre_hook_gas = pre_tx.cumulative_gas_used;
        self.eth_api.evm_revert(self.default_state).await.unwrap();

        Ok(SimResult::ExecutionResult(crate::BundleOrTransactionResult::HookSimResult {
            tx: hook.tx,
            pre_hook_gas,
            post_hook_gas
        }))
    }

    async fn simulate_bundle(
        &self,
        _caller_info: CallerInfo,
        bundle: RawBundle
    ) -> Result<SimResult, SimError> {
        let mut data = Vec::new();
        data.extend(hex!("ac8a9f85"));
        data.extend(bundle.clone().encode());

        let tx = EthTransactionRequest { data: Some(data.into()), ..Default::default() };
        let frame = self
            .eth_api
            .debug_trace_call(tx, None, GethDebugTracingOptions::default())
            .await
            .unwrap();
        if frame.failed {
            return Err(SimError::V4Failed)
        }

        let bundle = SimmedBundle { raw: bundle, gas_used: frame.gas };
        Ok(SimResult::ExecutionResult(BundleOrTransactionResult::Bundle(bundle)))
    }
}

async fn find_slot(
    token_addr: Address,
    user_addr: Address,
    eth_api: &EthApi,
    provider: Arc<Provider<Ipc>>
) -> U256 {
    let prob_value = U256::from(3566969).encode();
    let token_caller = ERC20::new(token_addr, provider);

    for i in 0..100 {
        let mut user_addr_encoded = user_addr.encode();
        user_addr_encoded.extend(U256::from(i).encode());

        let user_balance_slot = U256::from_big_endian(&ethers::utils::keccak256(user_addr_encoded));
        eth_api
            .anvil_set_storage_at(token_addr, user_balance_slot, H256::from_slice(&prob_value))
            .await
            .unwrap();

        let bal = token_caller.balance_of(user_addr).call().await.unwrap();
        if U256::from(3566969) == bal {
            return U256::from(i)
        }
    }
    unreachable!()
}
