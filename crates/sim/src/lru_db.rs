use std::path::{Path, PathBuf};

use ethers_middleware::Middleware;
use reth_db::transaction::DbTx;
use reth_provider::LatestStateProviderRef;
use reth_revm::database::State;
use revm::{
    db::{AccountState, DbAccount},
    Database,
};
use revm_primitives::{db::DatabaseRef, Bytecode, *};
use schnellru::{ByMemoryUsage, LruMap};
use tokio::runtime::Handle;

use crate::reth_client::RethClient;

pub struct RevmLRU<'a, 'b, TX: DbTx<'a>> {
    pub accounts: LruMap<B160, DbAccount, ByMemoryUsage>,
    pub db: State<LatestStateProviderRef<'a, 'b, TX>>,
}

impl<'a, 'b, TX: DbTx<'a>> RevmLRU<'a, 'b, TX> {
    pub fn new(max_bytes: usize, db_path: &Path) -> Self {
        let accounts = LruMap::new(ByMemoryUsage::new(max_bytes));

        let db = reth_db::mdbx::Env::<reth_db::mdbx::WriteMap>::open(
            db_path,
            reth_db::mdbx::EnvKind::RO,
            None,
        )?;

        let tx = db.begin_ro_txn().unwrap();
        let db = LatestStateProviderRef::new(&tx);

        //let middleware = RethClient::new(db, None, handle.clone());
        Self { accounts, db }
    }
}

impl<'a, 'b, TX: DbTx<'a>> Database for RevmLRU<'a, 'b, TX> {
    type Error = ();

    fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        if let Some(a) = self.accounts.get(&address) {
            return Ok(a.info());
        } else {
            let basic = self
                .db
                .basic(address)?
                .map(|info| DbAccount { info, ..Default::default() })
                .unwrap_or_else(DbAccount::new_not_existing);

            self.accounts.insert(address, basic.clone());
            return Ok(basic.info());
        }
    }

    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let account = self.accounts.get(&address);
        if let Some(acct_entry) = account {
            if let Some(idx_entry) = acct_entry.storage.get(&index) {
                return Ok(*idx_entry);
            } else {
                if matches!(
                    acct_entry.account_state,
                    AccountState::StorageCleared | AccountState::NotExisting
                ) {
                    return Ok(U256::ZERO);
                } else {
                    let slot_val = self.db.storage(address, index)?;
                    acct_entry.storage.insert(index, slot_val);
                    return Ok(slot_val);
                }
            }
        } else {
            let info = self.db.basic(address)?;
            let (account, value) = if info.is_some() {
                let value = self.db.storage(address, index)?;
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

impl<'a, 'b, TX: DbTx<'a>> DatabaseRef for RevmLRU<'a, 'b, TX> {
    type Error = ();

    fn basic(&self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        match self.accounts.peek(&address) {
            Some(acc) => Ok(acc.info()),
            None => self.basic(address),
        }
    }

    fn code_by_hash(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage(&self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let mut entry_val = U256::ZERO;

        if let Some(acc_entry) = self.accounts.peek(&address) {
            if let Some(entry) = acc_entry.storage.get(&index) {
                entry_val = *entry;
            }
            if !matches!(
                acc_entry.account_state,
                AccountState::StorageCleared | AccountState::NotExisting
            ) {
                entry_val = self.db.storage(address, index)?;
            }
        } else {
            entry_val = self.db.storage(address, index)?;
        }

        Ok(entry_val)
    }

    fn block_hash(&self, _number: U256) -> Result<B256, Self::Error> {
        unreachable!() // this should never be reached since we will never sim
                       // blocks
    }
}
