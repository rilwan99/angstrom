use crate::{
    lru_db::RevmLRU,
    sim::{SimError, SimResult},
};
use ethers_core::{
    abi::encode,
    types::{
        transaction::{
            eip2718::TypedTransaction,
            eip712::{encode_data, EIP712Domain, Eip712Error, TypedData},
        },
        H160,
    },
    utils::rlp::{Decodable, Rlp},
};
use ethers_middleware::Middleware;
use eyre::Result;
use parking_lot::RwLock;
use reth_primitives::Signature;
use revm::{
    db::{CacheDB, DatabaseRef, DbAccount, EmptyDB},
    DatabaseCommit, EVM,
};
use revm_primitives::*;
use secp256k1::{Message, SecretKey, SECP256K1};
use std::{collections::HashMap, sync::Arc};
use tokio::{runtime::Handle, sync::oneshot::Sender};

/// struct used to share the mutable state across threads
pub struct RevmState<M: Middleware + 'static> {
    /// touched slots in tx sim
    slot_changes: HashMap<B160, HashMap<U256, U256>>,
    /// cached database for bundle state differences
    cache_db: CacheDB<EmptyDB>,
    /// evm -> holds state to sim on
    evm: EVM<RevmLRU<M>>,
}

impl<M> RevmState<M>
where
    M: Middleware,
{
    pub fn new(db: M, max_bytes: usize, handle: Handle) -> Self {
        let mut evm = EVM::new();
        evm.database(RevmLRU::new(max_bytes, db, handle));
        Self { evm, cache_db: CacheDB::new(EmptyDB {}), slot_changes: HashMap::new() }
    }

    /// resets the cache of slot changes
    pub fn reset_cache_slot_changes(state: Arc<RwLock<Self>>) {
        let mut state = state.write();
        state.slot_changes.clear();
    }

    /// updates the evm state on a new block
    /// overhead from pulling state from disk on new block??
    pub fn update_evm_state(state: Arc<RwLock<Self>>) {
        let mut state = state.write();

        for (addr, storage) in state.slot_changes.clone().into_iter() {
            let verified_storage = storage
                .iter()
                .map(|(idx, val)| {
                    let new_state = state
                        .evm
                        .db
                        .as_ref()
                        .unwrap()
                        .db
                        .storage(addr, *idx)
                        .unwrap(); // verify the touched slots
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
                let acct = evm_db
                    .accounts
                    .get_or_insert(addr, || DbAccount {
                        info: evm_db.db.basic(addr).unwrap().unwrap(),
                        ..Default::default()
                    })
                    .unwrap();
                acct.storage.insert(idx.clone(), key.clone());
            });
        }

        state.slot_changes.clear(); // reset the cache
    }

    /// simulates a single transaction and caches touched slots
    /// CHANGE TO EIP712DOMAIN
    pub fn simulate_single_tx(
        state: Arc<RwLock<Self>>,
        tx: TypedTransaction,
        client_tx: Sender<SimResult>,
    ) {
        let res = {
            let mut state = state.write();
            state.evm.env.tx = convert_type_tx(&tx);
            state.set_touched_slots()
        };

        let _ = match res {
            Ok(Some(r)) => client_tx.send(SimResult::ExecutionResult(r)),
            Ok(None) => client_tx.send(SimResult::SimulationError(SimError::CreateTransaction(tx))),
            Err(_) => client_tx.send(SimResult::SimulationError(SimError::EVMTransactError(tx))),
        };
    }

    /// simulates a bundle of transactions
    /// CHANGE TO EIP712DOMAIN
    pub fn simulate_bundle(
        state: Arc<RwLock<Self>>,
        txs: Vec<TypedTransaction>,
        client_tx: Sender<SimResult>,
    ) {
        let mut state = state.write();

        state.cache_db = CacheDB::default(); // reset the cache db before new bundle sim

        let mut sim_res: SimResult = SimResult::SuccessfulBundle;
        for tx in txs {
            state.evm.env.tx = convert_type_tx(&tx);
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

    /// updates the slots touched by a transaction if they haven't already been touched
    fn set_touched_slots(
        &mut self,
    ) -> Result<Option<ExecutionResult>, EVMError<<RevmLRU<M> as DatabaseRef>::Error>> {
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

// helper function to convert EIP712 Typed Data to TxEnv
pub fn convert_eip712(eip_tx: TypedData) -> Result<TxEnv, SimResult> {
    let verifier =
        serde_json::from_value::<H160>(eip_tx.message.get("signature").unwrap().clone()).unwrap();
    println!("VERIFIER {:#x}\n", verifier);

    let tokens = encode_data(
        &eip_tx.primary_type,
        &serde_json::Value::Object(serde_json::Map::from_iter(eip_tx.message.clone())),
        &eip_tx.types,
    )
    .map_err(|e| SimResult::SimulationError(SimError::Eip712Error(e)))?;

    let binding = encode(&tokens).clone();
    println!("BINDING {:?}\n", binding);

    let encoded = Rlp::new(&binding);
    println!("ENCODING {:?}\n", encoded);

    let typed_tx = TypedTransaction::decode(&encoded.into()).unwrap();

    Ok(convert_type_tx(&typed_tx))
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

/// Signs message with the given secret key.
/// Returns the corresponding signature.
pub fn sign_message(secret: B256, message: B256) -> Result<Signature, secp256k1::Error> {
    let sec = SecretKey::from_slice(secret.as_ref())?;
    let s = SECP256K1.sign_ecdsa_recoverable(&Message::from_slice(&message[..])?, &sec);
    let (rec_id, data) = s.serialize_compact();

    let signature = Signature {
        r: U256::try_from_be_slice(&data[..32]).expect("The slice has at most 32 bytes"),
        s: U256::try_from_be_slice(&data[32..64]).expect("The slice has at most 32 bytes"),
        odd_y_parity: rec_id.to_i32() != 0,
    };
    Ok(signature)
}

#[cfg(test)]
mod tests {
    use ethers_core::types::{transaction::eip712::Eip712, H256};
    use hex_literal::hex;
    use reth_primitives::Signature;
    use rlp::Decodable;

    use super::*;
    use std::str::FromStr;

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
            ]
          },
          "primaryType": "UserSettlement",
          "message": {
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
            "signature": "0x5FB0A325",
            "amount_out": "0x5FB0A325"
          }
        }
        );

        let typed_data: TypedData = serde_json::from_value(json).unwrap();
        println!("TYPED DATA {:?}\n", typed_data);

        let val = typed_data.message.get("order");
        println!("{:?}", val);

        //let val2 = serde_json::Value::Object(serde_json::Map::from_iter(typed_data.message.clone()));
        //println!("{:?}", val2);

        let address = H160(hex!("aeadF7A763BB3706119671043526A52c3869e42F"));
        let secret_key =
            H256(hex!("11aefec542ec498858d838ef322e6fccea3bf9c69ac54cbc8c86e4693b6879bb"));
        let hash = typed_data.encode_eip712().unwrap();

        let signer = sign_message(secret_key.0.into(), hash.into()).unwrap();
        println!("{:?}", signer);

        assert_eq!(signer.recover_signer(revm_primitives::B256(hash)).unwrap(), address.into());

        //let hash = convert_eip712(typed_data).unwrap();
        //println!("{:?}", hash);

        assert_eq!(
            "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775",
            "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775"
        );
    }
}
