use std::{
    collections::HashSet,
    sync::Arc,
    task::{Context, Poll}
};

use alloy::{
    primitives::{Address, BlockHash, BlockNumber, B256},
    sol_types::SolEvent
};
use angstrom_types::{
    contract_bindings, contract_payloads::angstrom::AngstromBundle, primitive::NewInitializedPool
};
use futures::Future;
use futures_util::{FutureExt, StreamExt};
use pade::PadeDecode;
use reth_primitives::{Receipt, TransactionSigned};
use reth_provider::{CanonStateNotification, CanonStateNotifications, Chain};
use reth_tasks::TaskSpawner;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedSender};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};

use crate::handle::{EthCommand, EthHandle};

alloy::sol!(
    event Transfer(address indexed _from, address indexed _to, uint256 _value);
    event Approval(address indexed _owner, address indexed _spender, uint256 _value);
);

/// Listens for CanonStateNotifications and sends the appropriate updates to be
/// executed by the order pool
pub struct EthDataCleanser {
    angstrom_address: Address,
    /// our command receiver
    commander:        ReceiverStream<EthCommand>,
    /// people listening to events
    event_listeners:  Vec<UnboundedSender<EthEvent>>,

    /// Notifications for Canonical Block updates
    canonical_updates: BroadcastStream<CanonStateNotification>,
    angstrom_tokens:   HashSet<Address>
}

impl EthDataCleanser {
    pub fn spawn<TP: TaskSpawner>(
        angstrom_address: Address,
        canonical_updates: CanonStateNotifications,
        tp: TP,
        tx: Sender<EthCommand>,
        rx: Receiver<EthCommand>,
        angstrom_tokens: HashSet<Address>
    ) -> anyhow::Result<EthHandle> {
        let stream = ReceiverStream::new(rx);

        let this = Self {
            angstrom_address,
            canonical_updates: BroadcastStream::new(canonical_updates),
            commander: stream,
            event_listeners: Vec::new(),
            angstrom_tokens
        };
        tp.spawn_critical("eth handle", this.boxed());

        let handle = EthHandle::new(tx);

        Ok(handle)
    }

    fn send_events(&mut self, event: EthEvent) {
        self.event_listeners
            .retain(|e| e.send(event.clone()).is_ok());
    }

    fn on_command(&mut self, command: EthCommand) {
        match command {
            EthCommand::SubscribeEthNetworkEvents(tx) => self.event_listeners.push(tx)
        }
    }

    fn on_canon_update(&mut self, canonical_updates: CanonStateNotification) {
        match canonical_updates {
            CanonStateNotification::Reorg { old, new } => self.handle_reorg(old, new),
            CanonStateNotification::Commit { new } => self.handle_commit(new)
        }
    }

    fn handle_reorg(&mut self, old: Arc<impl ChainExt>, new: Arc<impl ChainExt>) {
        let mut eoas = self.get_eoa(old.clone());
        eoas.extend(self.get_eoa(new.clone()));

        // get all reorged orders
        let old_filled: HashSet<_> = self.fetch_filled_order(&old).collect();
        let new_filled: HashSet<_> = self.fetch_filled_order(&new).collect();

        let difference: Vec<_> = old_filled.difference(&new_filled).copied().collect();
        let reorged_orders = EthEvent::ReorgedOrders(difference);

        let transitions = EthEvent::NewBlockTransitions {
            block_number:      new.tip_number(),
            filled_orders:     new_filled.into_iter().collect(),
            address_changeset: eoas
        };
        self.send_events(transitions);
        self.send_events(reorged_orders);
    }

    fn handle_commit(&mut self, new: Arc<impl ChainExt>) {
        // handle this first so the newest state is the first available
        self.handle_new_pools(new.clone());

        let filled_orders = self.fetch_filled_order(&new).collect::<Vec<_>>();

        let eoas = self.get_eoa(new.clone());

        let transitions = EthEvent::NewBlockTransitions {
            block_number: new.tip_number(),
            filled_orders,
            address_changeset: eoas
        };
        self.send_events(transitions);
    }

    fn handle_new_pools(&mut self, chain: Arc<impl ChainExt>) {
        Self::get_new_pools(&chain)
            .inspect(|pool| {
                let token_0 = pool.currency_in;
                let token_1 = pool.currency_out;
                self.angstrom_tokens.insert(token_0);
                self.angstrom_tokens.insert(token_1);
            })
            .map(EthEvent::NewPool)
            .for_each(|pool_event| {
                // didn't use send event fn because of lifetimes.
                self.event_listeners
                    .retain(|e| e.send(pool_event.clone()).is_ok());
            });
    }

    /// TODO: check contract for state change. if there is change. fetch the
    /// transaction on Angstrom and process call-data to pull order-hashes.
    fn fetch_filled_order<'a>(
        &'a self,
        chain: &'a impl ChainExt
    ) -> impl Iterator<Item = B256> + 'a {
        chain
            .tip_transactions()
            .filter(|tx| tx.transaction.to() == Some(self.angstrom_address))
            .filter_map(|transaction| {
                let mut input: &[u8] = transaction.input();
                AngstromBundle::pade_decode(&mut input, None).ok()
            })
            .flat_map(move |bundle| bundle.get_order_hashes().collect::<Vec<_>>())
    }

    /// fetches all eoa addresses touched
    fn get_eoa(&self, chain: Arc<impl ChainExt>) -> Vec<Address> {
        chain
            .receipts_by_block_hash(chain.tip_hash())
            .unwrap()
            .into_iter()
            .flat_map(|receipt| &receipt.logs)
            .filter(|log| self.angstrom_tokens.contains(&log.address))
            .flat_map(|logs| {
                Transfer::decode_log(logs, true)
                    .map(|log| log._from)
                    .or_else(|_| Approval::decode_log(logs, true).map(|log| log._owner))
            })
            .collect()
    }

    /// gets any newly initialized pools in this block
    /// do we want to use logs here?
    fn get_new_pools(chain: &impl ChainExt) -> impl Iterator<Item = NewInitializedPool> + '_ {
        chain
            .receipts_by_block_hash(chain.tip_hash())
            .unwrap()
            .into_iter()
            .flat_map(|receipt| {
                receipt.logs.iter().filter_map(|log| {
                    contract_bindings::pool_manager::PoolManager::Initialize::decode_log(log, true)
                        .map(Into::into)
                        .ok()
                })
            })
    }
}

impl Future for EthDataCleanser {
    type Output = ();

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // poll all canonical updates
        while let Poll::Ready(is_some) = self.canonical_updates.poll_next_unpin(cx).map(|res| {
            res.transpose()
                .ok()
                .flatten()
                .map(|update| self.on_canon_update(update))
                .is_some()
        }) {
            if !is_some {
                return Poll::Ready(())
            }
        }

        while let Poll::Ready(Some(command)) = self.commander.poll_next_unpin(cx) {
            self.on_command(command)
        }

        Poll::Pending
    }
}

#[derive(Debug, Clone)]
pub enum EthEvent {
    //TODO: add shit here
    NewBlock(u64),
    NewBlockTransitions {
        block_number:      u64,
        filled_orders:     Vec<B256>,
        address_changeset: Vec<Address>
    },
    ReorgedOrders(Vec<B256>),
    FinalizedBlock(u64),
    NewPool(NewInitializedPool)
}

#[auto_impl::auto_impl(&,Arc)]
pub trait ChainExt {
    fn tip_number(&self) -> BlockNumber;
    fn tip_hash(&self) -> BlockHash;
    fn receipts_by_block_hash(&self, block_hash: BlockHash) -> Option<Vec<&Receipt>>;
    fn tip_transactions(&self) -> impl Iterator<Item = &TransactionSigned> + '_;
}

impl ChainExt for Chain {
    fn tip_hash(&self) -> BlockHash {
        self.tip().hash()
    }

    fn tip_number(&self) -> BlockNumber {
        self.tip().number
    }

    fn receipts_by_block_hash(&self, block_hash: BlockHash) -> Option<Vec<&Receipt>> {
        self.receipts_by_block_hash(block_hash)
    }

    fn tip_transactions(&self) -> impl Iterator<Item = &TransactionSigned> + '_ {
        self.tip().transactions()
    }
}

#[cfg(test)]
pub mod test {
    use angstrom_types::contract_payloads::angstrom::TopOfBlockOrder;
    use reth_primitives::Transaction;
    use testing_tools::type_generator::orders::tob::ToBOrderBuilder;
    use testing_tools::type_generator::orders::user::UserOrderBuilder;

    use super::*;

    pub struct MockChain<'a> {
        pub hash:         BlockHash,
        pub number:       BlockNumber,
        pub receipts:     Option<Vec<&'a Receipt>>,
        pub transactions: Vec<TransactionSigned>
    }

    impl ChainExt for MockChain<'_> {
        fn tip_hash(&self) -> BlockHash {
            self.hash
        }

        fn tip_number(&self) -> BlockNumber {
            self.number
        }

        fn receipts_by_block_hash(&self, _: BlockHash) -> Option<Vec<&Receipt>> {
            self.receipts.clone()
        }

        fn tip_transactions(&self) -> impl Iterator<Item = &TransactionSigned> + '_ {
            self.transactions.iter()
        }
    }

    fn setup_non_subscription_eth_manager(angstrom_address: Option<Address>) -> EthDataCleanser {
        let (command_tx, command_rx) = tokio::sync::mpsc::channel(3);
        let (cannon_tx, cannon_rx) = tokio::sync::broadcast::channel(3);
        EthDataCleanser {
            commander:         ReceiverStream::new(command_rx),
            event_listeners:   vec![],
            angstrom_tokens:   HashSet::default(),
            angstrom_address:  angstrom_address.unwrap_or_default(),
            canonical_updates: BroadcastStream::new(cannon_rx)
        }
    }

    #[test]
    fn test_fetch_filled_orders() {
        let angstrom_address = Address::random();
        let eth = setup_non_subscription_eth_manager(Some(angstrom_address));

        let top_of_block_order = TobOrderBuilder::

        let angstrom_bundle_with_orders = AngstromBundle::new(vec![],vec![],vec![]);

        let mut mock_tx = TransactionSigned::default();
        if let Transaction::Legacy(leg) = &mut mock_tx.transaction {
            leg.to = TxKind::Call(angstrom_address);

        }

        let mock_chain = MockChain {
            transactions
        }
        //
        // fn fetch_filled_order<'a>(
        //     &'a self,
        //     chain: &'a impl ChainExt
        // ) -> impl Iterator<Item = B256> + 'a {
        //     chain
        //         .tip_transactions()
        //         .filter(|tx| tx.transaction.to() ==
        // Some(self.angstrom_address))
        //         .filter_map(|transaction| {
        //             let mut input: &[u8] = transaction.input();
        //             AngstromBundle::pade_decode(&mut input, None).ok()
        //         })
        //         .flat_map(move |bundle|
        // bundle.get_order_hashes().collect::<Vec<_>>()) }
    }
}
