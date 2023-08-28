use std::sync::Arc;
use parking_lot::RwLock;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use revm::{db::{CacheDB, DatabaseRef, EmptyDB}, EVM, DatabaseCommit};
use revm_primitives::*;
use tokio::{sync::oneshot::Sender, runtime::Handle};
use crate::{sim::{SimResult, SimError}, lru_db::RevmLRU};
use ethers_middleware::Middleware;
use eyre::Result;


/// struct used to share the mutable state across threads
pub struct RevmState<M: Middleware + 'static> {
    /// touched slots in tx sim
    slot_changes: HashMap<B160, HashMap<U256, StorageSlot>>,
    /// cached database for bundle state differences
    cache_db: CacheDB<EmptyDB>,
    /// evm -> holds state to sim on
    evm: EVM<RevmLRU<M>>
}

impl<M> RevmState<M> 
where
    M: Middleware
{
    pub fn new(db: M, max_bytes: usize, handle: Handle) -> Self {
        let mut evm = EVM::new();
        evm.database(RevmLRU::new(max_bytes, db, handle));
        Self { evm, cache_db: CacheDB::new(EmptyDB{}), slot_changes: HashMap::new() }
    }

    /// resets the cache of slot changes
    pub fn reset_cache_slot_changes(state: Arc<RwLock<Self>>) {
        let mut state = state.write();
        state.slot_changes.clear();
    }

    /// updates the evm state on a new block
    pub fn update_evm_state(state: Arc<RwLock<Self>>) {
        todo!()
        // overhead from pulling state from disk on new block??
    }

    /// simulates a single transaction and caches touched slots
    /// CHANGE TO EIP712DOMAIN
    pub fn simulate_single_tx(state: Arc<RwLock<Self>>, tx: TypedTransaction, client_tx: Sender<SimResult>) {
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
    pub fn simulate_bundle(state: Arc<RwLock<Self>>, txs: Vec<TypedTransaction>, client_tx: Sender<SimResult>) {

        let mut state = state.write();

        state.cache_db = CacheDB::default(); // reset the cache db before new bundle sim

        let mut sim_res: SimResult = SimResult::SuccessfulBundle;
        for tx in txs {
            state.evm.env.tx = convert_type_tx(&tx);
            let state_change = state.evm.transact();
            if let Ok(r) = state_change  {
                state.cache_db.commit(r.state)
            } else {
                sim_res =  SimResult::SimulationError(SimError::EVMTransactError(tx));
                break;
            };
        }
        let _ = client_tx.send(sim_res);
    }

    /// updates the slots touched by a transaction if they haven't already been touched
    fn set_touched_slots(&mut self) -> Result<Option<ExecutionResult>, EVMError<<RevmLRU<M> as DatabaseRef>::Error>> {
        let contract_address = match self.evm.env.tx.transact_to {
            TransactTo::Call(a) => a,
            TransactTo::Create(_) => return Ok(None),
        };
        let contract_slots = self.slot_changes.entry(contract_address).or_insert(HashMap::new());

        let result = self.evm.transact()?;
        let slots = &result.state.get(&contract_address).unwrap().storage;

        for (idx, slot) in slots.into_iter() {
            if slot.is_changed() {
                contract_slots.entry(*idx).or_insert_with(|| slot.clone());
            }
        }

        Ok(Some(result.result))
    }
    
}




// helper function to convert a EIP712Domain to TxEnv
/* 
pub fn convert_eip712(eip_tx: EIP712Domain) -> TxEnv {
    TxEnv { caller: eip_tx., gas_limit: (), gas_price: (), gas_priority_fee: (), transact_to: (), value: (), data: (), chain_id: (), nonce: (), access_list: () }
}
*/


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
        data: tx.data().unwrap_or(&ethers_core::types::Bytes::default()).to_vec().into(),
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    };

    tx_env
}