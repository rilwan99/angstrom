use std::{collections::HashMap, sync::Arc, task::Poll};

use futures_util::{stream::FuturesUnordered, Future, StreamExt};
use tokio::{runtime::Handle, sync::mpsc::UnboundedReceiver, task::JoinHandle};

use crate::{
    errors::{SimError, SimResult},
    executor::{TaskKind, ThreadPool},
    lru_db::RevmLRU,
    slot_keeper::SlotKeeper,
    state::{AddressSlots, RevmState},
    SimEvent
};

/// revm state handler
pub struct Revm {
    transaction_rx: UnboundedReceiver<SimEvent>,
    threadpool:     ThreadPool,
    state:          Arc<RevmState>,
    slot_changes:   AddressSlots,
    slot_keeper:    SlotKeeper,
    futures:        FuturesUnordered<JoinHandle<Option<AddressSlots>>>
}

impl Revm {
    pub fn new(transaction_rx: UnboundedReceiver<SimEvent>, db: RevmLRU) -> Result<Self, SimError> {
        let threadpool = ThreadPool::new()?;
        Ok(Self {
            slot_keeper: SlotKeeper::new(db.clone(), vec![], threadpool.runtime.handle().clone()),
            transaction_rx,
            threadpool,
            state: Arc::new(RevmState::new(db)),
            slot_changes: HashMap::new(),
            futures: FuturesUnordered::new()
        })
    }

    pub fn get_threadpool_handle(&self) -> Handle {
        self.threadpool.runtime.handle().clone()
    }

    fn update_slots(&mut self, touched_slots: AddressSlots) {
        for (addr, t_slots) in touched_slots.into_iter() {
            let slot = self
                .slot_changes
                .entry(addr)
                .or_insert_with(|| HashMap::new());
            for (key, val) in t_slots.into_iter() {
                slot.insert(key, val);
            }
        }
    }

    /// handles incoming transactions from clients
    fn handle_incoming_event(&mut self, tx_type: SimEvent) {
        let state = self.state.clone();

        match tx_type {
            SimEvent::Hook(data, overrides, sender) => {}
            SimEvent::UniswapV4(tx, sender) => {}
            SimEvent::BundleTx(tx, caller_info, sender) => {
                let fut = async move {
                    let res = state.simulate_bundle(tx, caller_info);
                    let _ = if let Err(e) = res {
                        sender.send(SimResult::SimError(e))
                    } else {
                        sender.send(res.unwrap())
                    };
                };
                let _ = self.threadpool.spawn_task_as(fut, TaskKind::Blocking);
            }
            SimEvent::NewBlock(sender) => {
                let slot_changes = self.slot_changes.clone();
                let fut = async move {
                    let res = RevmState::update_evm_state(state, &slot_changes);
                    let _ = if let Err(e) = res {
                        sender.send(SimResult::SimError(e))
                    } else {
                        sender.send(SimResult::SuccessfulRevmBlockUpdate)
                    };
                };
                let _ = self.threadpool.block_on_rt(fut);
                self.slot_changes.clear();
            } /* SimEvent::NewSearcherTx(tx, caller_info, sender) => {
               *     let fut = async move {
               *         let res = state.simulate_searcher_tx(tx, caller_info);
               *         if let Err(e) = res {
               *             let _ = sender.send(SimResult::SimError(e));
               *             return None
               *         } else {
               *             let (result, slots) = res.unwrap();
               *             let _ = sender.send(result);
               *             return Some(slots)
               *         };
               *     };
               *     self.futures.push(self.threadpool.spawn_return_task_as(fut));
               * } */
        };
    }
}

impl Future for Revm {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        while let Poll::Ready(poll_tx) = this.transaction_rx.poll_recv(cx) {
            match poll_tx {
                Some(tx) => this.handle_incoming_event(tx),
                None => return Poll::Ready(())
            }
        }

        while let Poll::Ready(Some(Ok(poll_slot))) = this.futures.poll_next_unpin(cx) {
            match poll_slot {
                Some(slot) => this.update_slots(slot),
                None => ()
            }
        }
        return Poll::Pending
    }
}
