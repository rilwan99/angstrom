use ethers_middleware::Middleware;
use revm::{
    db::{AccountState, DbAccount},
    Database,
};
use revm_primitives::{db::DatabaseRef, Bytecode, *};
use schnellru::{ByMemoryUsage, LruMap};
use tokio::runtime::Handle;

use crate::reth_client::RethClient;

pub struct RevmLRU<M: Middleware> {
    pub accounts: LruMap<B160, DbAccount, ByMemoryUsage>,
    pub db: RethClient<M>,
}

impl<M: Middleware> RevmLRU<M> {
    pub fn new(max_bytes: usize, db: M, handle: Handle) -> Self {
        let accounts = LruMap::new(ByMemoryUsage::new(max_bytes));

        let middleware = RethClient::new(db, None, handle.clone());
        Self { accounts, db: middleware }
    }
}

impl<M: Middleware> Database for RevmLRU<M> {
    type Error = M::Error;

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
                    //let slot_val = Self::middleware_storage(self.handle.clone(), self.db.clone(),
                    // self.block_number, address, index)?;
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

impl<M: Middleware> DatabaseRef for RevmLRU<M> {
    type Error = M::Error;

    fn basic(&self, address: B160) -> Result<Option<AccountInfo>, M::Error> {
        match self.accounts.peek(&address) {
            Some(acc) => Ok(acc.info()),
            None => self.basic(address),
        }
    }

    fn code_by_hash(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is
                       // defined in basic()
    }

    fn storage(&self, address: B160, index: U256) -> Result<U256, M::Error> {
        // TODO: this can be simplified
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
        unreachable!() // this should never be reached since we will never sim blocks
    }
}
