use std::sync::Arc;
use ethers_middleware::Middleware;
use revm::{Database, db::DbAccount};
use revm_primitives::{Bytecode, *, db::DatabaseRef};
use ethers_core::types::{H160 as eH160, H256 as eH256, U64 as eU64, BlockId};
use schnellru::{LruMap, ByMemoryUsage};
use tokio::runtime::Handle;


pub struct RevmLRU<M: Middleware> {
    pub accounts: LruMap<B160, DbAccount, ByMemoryUsage>,
    pub contracts: HashMap<B256, Bytecode>,
    pub db: Arc<M>,
    pub block_number: BlockId,
    pub handle: Handle
}


impl<M: Middleware> RevmLRU<M> {
    pub fn new(max_bytes: usize, db: M, handle: Handle) -> Self {
        let accounts = LruMap::new(ByMemoryUsage::new(max_bytes));
        let block_number = BlockId::from(
            handle.block_on(db.get_block_number()).unwrap()
        );
        Self { accounts, contracts: HashMap::new(), db: Arc::new(db), block_number, handle }
    }

    /// internal utility function to call tokio feature and wait for output
    fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
        self.handle.block_on(f)
    }


    /// gets the basic account info from the middleware
    fn middleware_basic(&mut self, address: B160) -> Result<Option<AccountInfo>, M::Error> {
        let add = eH160::from(address.0);
        let f = async {
            let nonce = self.db.get_transaction_count(add, Some(self.block_number));
            let balance = self.db.get_balance(add, Some(self.block_number));
            let code = self.db.get_code(add, Some(self.block_number));
            tokio::join!(nonce, balance, code)
        };
        let (nonce, balance, code) = self.block_on(f);
        
        // panic on not getting data?
        let bytecode = Bytecode::new_raw(
            code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                .0,
        );
        let code_hash: B256 = KECCAK_EMPTY;
        if !bytecode.is_empty() {
            keccak256(&bytecode.original_bytes());
        }
        let db_acct = Some(AccountInfo::new(
            U256::from_limbs(
                balance
                    .unwrap_or_else(|e| panic!("ethers get balance error: {e:?}"))
                    .0,
            ),
            nonce
                .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
                .as_u64(),
            bytecode,
        ));
        Ok(db_acct)
    }
}


impl<M: Middleware> Database for RevmLRU<M> {
    type Error = M::Error;

    fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        let basic = self.accounts.get(&address);
        if let Some(a) = basic {
            return Ok(a.info())
        }
        
        let basic = self.middleware_basic(address).unwrap()
        .map(|info| DbAccount {
            info, 
            ..Default::default()
        }
        ).unwrap_or_else(DbAccount::new_not_existing);

        self.accounts.insert(address, basic.clone());

        return Ok(basic.info())
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        unreachable!() // this should never be reached since the code hash is defined in basic()
    }

    /// NEED TO IMPLEMENT
    fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error> {
        todo!()
    }

    /// NEED TO IMPLEMENT
    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        todo!()
    }
}


impl<M: Middleware> DatabaseRef for RevmLRU<M> {
    type Error = M::Error;

    #[doc = " Whether account at address exists."]
    #[doc = " Get basic account information."]
    fn basic(&self, address:B160) -> Result<Option<AccountInfo> ,Self::Error>  {
        todo!()
    }

    #[doc = " Get account code by its hash"]
    fn code_by_hash(&self, code_hash:B256) -> Result<Bytecode,Self::Error>  {
        todo!()
    }

    #[doc = " Get storage value of address at index."]
    fn storage(&self,address:B160, index:U256) -> Result<U256,Self::Error>  {
        todo!()
    }

    fn block_hash(&self, number:U256) -> Result<B256,Self::Error>  {
        todo!()
    }
}