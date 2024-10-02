use std::future::IntoFuture;

use alloy::{primitives::keccak256, providers::Provider, transports::TransportResult};
use reth_primitives::{Account, Address, StorageKey, StorageValue};
use reth_provider::{ProviderError, ProviderResult};
use validation::common::lru_db::BlockStateProvider;

use crate::{async_to_sync, AnvilWalletRpc};

#[derive(Clone, Debug)]
pub struct RpcStateProvider {
    block:    u64,
    provider: AnvilWalletRpc
}

impl RpcStateProvider {
    pub fn new(block: u64, provider: AnvilWalletRpc) -> Self {
        Self { block, provider }
    }

    async fn get_account(&self, address: Address) -> TransportResult<Account> {
        let (nonce, balance, bytecode) = futures::try_join!(
            self.provider.get_transaction_count(address).into_future(),
            self.provider.get_balance(address).into_future(),
            // TODO: Ensure correct handling of EOA empty accounts.
            self.provider.get_code_at(address).into_future()
        )?;

        let hash = keccak256(bytecode);

        Ok(Account { nonce, balance, bytecode_hash: Some(hash) })
    }
}

impl BlockStateProvider for RpcStateProvider {
    fn get_storage(
        &self,
        address: Address,
        key: StorageKey
    ) -> ProviderResult<Option<StorageValue>> {
        async_to_sync(
            self.provider
                .get_storage_at(address, key.into())
                .into_future()
        )
        .map(Some)
        // TODO: Better error.
        .map_err(|_| ProviderError::StorageChangesetNotFound {
            block_number: self.block,
            address,
            storage_key: Box::new(key)
        })
    }

    fn get_basic_account(&self, address: Address) -> ProviderResult<Option<Account>> {
        async_to_sync(self.get_account(address))
            .map(Some)
            .map_err(|_| ProviderError::AccountChangesetNotFound {
                block_number: self.block,
                address
            })
    }
}
