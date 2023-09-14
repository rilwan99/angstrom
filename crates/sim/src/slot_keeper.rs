use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll}
};

use ethers_core::{
    abi::AbiEncode,
    types::{Address, U256 as EU256}
};
use ethers_providers::{JsonRpcClient, Middleware, ProviderError};
use futures::Future;
use futures_util::FutureExt;
use guard_types::contract_bindings::ERC20;
use revm::EVM;
use revm_primitives::{TransactTo, TxEnv, B160, U256};
use tokio::{runtime::Handle, task::JoinHandle};

use crate::lru_db::RevmLRU;

#[derive(Debug)]
/// we use this to be able to generate our fake transaction data
struct FakeClient;

#[async_trait::async_trait]
impl JsonRpcClient for FakeClient {
    type Error = ProviderError;

    async fn request<T, R>(&self, _method: &str, _params: T) -> Result<R, Self::Error>
    where
        T: std::fmt::Debug + serde::Serialize + Send + Sync,
        R: serde::de::DeserializeOwned + Send
    {
        unreachable!()
    }
}
impl Middleware for FakeClient {
    type Error = ProviderError;
    /// The next-lower middleware in the middleware stack
    type Inner = Self;
    /// The JSON-RPC client type at the bottom of the stack
    type Provider = Self;

    /// Get a reference to the next-lower middleware in the middleware stack
    fn inner(&self) -> &Self::Inner {
        self
    }
}

pub struct SlotKeeper {
    addresses: Vec<Address>,
    slots:     HashMap<B160, U256>,
    db:        RevmLRU,
    handle:    Handle,
    fut:       Option<JoinHandle<HashMap<B160, U256>>>
}

impl SlotKeeper {
    pub fn new(db: RevmLRU, addresses: Vec<Address>, handle: Handle) -> Self {
        let slots = SlotKeeper::get_slots(addresses.clone(), db.clone());

        Self { addresses, db, slots, handle, fut: None }
    }

    pub fn get_current_slots(&self) -> &HashMap<B160, U256> {
        &self.slots
    }

    pub fn new_addresses(&mut self, address: Vec<Address>) {
        let clone_addr = address.clone();
        let db = self.db.clone();

        self.fut = Some(
            self.handle
                .spawn(async move { Self::get_slots(clone_addr, db) })
        );

        self.addresses.extend(address);
    }

    pub fn get_slots(addresses: Vec<Address>, db: RevmLRU) -> HashMap<B160, U256> {
        // this is pretty big. would be nice to breakup
        addresses
            .iter()
            .map(|token_addr| {
                let erc = ERC20::new(*token_addr, Arc::new(FakeClient));
                let tx = erc.balance_of(*token_addr).tx;
                let mut tx_env = TxEnv::default();

                tx_env.data = tx.data().unwrap().clone().0;
                tx_env.transact_to = TransactTo::Call((*tx.to_addr().unwrap()).into());

                let prob_value = U256::from_limbs([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
                let search_addr = Address::zero();

                for i in 0..100 {
                    let mut user_addr_encoded = search_addr.clone().encode();
                    user_addr_encoded.extend(EU256::from(i).encode());

                    let user_balance_slot =
                        U256::from_be_bytes(ethers::utils::keccak256(user_addr_encoded));

                    let mut slot = HashMap::new();
                    slot.insert(user_balance_slot, prob_value);
                    let mut overrides = HashMap::new();
                    overrides.insert((*token_addr).into(), slot);

                    let mut db = db.clone();
                    db.set_state_overrides(overrides);
                    let mut evm = EVM::new();
                    evm.database(db);

                    // this is just a balance_of call. should never fail
                    let output = match evm.transact_ref().unwrap().result {
                        revm_primitives::ExecutionResult::Success { output, .. } => {
                            output.into_data()
                        }
                        _ => unreachable!()
                    };

                    // bad but needed to convert directly. ideally we can remove this
                    let result = U256::from_be_bytes(unsafe {
                        *(output.to_vec().as_slice() as *const _ as *mut [u8; 32])
                    });

                    if U256::MAX == result {
                        return ((*token_addr).into(), U256::from(i))
                    }
                }
                unreachable!()
            })
            .collect()
    }
}

impl Future for SlotKeeper {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(mut fut) = self.fut.take() {
            if let Poll::Ready(res) = fut.poll_unpin(cx) {
                let Ok(res) = res else { return Poll::Ready(()) };
                self.slots.extend(res);
            } else {
                self.fut = Some(fut)
            }
        }

        Poll::Pending
    }
}
