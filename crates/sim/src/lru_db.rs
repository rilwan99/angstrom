use ethers_middleware::Middleware;
use revm::Database;
use revm_primitives::{db::DatabaseRef, *};
use schnellru::{LruMap, ByMemoryUsage};


pub struct RevmLRU<M: Middleware> {
    pub accounts: LruMap<B160, HashMap<U256, U256>, ByMemoryUsage>,
    pub contracts: HashMap<B256, Bytecode>,
    pub db: M
}


impl<M: Middleware> RevmLRU<M> {
    pub fn new(max_bytes: usize, db: M) -> Self {
        let accounts = LruMap::new(ByMemoryUsage::new(max_bytes));
        Self { accounts, contracts: HashMap::new(), db }
    }
}


impl<M: Middleware> Database for Revm<M> {
    
}