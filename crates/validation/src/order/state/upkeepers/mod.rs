pub mod angstrom_tokens;
pub mod approvals;
pub mod balances;
pub mod nonces;
pub mod angstrom_pools;

use std::collections::HashMap;

use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use reth_provider::StateProviderFactory;
use revm::{db::WrapDatabaseRef, interpreter::opcode, new, Database, Inspector, EVM};
use revm_primitives::{Env, TransactTo, TxEnv};

use self::{angstrom_tokens::AngstromTokens, approvals::Approvals, balances::Balances};
use crate::common::lru_db::RevmLRU;

pub struct UserAccountDetails {
    pub token_bals:      HashMap<Address, U256>,
    pub token_approvals: HashMap<Address, U256>,
    pub is_valid_nonce:  bool,
    pub is_valid_pool:   bool,

}

pub struct Upkeepers {
    new_pairs: AngstromTokens,
    approvals: Approvals,
    balances:  Balances
}

impl Upkeepers {
    pub fn new<DB>(db: DB) -> Self {}
}

pub fn find_storage_slot<DB>(
    call_data: Bytes,
    wanted_address: Address,
    db: RevmLRU<DB>
) -> anyhow::Result<U256>
where
    DB: StateProviderFactory + Send + Sync + Clone + 'static
{
    let prob_address = Address::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0]);

    let mut evm: EVM<RevmLRU<DB>> = new();
    evm.database(db.clone());

    let mut tx_env = TxEnv::default();

    tx_env.transact_to = TransactTo::Call(wanted_address);
    tx_env.data = call_data.clone();
    tx_env.caller = prob_address.clone();
    evm.env = Env { tx: tx_env, ..Default::default() };

    let res = U256::from_be_slice(&evm.transact_ref().unwrap().result.output().unwrap().0);

    let one = U256::from(1);
    let prob_value = if res == U256::MAX { res - one } else { res + one };

    for i in 0..100 {
        let mut user_addr_encoded = prob_address.to_vec();
        user_addr_encoded.extend(U256::from(i).to_be_bytes::<32>().to_vec());

        let user_slot = U256::from_be_bytes(*keccak256(user_addr_encoded));
        let mut slot = HashMap::new();
        slot.insert(user_slot, prob_value.clone());
        let mut overrides = HashMap::new();
        overrides.insert(wanted_address.clone(), slot);
        let mut db = db.clone();
        db.set_state_overrides(overrides);

        let mut evm: EVM<RevmLRU<DB>> = new();
        evm.database(db);
        let mut tx_env = TxEnv::default();
        tx_env.transact_to = TransactTo::Call(wanted_address);
        tx_env.data = call_data.clone();
        tx_env.caller = prob_address.clone();
        evm.env = Env { tx: tx_env, ..Default::default() };

        let res = U256::from_be_slice(&evm.transact_ref().unwrap().result.output().unwrap().0);

        if res == prob_value {
            return Ok(U256::from(i))
        }
    }

    anyhow::bail!("should never be reached");
}
