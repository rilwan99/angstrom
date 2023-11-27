pub mod approvals;
pub mod balances;
pub mod new_pairs;
pub mod nonces;

use std::collections::HashMap;

use alloy_primitives::{Address, Bytes, B256, U256};
use reth_provider::StateProviderFactory;
use revm::{db::WrapDatabaseRef, interpreter::opcode, new, Database, Inspector, EVM};

use crate::common::lru_db::RevmLRU;

pub struct Upkeepers {
    token_list: Vec<Address>
}

#[derive(Debug, Default)]
struct SlotInspector {
    reads:          HashMap<Address, HashMap<U256, B256>>,
    pending_value:  Option<B256>,
    last_was_sload: bool
}

impl<DB: Database> Inspector<DB> for SlotInspector {
    fn step(
        &mut self,
        interp: &mut revm::interpreter::Interpreter<'_>,
        data: &mut revm::EVMData<'_, DB>
    ) {
        if let opcode::SLOAD = interp.current_opcode() {
            if let Ok(slot) = interp.stack().peek(0) {
                self.pending_value = Some(B256::from(slot.to_be_bytes()));
                self.last_was_sload = true
            }
        }
    }

    fn step_end(
        &mut self,
        interp: &mut revm::interpreter::Interpreter<'_>,
        data: &mut revm::EVMData<'_, DB>
    ) {
        if self.last_was_sload {
            let storage_return = interp.stack().peek(0).expect("unreachable!");
            if self
                .reads
                .entry(interp.contract().address)
                .or_default()
                .insert(storage_return, self.pending_value.take().unwrap())
                .is_some()
            {
                panic!("Two storage slots with the same value");
            }
        }
        self.last_was_sload = false;
    }
}

/// This uses a method called storage slot probing to find the wanted slot for a
/// call data output. we trace this call and find the exact storage slot it
/// reads from for the result. The return value is the index i of the mapping
/// location. Mappings work by taking the hash of the concatenated
/// (USER_ADDRESS, i) where i is the location of the mapping. We find the
/// storage slots that link
pub fn find_storage_slot<DB>(call_data: Bytes, wanted_address: Address, db: RevmLRU<DB>) -> U256
where
    DB: StateProviderFactory + Send + Sync + Clone + 'static
{
    let inspector = SlotInspector::default();

    let mut evm: EVM<RevmLRU<DB>> = new();
    evm.database(db);

    todo!()
}
