use std::{
    collections::HashMap,
    pin::Pin,
    sync::{mpsc::Receiver, Arc},
    task::{Context, Poll},
    time::{Duration, SystemTime, UNIX_EPOCH}
};

use alloy_primitives::{B256, U256};
use angstrom_types::{
    orders::{OrderId, OrderOrigin, OrderSet},
    primitive::PoolId,
    sol_bindings::{
        grouped_orders::{AllOrders, OrderWithStorageData, *},
        rpc_orders::TopOfBlockOrder
    }
};
use futures_util::{Stream, StreamExt};
use reth_network_peers::PeerId;
use reth_primitives::Address;
use tokio::sync::oneshot::Sender;
use tracing::{error, trace};
use validation::order::{OrderValidationResults, OrderValidatorHandle};

use crate::{
    order_storage::OrderStorage,
    validator::{OrderValidator, OrderValidatorRes},
    PoolManagerUpdate,
    PoolManagerUpdate::NewOrder
};

/// This is used to remove validated orders. During validation
/// the same check wil be ran but with more accuracy
const ETH_BLOCK_TIME: Duration = Duration::from_secs(12);

pub struct OrderIndexer<V: OrderValidatorHandle> {
    /// order storage
    order_storage:          Arc<OrderStorage>,
    /// Address to order id, used for eoa invalidation
    address_to_orders:      HashMap<Address, Vec<OrderId>>,
    /// current block_number
    block_number:           u64,
    /// Order hash to order id, used for order inclusion lookups
    hash_to_order_id:       HashMap<B256, OrderId>,
    /// Orders that are being validated
    pending_order_indexing: HashMap<B256, Vec<PeerId>>,
    /// Order Validator
    validator:              OrderValidator<V>,
    // list of subscribers to a specific order validation
    order_validation_subs:  HashMap<B256, Vec<Sender<OrderValidationResults>>>,
    orders_subscriber_tx:   tokio::sync::broadcast::Sender<PoolManagerUpdate>
}

impl<V: OrderValidatorHandle<Order = AllOrders>> OrderIndexer<V> {
    pub fn new(
        validator: V,
        order_storage: Arc<OrderStorage>,
        block_number: u64,
        orders_subscriber_tx: tokio::sync::broadcast::Sender<PoolManagerUpdate>
    ) -> Self {
        Self {
            order_storage,
            block_number,
            address_to_orders: HashMap::new(),
            hash_to_order_id: HashMap::new(),
            pending_order_indexing: HashMap::new(),
            order_validation_subs: HashMap::new(),
            validator: OrderValidator::new(validator),
            orders_subscriber_tx
        }
    }

    pub fn is_valid_order(&self, order: &AllOrders) -> bool {
        let hash = order.order_hash();
        self.hash_to_order_id.contains_key(&hash)
    }

    pub fn new_rpc_order(
        &mut self,
        origin: OrderOrigin,
        order: AllOrders,
        validation_tx: tokio::sync::oneshot::Sender<OrderValidationResults>
    ) {
        self.new_order(None, origin, order, validation_tx)
    }

    pub fn new_network_order(
        &mut self,
        peer_id: PeerId,
        origin: OrderOrigin,
        order: AllOrders,
        validation_tx: tokio::sync::oneshot::Sender<OrderValidationResults>
    ) {
        self.new_order(Some(peer_id), origin, order, validation_tx)
    }

    fn new_order(
        &mut self,
        peer_id: Option<PeerId>,
        origin: OrderOrigin,
        order: AllOrders,
        validation_tx: Sender<OrderValidationResults>
    ) {
        // what is the best way to handle duplicates for:
        //  1. rpc validation (valid/invalid order)
        //  2. peer validation (valid/invalid order)
        // if nothing is returned, the channel will block forever
        // we probably want to cache the validation result to avoid extra computation?
        // what about the case when a *peer* spams an invalid order?
        if self.is_duplicate(&order) {
            return
        }

        if let Err(e) = self
            .orders_subscriber_tx
            .send(PoolManagerUpdate::NewOrder(order.clone()))
        {
            error!("could not send new order update {:?}", e)
        }

        let hash = order.order_hash();
        if let Some(peer) = peer_id {
            self.pending_order_indexing
                .entry(hash)
                .or_default()
                .push(peer);
        }

        self.order_validation_subs
            .entry(hash)
            .or_default()
            .push(validation_tx);

        self.validator.validate_order(origin, order);
    }

    fn is_duplicate(&self, order: &AllOrders) -> bool {
        let hash = order.order_hash();
        if self.hash_to_order_id.contains_key(&hash)
            || self.pending_order_indexing.contains_key(&hash)
        {
            trace!(?hash, "got duplicate order");
            return true
        }

        false
    }

    /// used to remove orders that expire before the next ethereum block
    fn remove_expired_orders(&mut self, block_number: u64) -> Vec<B256> {
        self.block_number = block_number;
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiry_deadline = U256::from((time + ETH_BLOCK_TIME).as_secs()); // grab all expired hashes
        let hashes = self
            .hash_to_order_id
            .iter()
            .filter(|(_, v)| {
                v.deadline.map(|i| i <= expiry_deadline).unwrap_or_default()
                    || v.flash_block.map(|b| b != block_number).unwrap_or_default()
            })
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();

        // TODO: notify rpc of dead orders
        let _expired_orders = hashes
            .iter()
            // remove hash from id
            .map(|hash| self.hash_to_order_id.remove(hash).unwrap())
            .inspect(|order_id| {
                self.address_to_orders
                    .values_mut()
                    // remove from address to orders
                    .for_each(|v| v.retain(|o| o != order_id));
            })
            // remove from all underlying pools
            .filter_map(|id| match id.location {
                angstrom_types::orders::OrderLocation::Searcher => {
                    self.order_storage.remove_searcher_order(&id)
                }
                angstrom_types::orders::OrderLocation::Limit => {
                    self.order_storage.remove_limit_order(&id)
                }
            })
            .collect::<Vec<_>>();

        hashes
    }

    fn eoa_state_change(&mut self, eoas: &[Address]) {
        eoas.iter()
            .filter_map(|eoa| self.address_to_orders.remove(eoa))
            .for_each(|order_ids| {
                order_ids.into_iter().for_each(|id| {
                    let Some(order) = (match id.location {
                        angstrom_types::orders::OrderLocation::Limit => {
                            self.order_storage.remove_limit_order(&id)
                        }
                        angstrom_types::orders::OrderLocation::Searcher => {
                            self.order_storage.remove_searcher_order(&id)
                        }
                    }) else {
                        return
                    };

                    self.validator
                        .validate_order(OrderOrigin::Local, order.order);
                })
            });
    }

    pub fn finalized_block(&mut self, block: u64) {
        self.order_storage.finalized_block(block);
    }

    pub fn reorg(&mut self, orders: Vec<B256>) {
        self.order_storage
            .reorg(orders)
            .into_iter()
            .for_each(|order| {
                if let Err(e) = self
                    .orders_subscriber_tx
                    .send(PoolManagerUpdate::UnfilledOrders(order.clone()))
                {
                    error!("could not send new order update {:?}", e)
                }
                self.validator.validate_order(OrderOrigin::Local, order)
            });
    }

    fn subscribe_order_events(&self) -> tokio::sync::broadcast::Receiver<PoolManagerUpdate> {
        self.orders_subscriber_tx.subscribe()
    }

    /// Removes all filled orders from the pools and moves to regular pool
    fn filled_orders(&mut self, block: u64, orders: &[B256]) {
        if orders.is_empty() {
            return
        }

        let filled_orders = orders
            .iter()
            .filter_map(|hash| self.hash_to_order_id.remove(hash))
            .filter_map(|order_id| match order_id.location {
                angstrom_types::orders::OrderLocation::Limit => {
                    self.order_storage.remove_limit_order(&order_id)
                }
                angstrom_types::orders::OrderLocation::Searcher => {
                    self.order_storage.remove_searcher_order(&order_id)
                }
            })
            .collect::<Vec<OrderWithStorageData<AllOrders>>>();

        for order in filled_orders.iter() {
            if let Err(e) = self
                .orders_subscriber_tx
                .send(PoolManagerUpdate::FilledOrder((block, order.order.clone())))
            {
                tracing::error!("could not send filled order update {:?}", e)
            }
        }
        self.order_storage.add_filled_orders(block, filled_orders);
    }

    /// Given the nonce ordering rule. Sometimes new transactions can park old
    /// transactions.
    fn park_transactions(&mut self, txes: &[B256]) {
        let order_info = txes
            .iter()
            .filter_map(|tx_hash| self.hash_to_order_id.get(tx_hash))
            .collect::<Vec<_>>();

        self.order_storage.park_orders(order_info);
    }

    fn handle_validated_order(
        &mut self,
        res: OrderValidationResults
    ) -> eyre::Result<PoolInnerEvent> {
        let res = match res {
            OrderValidationResults::Valid(valid) => {
                if let Some(subscribers) = self.order_validation_subs.remove(&valid.order_hash()) {
                    for subscriber in subscribers {
                        if let Err(e) =
                            subscriber.send(OrderValidationResults::Valid(valid.clone()))
                        {
                            error!("Failed to send order validation result to subscriber: {:?}", e);
                        }
                    }
                }
                valid
            }
            OrderValidationResults::Invalid(bad_hash) => {
                if let Some(subscribers) = self.order_validation_subs.remove(&bad_hash) {
                    for subscriber in subscribers {
                        if let Err(e) =
                            subscriber.send(OrderValidationResults::Invalid(bad_hash.clone()))
                        {
                            error!("Failed to send order validation result to subscriber: {:?}", e);
                        }
                    }
                }

                let peers = self
                    .pending_order_indexing
                    .remove(&bad_hash)
                    .unwrap_or_default();
                return Ok(PoolInnerEvent::BadOrderMessages(peers));
            }
            OrderValidationResults::TransitionedToBlock => return Ok(PoolInnerEvent::None)
        };

        if res.valid_block == self.block_number {
            let to_propagate = res.order.clone();
            // set tracking
            self.update_order_tracking(&res);
            // move orders that this invalidates to pending pools.
            self.park_transactions(&res.invalidates);

            // insert
            match res.order_id.location {
                angstrom_types::orders::OrderLocation::Searcher => {
                    self.order_storage.add_new_searcher_order(
                        res.try_map_inner(|inner| {
                            let AllOrders::TOB(order) = inner else { eyre::bail!("unreachable") };
                            Ok(order)
                        })
                        .expect("should be unreachable")
                    )?;
                }
                angstrom_types::orders::OrderLocation::Limit => {
                    self.order_storage.add_new_limit_order(
                        res.try_map_inner(|inner| {
                            Ok(match inner {
                                // TODO: will rewire when we have composable orders. good chance we
                                // will just trait this so we can get rid of match statement
                                AllOrders::Standing(p) => {
                                    // if p.hook_data.is_empty() {
                                    GroupedUserOrder::Vanilla(GroupedVanillaOrder::Partial(p))
                                    // } else {
                                    //     GroupedUserOrder::Composable(
                                    //         GroupedComposableOrder::Partial(p)
                                    //     )
                                    // }
                                }
                                // TODO: will rewire when we have composable orders. good chance we
                                AllOrders::Flash(kof) => {
                                    // if kof.hook_data.is_empty() {
                                    GroupedUserOrder::Vanilla(GroupedVanillaOrder::KillOrFill(kof))
                                    // } else {
                                    //     GroupedUserOrder::Composable(
                                    //         GroupedComposableOrder::KillOrFill(kof)
                                    //     )
                                    // }
                                }
                                _ => eyre::bail!("unreachable")
                            })
                        })
                        .expect("should be unreachable")
                    )?;
                }
            }

            return Ok(PoolInnerEvent::Propagation(to_propagate))
        }

        // bad order
        let hash = res.order_hash();
        let peers = self
            .pending_order_indexing
            .remove(&hash)
            .unwrap_or_default();

        Ok(PoolInnerEvent::BadOrderMessages(peers))
    }

    fn update_order_tracking(&mut self, order: &OrderWithStorageData<AllOrders>) {
        let hash = order.order_hash();
        let user = order.from();
        let id: OrderId = order.order_id;

        self.pending_order_indexing.remove(&hash);
        self.hash_to_order_id.insert(hash, id);
        // nonce overlap is checked during validation so its ok we
        // don't check for duplicates
        self.address_to_orders.entry(user).or_default().push(id);
    }

    pub fn get_all_orders(&self) -> OrderSet<GroupedVanillaOrder, TopOfBlockOrder> {
        self.order_storage.get_all_orders()
    }

    pub fn start_new_block_processing(
        &mut self,
        block: u64,
        completed_orders: Vec<B256>,
        address_changes: Vec<Address>
    ) {
        tracing::info!(%block, "starting transition to new block processing");
        self.validator
            .on_new_block(block, completed_orders, address_changes);
    }

    fn finish_new_block_processing(
        &mut self,
        block: u64,
        mut completed_orders: Vec<B256>,
        address_changes: Vec<Address>
    ) {
        // deal with changed orders
        self.eoa_state_change(&address_changes);
        // deal with filled orders
        self.filled_orders(block, &completed_orders);
        // add expired orders to completed
        completed_orders.extend(self.remove_expired_orders(block));

        self.validator
            .notify_validation_on_changes(block, completed_orders, address_changes);
    }
}

impl<V> Stream for OrderIndexer<V>
where
    V: OrderValidatorHandle<Order = AllOrders>
{
    type Item = Vec<PoolInnerEvent>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut validated = Vec::new();

        while let Poll::Ready(Some(next)) = self.validator.poll_next_unpin(cx) {
            match next {
                OrderValidatorRes::EnsureClearForTransition { block, orders, addresses } => {
                    self.finish_new_block_processing(block, orders, addresses);
                }
                OrderValidatorRes::ValidatedOrder(next) => {
                    if let Ok(prop) = self.handle_validated_order(next) {
                        validated.push(prop);
                    }
                }
            }
        }

        if validated.is_empty() {
            Poll::Pending
        } else {
            Poll::Ready(Some(validated))
        }
    }
}

pub enum PoolInnerEvent {
    Propagation(AllOrders),
    BadOrderMessages(Vec<PeerId>),
    None
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum PoolError {
    #[error("Pool has reached max size, and order doesn't satisify replacment requirements")]
    MaxSize,
    #[error("No pool was found for address: {0}")]
    NoPool(PoolId),
    #[error("Already have a ordered with {0:?}")]
    DuplicateNonce(OrderId),
    #[error("Duplicate order")]
    DuplicateOrder
}
