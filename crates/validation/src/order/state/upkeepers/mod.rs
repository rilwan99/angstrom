pub mod approvals;
pub mod balances;
pub mod new_pairs;
pub mod nonces;

use std::collections::HashMap;

use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use reth_provider::StateProviderFactory;
use revm::{db::WrapDatabaseRef, interpreter::opcode, new, Database, Inspector, EVM};
use revm_primitives::{TransactTo, TxEnv};

use self::{approvals::Approvals, balances::Balances};
use crate::common::lru_db::RevmLRU;

pub struct Upkeepers {
    token_list: Vec<Address>,
    approvals:  Approvals,
    balances:   Balances
}

impl Upkeepers {}

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
    evm.database(db);
    let mut env = TxEnv::default();

    env.transact_to = TransactTo::Call(wanted_address);
    env.data = call_data;
    env.caller = prob_address.clone();
    evm.env = env;

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
        let mut env = TxEnv::default();
        env.transact_to = TransactTo::Call(wanted_address);
        env.data = call_data;
        env.caller = prob_address.clone();
        evm.env = env;
        let res = U256::from_be_slice(&evm.transact_ref().unwrap().result.output().unwrap().0);

        if res == prob_value {
            return Ok(U256::from(i))
        }
    }

    anyhow::bail!("should never be reached");

    // let output = U256::from_be_slice(&res.result.output().unwrap().0);
    //
    // let err = || anyhow::anyhow!("inspector missing slot");
    //
    // Ok(inspector
    //     .reads
    //     .remove(&wanted_address)
    //     .ok_or_else(err)?
    //     .remove(&output)
    //     .ok_or_else(err)?)
}
