use ethers_middleware::Middleware;
use revm::Database;
use revm_primitives::{AccountInfo, Bytecode, B160, B256, KECCAK_EMPTY, U256, db::DatabaseRef};
use ethers_core::types::{BlockId, H160 as eH160, H256, U64 as eU64};
use std::sync::Arc;
use tokio::runtime::Handle;


pub struct RevmMiddleware<M>
where
    M: Middleware,
{
    pub client: Arc<M>,
    runtime_handle: Handle,
    block_number: Option<BlockId>,
}

impl<M> RevmMiddleware<M>
where
    M: Middleware,
{
    /// create ethers db connector inputs are url and block on what we are basing our database (None for latest)
    pub fn new(client: M, block_number: Option<BlockId>, handle: Handle) -> Self {
        let mut out = Self {
            client: Arc::new(client),
            runtime_handle: handle,
            block_number: None,
        };

        out.block_number = if block_number.is_some() {
            block_number
        } else {
            Some(BlockId::from(
                out.block_on(out.client.get_block_number()).ok().unwrap(),
            ))
        };

        out
    }

    /// internal utility function to call tokio feature and wait for output
    fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
       self.runtime_handle.block_on(f)
    }
}

impl<M> Database for RevmMiddleware<M>
where
    M: Middleware,
{
    type Error = M::Error;

    fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        let add = eH160::from(address.0);

        let f = async {
            let nonce = self.client.get_transaction_count(add, self.block_number);
            let balance = self.client.get_balance(add, self.block_number);
            let code = self.client.get_code(add, self.block_number);
            tokio::join!(nonce, balance, code)
        };
        let (nonce, balance, code) = self.block_on(f);
        // panic on not getting data?
        let bytecode = Bytecode::new_raw(
            code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                .0,
        );
        Ok(Some(AccountInfo::new(
            U256::from_limbs(
                balance
                    .unwrap_or_else(|e| panic!("ethers get balance error: {e:?}"))
                    .0,
            ),
            nonce
                .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
                .as_u64(),
            bytecode,
        )))
    }

    fn code_by_hash(&mut self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        panic!("Should not be called. Code is already loaded");
        // not needed because we already load code with basic info
    }

    fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let add = eH160::from(address.0);
        let index = H256::from(index.to_be_bytes());
        let f = async {
            let storage = self
                .client
                .get_storage_at(add, index, self.block_number)
                .await
                .unwrap();
            U256::from_be_bytes(storage.to_fixed_bytes())
        };
        Ok(self.block_on(f))
    }

    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        // saturate usize
        if number > U256::from(u64::MAX) {
            return Ok(KECCAK_EMPTY);
        }
        let number = eU64::from(u64::try_from(number).unwrap());
        let f = async {
            self.client
                .get_block(BlockId::from(number))
                .await
                .ok()
                .flatten()
        };
        Ok(B256(self.block_on(f).unwrap().hash.unwrap().0))
    }
}


impl<M> DatabaseRef for RevmMiddleware<M>
where
    M: Middleware,
{
    type Error = M::Error;

    fn basic(&self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        let add = eH160::from(address.0);

        let f = async {
            let nonce = self.client.get_transaction_count(add, self.block_number);
            let balance = self.client.get_balance(add, self.block_number);
            let code = self.client.get_code(add, self.block_number);
            tokio::join!(nonce, balance, code)
        };
        let (nonce, balance, code) = self.block_on(f);
        // panic on not getting data?
        let bytecode = Bytecode::new_raw(
            code.unwrap_or_else(|e| panic!("ethers get code error: {e:?}"))
                .0,
        );
        Ok(Some(AccountInfo::new(
            U256::from_limbs(
                balance
                    .unwrap_or_else(|e| panic!("ethers get balance error: {e:?}"))
                    .0,
            ),
            nonce
                .unwrap_or_else(|e| panic!("ethers get nonce error: {e:?}"))
                .as_u64(),
            bytecode,
        )))
    }

    fn code_by_hash(&self, _code_hash: B256) -> Result<Bytecode, Self::Error> {
        panic!("Should not be called. Code is already loaded");
        // not needed because we already load code with basic info
    }

    fn storage(&self, address: B160, index: U256) -> Result<U256, Self::Error> {
        let add = eH160::from(address.0);
        let index = H256::from(index.to_be_bytes());
        let f = async {
            let storage = self
                .client
                .get_storage_at(add, index, self.block_number)
                .await
                .unwrap();
            U256::from_be_bytes(storage.to_fixed_bytes())
        };
        Ok(self.block_on(f))
    }

    fn block_hash(&self, number: U256) -> Result<B256, Self::Error> {
        // saturate usize
        if number > U256::from(u64::MAX) {
            return Ok(KECCAK_EMPTY);
        }
        let number = eU64::from(u64::try_from(number).unwrap());
        let f = async {
            self.client
                .get_block(BlockId::from(number))
                .await
                .ok()
                .flatten()
        };
        Ok(B256(self.block_on(f).unwrap().hash.unwrap().0))
    }
}