use std::{collections::HashMap, sync::Arc};

use ethers_core::types::transaction::{
    eip2718::TypedTransaction,
    eip712::{Eip712, TypedData},
};
use eyre::Result;
use parking_lot::RwLock;
use reth_db::mdbx::WriteMap;
use reth_primitives::Signature;
use revm::{
    db::{CacheDB, DatabaseRef, DbAccount, EmptyDB},
    DatabaseCommit, EVM,
};
use revm_primitives::*;
use shared::UserSettlement;
use tokio::sync::oneshot::Sender;

use crate::{
    lru_db::RevmLRU,
    sim::{SimError, SimResult},
};

/// struct used to share the mutable state across threads
pub struct RevmState {
    /// touched slots in tx sim
    slot_changes: HashMap<B160, HashMap<U256, U256>>,
    /// cached database for bundle state differences
    cache_db: CacheDB<EmptyDB>,
    /// evm -> holds state to sim on
    evm: EVM<RevmLRU>,
}

impl RevmState {
    pub fn new(db: Arc<reth_db::mdbx::Env<WriteMap>>, max_bytes: usize) -> Self {
        let mut evm = EVM::new();
        evm.database(RevmLRU::new(max_bytes, db));
        Self { evm, cache_db: CacheDB::new(EmptyDB {}), slot_changes: HashMap::new() }
    }

    /// resets the cache of slot changes
    pub fn reset_cache_slot_changes(state: Arc<RwLock<Self>>) {
        let mut state = state.write();
        state.slot_changes.clear();
    }

    /// updates the evm state on a new block
    /// overhead from pulling state from disk on new block??
    /// CLEAN THIS FUNCTION UP
    pub fn update_evm_state(state: Arc<RwLock<Self>>) {
        let mut state = state.write();

        /*
                let block = state.evm.db().unwrap().db.get_block_gas_limit();

                state.evm.env.block.number =
                    Into::<ethers_core::types::U256>::into(block.number.unwrap().as_u64()).into();
                state.evm.env.block.timestamp = block.timestamp.into();
                state.evm.env.block.difficulty = block.difficulty.into();
                state.evm.env.block.basefee = block.base_fee_per_gas.unwrap().into();
                state.evm.env.block.gas_limit = block.gas_limit.into();
        */
        for (addr, storage) in state.slot_changes.clone().into_iter() {
            let verified_storage = storage
                .iter()
                .map(|(idx, val)| {
                    let new_state = state.evm.db.as_ref().unwrap().storage(addr, *idx).unwrap(); // verify the touched slots
                    if new_state == *val {
                        (*idx, *val)
                    } else {
                        (*idx, new_state)
                    }
                })
                .collect::<Vec<(U256, U256)>>();

            let evm_db = state.evm.db().unwrap();
            let _ = verified_storage.into_iter().map(|(idx, key)| {
                // update the touched slots that were correct
                let acct = if let Some(a) = evm_db.accounts.get(&addr) {
                    a
                } else {
                    evm_db.accounts.insert(
                        addr,
                        DbAccount {
                            info: evm_db.basic(addr).unwrap().unwrap(),
                            ..Default::default()
                        },
                    );
                    evm_db.accounts.get(&addr).unwrap()
                };

                acct.storage.insert(idx.clone(), key.clone());
            });
        }

        state.slot_changes.clear(); // reset the cache
    }

    /// simulates a single transaction and caches touched slots
    pub fn simulate_single_tx(
        state: Arc<RwLock<Self>>,
        tx: TypedData,
        client_tx: Sender<SimResult>,
    ) {
        let tx = &convert_eip712(tx).unwrap()[0];

        let res = {
            let mut state = state.write();
            state.evm.env.tx = tx.clone();
            state.set_touched_slots()
        };

        let _ =
            match res {
                Ok(Some(r)) => client_tx.send(SimResult::ExecutionResult(r)),
                Ok(None) => client_tx
                    .send(SimResult::SimulationError(SimError::CreateTransaction(tx.clone()))),
                Err(_) => client_tx
                    .send(SimResult::SimulationError(SimError::EVMTransactError(tx.clone()))),
            };
    }

    /// simulates a bundle of transactions
    pub fn simulate_bundle(state: Arc<RwLock<Self>>, txs: TypedData, client_tx: Sender<SimResult>) {
        let txs = convert_eip712(txs).unwrap();
        let mut state = state.write();

        state.cache_db = CacheDB::default(); // reset the cache db before new bundle sim

        let mut sim_res: SimResult = SimResult::SuccessfulBundle;
        for tx in txs {
            state.evm.env.tx = tx.clone();
            let state_change = state.evm.transact();
            if let Ok(r) = state_change {
                state.cache_db.commit(r.state)
            } else {
                sim_res = SimResult::SimulationError(SimError::EVMTransactError(tx));
                break;
            };
        }
        let _ = client_tx.send(sim_res);
    }

    /// updates the slots touched by a transaction if they haven't already been
    /// touched
    fn set_touched_slots(
        &mut self,
    ) -> Result<Option<ExecutionResult>, EVMError<<RevmLRU as DatabaseRef>::Error>> {
        let contract_address = match self.evm.env.tx.transact_to {
            TransactTo::Call(a) => a,
            TransactTo::Create(_) => return Ok(None),
        };
        let contract_slots = self
            .slot_changes
            .entry(contract_address)
            .or_insert(HashMap::new());

        let result = self.evm.transact()?;
        let slots = &result.state.get(&contract_address).unwrap().storage;

        for (idx, slot) in slots.into_iter() {
            if slot.is_changed() {
                contract_slots
                    .entry(*idx)
                    .or_insert_with(|| slot.present_value.clone());
            }
        }

        Ok(Some(result.result))
    }
}

/// helper function to convert EIP712 Typed Data to TxEnv
/// ADD FUNCTION DATA WHEN FINIALIZED
pub fn convert_eip712(eip_typed_data: TypedData) -> Result<Vec<TxEnv>, SimResult> {
    let hash = eip_typed_data.encode_eip712().unwrap();

    let user_txs: Vec<UserSettlement> =
        serde_json::from_value(eip_typed_data.message.get("users").unwrap().clone()).unwrap();

    let address = eip_typed_data.domain.verifying_contract.unwrap();

    let mut transactions = Vec::new();
    for tx in user_txs {
        let sig = Signature::decode(&mut &tx.signature[..]).unwrap();
        let from = sig.recover_signer(hash.into()).unwrap();
        let gas_limit = tx.order.gas_bid;
        let data = Bytes::default(); // add data

        let tx_env = TxEnv {
            caller: from,
            gas_limit: gas_limit.as_u64(),
            gas_price: U256::ZERO,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(address.into()),
            value: U256::ZERO,
            data,
            chain_id: eip_typed_data.domain.chain_id.map(|c| c.as_u64().into()),
            nonce: None,
            access_list: Vec::new(),
        };
        transactions.push(tx_env)
    }

    Ok(transactions)
}

// helper function to convert a TypedTransaction to TxEnv
pub fn convert_type_tx(tx: &TypedTransaction) -> TxEnv {
    let transact_to = match tx.to_addr() {
        Some(to) => TransactTo::Call(B160::from(*to)),
        None => TransactTo::Create(CreateScheme::Create),
    };

    let tx_env = TxEnv {
        caller: Into::<B160>::into(*tx.from().unwrap()),
        gas_limit: u64::MAX,
        gas_price: U256::ZERO,
        gas_priority_fee: None,
        transact_to,
        value: U256::ZERO,
        data: tx
            .data()
            .unwrap_or(&ethers_core::types::Bytes::default())
            .to_vec()
            .into(),
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    };

    tx_env
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use ethers_core::types::{transaction::eip712::Eip712, Bytes, H256};
    use hex_literal::hex;
    use shared::UserSettlement;

    #[test]
    fn test_hash_custom_data_type() {
        let json = serde_json::json!({
            "domain": {
                "name": "ExampleDApp",
                "version": "1",
                "chainId": 1,
                "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"
            },
            "types": {
                "EIP712Domain": [
                    { "name": "name", "type": "string" },
                    { "name": "version", "type": "string" },
                    { "name": "chainId", "type": "uint256" },
                    { "name": "verifyingContract", "type": "address" }
                ],
                "UserOrder": [
                    { "name": "token_out", "type": "address" },
                    { "name": "token_in", "type": "address" },
                    { "name": "amount_in", "type": "uint128" },
                    { "name": "amount_out_min", "type": "uint128" },
                    { "name": "deadline", "type": "uint256" },
                    { "name": "gas_bid", "type": "uint256" },
                    { "name": "pre_hook", "type": "bytes" },
                    { "name": "post_hook", "type": "bytes" }
                ],
                "UserSettlement": [
                    { "name": "order", "type": "UserOrder" },
                    { "name": "signature", "type": "bytes" },
                    { "name": "amount_out", "type": "uint256" }
                ],
                "SealedBundle": [
                    { "name": "arbs", "type": "SealedOrder[]" },
                    { "name": "pools", "type": "PoolSettlement[]" },
                    { "name": "users", "type": "UserSettlement[]" }
                ]
            },
            "primaryType": "SealedBundle",
            "message": {
                "arbs": [],
                "pools": [],
                "users": [
                    {
                        "order": {
                            "token_out": "0xaaadF7A763BB3706119671043526A52c3869e42F",
                            "token_in": "0xabadF7A763BB3706119671043526A52c3869e42F",
                            "amount_in": 100,
                            "amount_out_min": 50,
                            "deadline": "0x5FB0A325",
                            "gas_bid": "0x5FB0A325",
                            "pre_hook": "0x5FB0A325",
                            "post_hook": "0x5FB0A325"
                        },
                        "signature": "0x3004f60be5a7d84e6fa12294c451faedf60d8701ccd85f0ae0c3149097a029827bf6494f51f079a214d4336437239592becb29ab6da61e89d3cb4eb38bd720da1b",
                        "amount_out": "0x5FB0A325"
                    },
                    {
                        "order": {
                            "token_out": "0xaaadF7A763BB3706119671043526A52c3869e42F",
                            "token_in": "0xabadF7A763BB3706119671043526A52c3869e42F",
                            "amount_in": 100,
                            "amount_out_min": 50,
                            "deadline": "0x5FB0A325",
                            "gas_bid": "0x5FB0A325",
                            "pre_hook": "0x5FB0A325",
                            "post_hook": "0x5FB0A325"
                        },
                        "signature": "0xaa04f60be5a7d84e6fa12294a451faedf60d8701ccd85f0ae0c3149097a029827bf6494f51f079a214d4336437239592becb29ab6da61e89d3cb4eb38bd720da1b",
                        "amount_out": "0x5FB0A325"
                    }
                ]
            }
        });

        let typed_data: TypedData = serde_json::from_value(json).unwrap();
        println!("TYPED DATA {:?}\n", typed_data);

        let val: Vec<UserSettlement> =
            serde_json::from_value(typed_data.message.get("users").unwrap().clone()).unwrap();
        println!("{:?}", val);

        //let val2 = serde_json::Value::Object(serde_json::Map::from_iter(typed_data.message.clone()));
        //println!("{:?}", val2);

        let address = H160(hex!("aeadF7A763BB3706119671043526A52c3869e42F"));
        let secret_key =
            H256(hex!("11aefec542ec498858d838ef322e6fccea3bf9c69ac54cbc8c86e4693b6879bb"));

        let hash = typed_data.encode_eip712().unwrap();

        let signer = sign_message(secret_key.0.into(), hash.into()).unwrap();
        println!("{:?}", hex::encode(signer.to_bytes()));

        assert_eq!(signer.recover_signer(revm_primitives::B256(hash)).unwrap(), address.into());
    }
}
*/
