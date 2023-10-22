use std::{collections::HashMap, sync::Arc};

use alloy_primitives::Address;
use parking_lot::RwLock;
use reth_db::mdbx::{tx::Tx, WriteMap, RO};
use reth_provider::LatestStateProvider;
use reth_revm::{database::StateProviderDatabase, State};
use revm::{
    db::{AccountState, DbAccount},
    Database
};
use revm_primitives::{db::DatabaseRef, AccountInfo, Bytecode, B256, U256};
use schnellru::{ByMemoryUsage, LruMap};

use crate::{errors::SimError, state::RevmBackend};

pub struct RevmLRU {
    state_overrides:    HashMap<Address, HashMap<U256, U256>>,
    bytecode_overrides: HashMap<Address, Bytecode>,
    accounts:           Arc<RwLock<LruMap<Address, DbAccount, ByMemoryUsage>>>,
    contracts:          Arc<RwLock<LruMap<B256, Bytecode, ByMemoryUsage>>>,
    db:                 Arc<reth_db::mdbx::Env<WriteMap>>
}

impl Clone for RevmLRU {
    fn clone(&self) -> Self {
        Self {
            state_overrides:    HashMap::default(),
            bytecode_overrides: HashMap::default(),
            accounts:           self.accounts.clone(),
            contracts:          self.contracts.clone(),
            db:                 self.db.clone()
        }
    }
}

impl RevmBackend for RevmLRU {
    fn update_evm_state(
        &self,
        slot_changes: &crate::state::AddressSlots
    ) -> eyre::Result<(), SimError> {
        let db = self.db.clone();
        let state_provider = RevmLRU::get_lastest_state_provider(Tx::new(db.begin_ro_txn()?));

        let mut accounts = self.accounts.write();

        for (addr, storage) in slot_changes.iter() {
            let acct_storage = accounts
                .get_or_insert(*addr, || DbAccount {
                    info: state_provider.basic(*addr).unwrap().unwrap(),
                    ..Default::default()
                })
                .unwrap();
            for (idx, val) in storage {
                let new_state = state_provider.storage(*addr, *idx)?;
                if new_state != *val {
                    acct_storage.storage.insert(*idx, new_state);
                }
            }
        }
        Ok(())
    }
}

impl RevmLRU {
    pub fn new(max_bytes: usize, db: Arc<reth_db::mdbx::Env<WriteMap>>) -> Self {
        let accounts = Arc::new(RwLock::new(LruMap::new(ByMemoryUsage::new(max_bytes))));
        let contracts = Arc::new(RwLock::new(LruMap::new(ByMemoryUsage::new(max_bytes))));
        Self {
            accounts,
            contracts,
            db,
            state_overrides: HashMap::default(),
            bytecode_overrides: HashMap::default()
        }
    }

    pub fn set_state_overrides(&mut self, overrides: HashMap<Address, HashMap<U256, U256>>) {
        self.state_overrides = overrides
    }

    pub fn set_bytecode_overrides(&mut self, overrides: HashMap<Address, Bytecode>) {
        self.bytecode_overrides = overrides
    }

    pub fn get_lastest_state_provider(
        tx: Tx<'_, RO, WriteMap>
    ) -> StateProviderDatabase<LatestStateProvider<Tx<'_, RO, WriteMap>>> {
        let db_provider = LatestStateProvider::new(tx);

        StateProviderDatabase(db_provider)
    }
}

impl DatabaseRef for RevmLRU {
    type Error = SimError;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let accounts = self.accounts.read();
        match accounts.peek(&address) {
            Some(acc) => Ok(acc.info()),
            None => {
                let db = Self::get_lastest_state_provider(Tx::new(self.db.begin_ro_txn()?));
                Ok(db.basic(address)?)
            }
        }
    }

    fn code_by_hash_ref(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        // check overrides
        if let Some(storage) = self.state_overrides.get(&address) {
            if let Some(value) = storage.get(&index) {
                return Ok(*value)
            }
        }

        // check default
        let db = Self::get_lastest_state_provider(Tx::new(self.db.begin_ro_txn()?));
        let accounts = self.accounts.read();
        if let Some(acc_entry) = accounts.peek(&address) {
            if let Some(entry) = acc_entry.storage.get(&index) {
                return Ok(*entry)
            }
            if !matches!(
                acc_entry.account_state,
                AccountState::StorageCleared | AccountState::NotExisting
            ) {
                return Ok(db.storage(address, index)?)
            }
        } else {
            return Ok(db.storage(address, index)?)
        }

        Ok(U256::default())
    }

    fn block_hash_ref(&self, _number: U256) -> Result<B256, Self::Error> {
        unreachable!() // this should never be reached since we will never sim
                       // blocks
    }
}
