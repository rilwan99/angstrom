use std::{
    path::{Path, PathBuf},
    sync::Arc
};

use ethers_middleware::Middleware;
use reth_db::{
    mdbx::{tx::Tx, WriteMap, RO},
    transaction::DbTx
};
use reth_provider::{LatestStateProvider, LatestStateProviderRef, StateProvider, StateProviderBox};
use reth_revm::database::State;
use revm::{
    db::{AccountState, DbAccount},
    Database
};
use revm_primitives::{db::DatabaseRef, Bytecode, *};
use schnellru::{ByMemoryUsage, LruMap};
use tokio::runtime::Handle;

pub struct RevmLRU {
    pub accounts: LruMap<B160, DbAccount, ByMemoryUsage>,
    pub db:       Arc<reth_db::mdbx::Env<WriteMap>>
}

impl RevmLRU {
    pub fn new(max_bytes: usize, db: Arc<reth_db::mdbx::Env<WriteMap>>) -> Self {
        let accounts = LruMap::new(ByMemoryUsage::new(max_bytes));

        Self { accounts, db }
    }

    pub fn get_lastest_state_provider(
        tx: Tx<'_, RO, WriteMap>
    ) -> State<LatestStateProvider<'_, Tx<'_, RO, WriteMap>>> {
        //let tx = Tx::new(self.db.begin_ro_txn().unwrap());
        let db_provider = LatestStateProvider::new(tx);

        State::new(db_provider)
    }
}

impl Database for RevmLRU {
    type Error = ();

    fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        let db = Self::get_lastest_state_provider(Tx::new(self.db.begin_ro_txn().unwrap()));
        if let Some(a) = self.accounts.get(&address) {
            return Ok(a.info())
        } else {
            let basic = db
                .basic(address)
                .unwrap()
                .map(|info| DbAccount { info, ..Default::default() })
                .unwrap_or_else(DbAccount::new_not_existing);

            self.accounts.insert(address, basic.clone());
            return Ok(basic.info())
        }
    }

    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let db = Self::get_lastest_state_provider(Tx::new(self.db.begin_ro_txn().unwrap()));
        let account = self.accounts.get(&address);
        if let Some(acct_entry) = account {
            if let Some(idx_entry) = acct_entry.storage.get(&index) {
                return Ok(*idx_entry)
            } else {
                if matches!(
                    acct_entry.account_state,
                    AccountState::StorageCleared | AccountState::NotExisting
                ) {
                    return Ok(U256::ZERO)
                } else {
                    let slot_val = db.storage(address, index).unwrap();
                    acct_entry.storage.insert(index, slot_val);
                    return Ok(slot_val)
                }
            }
        } else {
            let info = db.basic(address).unwrap();
            let (account, value) = if info.is_some() {
                let value = db.storage(address, index).unwrap();
                let mut account: DbAccount = info.into();
                account.storage.insert(index, value);
                (account, value)
            } else {
                (info.into(), U256::ZERO)
            };
            self.accounts.insert(address, account);
            Ok(value)
        }
    }

    fn block_hash(&mut self, _number: U256) -> Result<B256, Self::Error> {
        unreachable!() // this should never be reached since we will never sim
                       // blocks
    }
}

impl DatabaseRef for RevmLRU {
    type Error = ();

    fn basic(&self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        match self.accounts.peek(&address) {
            Some(acc) => Ok(acc.info()),
            None => self.basic(address)
        }
    }

    fn code_by_hash(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage(&self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let db = Self::get_lastest_state_provider(Tx::new(self.db.begin_ro_txn().unwrap()));

        let mut entry_val = U256::ZERO;

        if let Some(acc_entry) = self.accounts.peek(&address) {
            if let Some(entry) = acc_entry.storage.get(&index) {
                entry_val = *entry;
            }
            if !matches!(
                acc_entry.account_state,
                AccountState::StorageCleared | AccountState::NotExisting
            ) {
                entry_val = db.storage(address, index).unwrap();
            }
        } else {
            entry_val = db.storage(address, index).unwrap();
        }

        Ok(entry_val)
    }

    fn block_hash(&self, _number: U256) -> Result<B256, Self::Error> {
        unreachable!() // this should never be reached since we will never sim
                       // blocks
    }
}
