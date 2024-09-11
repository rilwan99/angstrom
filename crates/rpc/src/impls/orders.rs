use alloy_sol_types::SolStruct;
use angstrom_types::{
    orders::OrderOrigin,
    rpc::{
        FlashOrderRequest, SignedOrder, StandingOrderRequest, TopOfBlockOrderRequest,
        ANGSTROM_DOMAIN
    },
    sol_bindings::grouped_orders::AllOrders
};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink, SubscriptionMessage};
use order_pool::OrderPoolHandle;
use reth_tasks::TaskSpawner;

use crate::{api::OrderApiServer, types::OrderSubscriptionKind};

pub struct OrderApi<OrderPool, Spawner> {
    pool:         OrderPool,
    task_spawner: Spawner
}

impl<OrderPool, Spawner> OrderApi<OrderPool, Spawner> {
    pub fn new(pool: OrderPool, task_spawner: Spawner) -> Self {
        Self { pool, task_spawner }
    }
}

#[async_trait::async_trait]
impl<OrderPool, Spawner> OrderApiServer for OrderApi<OrderPool, Spawner>
where
    OrderPool: OrderPoolHandle,
    Spawner: TaskSpawner + 'static
{
    async fn send_standing_order(
        &self,
        request: SignedOrder<StandingOrderRequest>
    ) -> RpcResult<bool> {
        let eip_hash = request.order.eip712_signing_hash(&ANGSTROM_DOMAIN);
        let signature = request.signature;
        signature
            .recover_from_msg(eip_hash)
            .map(|order| {
                self.pool
                    .new_order(OrderOrigin::External, AllOrders::Partial(request.order.into()))
            })
            .map_or(Ok(false), Ok)
    }

    async fn send_searcher_order(
        &self,
        request: SignedOrder<TopOfBlockOrderRequest>
    ) -> RpcResult<bool> {
        let eip_hash = request.order.eip712_signing_hash(&ANGSTROM_DOMAIN);
        let signature = request.signature;
        signature
            .recover_from_msg(eip_hash)
            .map(|order| {
                self.pool
                    .new_order(OrderOrigin::External, AllOrders::TOB(request.order.into()))
            })
            .map_or(Ok(false), Ok)
    }

    async fn send_flash_order(&self, request: SignedOrder<FlashOrderRequest>) -> RpcResult<bool> {
        let eip_hash = request.order.eip712_signing_hash(&ANGSTROM_DOMAIN);
        let signature = request.signature;
        signature
            .recover_from_msg(eip_hash)
            .map(|order| {
                self.pool
                    .new_order(OrderOrigin::External, AllOrders::KillOrFill(request.order.into()))
            })
            .map_or(Ok(false), Ok)
    }

    async fn subscribe_orders(
        &self,
        pending: PendingSubscriptionSink,
        kind: OrderSubscriptionKind
    ) -> jsonrpsee::core::SubscriptionResult {
        let sink = pending.accept().await?;
        let mut subscription = self.pool.subscribe_orders();

        self.task_spawner.spawn(Box::pin(async move {
            while let Ok(order) = subscription.recv().await {
                if sink.is_closed() {
                    break;
                }

                let msg = match (&kind, order) {
                    (
                        OrderSubscriptionKind::NewOrders,
                        order_pool::PoolManagerUpdate::NewOrder(order_update)
                    ) => Some(SubscriptionMessage::from_json(&order_update).unwrap()),
                    (
                        OrderSubscriptionKind::FilledOrders,
                        order_pool::PoolManagerUpdate::FilledOrder(filled_order)
                    ) => Some(SubscriptionMessage::from_json(&filled_order).unwrap()),
                    _ => None
                };

                if let Some(msg) = msg {
                    if sink.send(msg).await.is_err() {
                        break;
                    }
                }
            }
        }));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_matches::assert_matches, str::FromStr};

    use alloy_primitives::{Address, Signature, U256};
    use angstrom_network::pool_manager::OrderCommand;
    use angstrom_types::rpc::{SignedOrder, StandingOrderRequest};
    use order_pool::PoolManagerUpdate;
    use reth_tasks::TokioTaskExecutor;
    use tokio::sync::{
        broadcast::Receiver,
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}
    };

    use super::*;

    #[tokio::test]
    async fn test_send_standing_order() {
        let (mut handle, api) = setup_order_api();

        let order_request = StandingOrderRequest {
            exactIn:     false,
            amount:      1000000u128,
            minPrice:    U256::from(1000),
            useInternal: true,
            assetIn:     Address::random(),
            assetOut:    Address::random(),
            recipient:   Address::random(),
            hook:        Address::random(),
            hookPayload: vec![1, 2, 3, 4].into(),
            nonce:       12345u64,
            deadline:    1000000u64
        };
        let standing_order = SignedOrder{
            order: order_request.clone(),
            signature: Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c8041b").unwrap()
        } ;

        let res = api.send_standing_order(standing_order).await;
        assert!(res.is_ok());
        assert!(res.unwrap());
        let received_order = handle
            .from_api
            .recv()
            .await
            .expect("Should receive an order");
        assert_matches!(
            received_order,
            OrderCommand::NewOrder(OrderOrigin::External, AllOrders::Partial(_))
        );
        if let OrderCommand::NewOrder(OrderOrigin::External, AllOrders::Partial(order)) =
            received_order
        {
            assert_eq!(order, order_request.into());
        };
    }
    //
    // #[tokio::test]
    // async fn test_send_flash_order() {
    //     let (mut handle, api) = setup_order_api();
    //
    //     let flash_order = FlashOrder {
    //         mode: "market".to_string(),
    //         max_amount_in_or_out: U256::from(500000),
    //         min_price: U256::from(50),
    //         asset_in: 3u16,
    //         asset_in_form: SolAssetForm::Liquid,
    //         asset_out: 4u16,
    //         asset_out_form: SolAssetForm::Liquid,
    //         recipient: Address::random(),
    //         hook_data: vec![9, 10, 11, 12].into(),
    //         valid_for_block: 1000,
    //         signature: vec![13, 14, 15, 16].into(),
    //     };
    //     let signed_order = SignedOrder {
    //         order: flash_order.clone(),
    //         signature: vec![13, 14, 15, 16].into(),
    //     };
    //
    //     let res = api.send_flash_order(signed_order).await;
    //     assert!(res.is_ok());
    //     let received_order = handle.from_api.recv().await.expect("Should receive
    // an order");     assert_matches!(received_order,
    // OrderCommand::NewOrder(OrderOrigin::External, AllOrders::KillOrFill(_)));
    //     if let OrderCommand::NewOrder(OrderOrigin::External,
    // AllOrders::KillOrFill(order)) = received_order {         assert_eq!
    // (order, flash_order);     };
    // }
    //
    // #[tokio::test]
    // async fn test_send_top_of_block_order() {
    //     let (mut handle, api) = setup_order_api();
    //
    //     let top_of_block_order = TopOfBlockOrder {
    //         amountIn: U256::from(100000),
    //         amountOut: U256::from(90000),
    //         assetInIndex: 5,
    //         assetInForm: SolAssetForm::Liquid,
    //         assetOutIndex: 6,
    //         assetOutForm: SolAssetForm::Liquid,
    //         recipient: Address::random(),
    //         hook: Address::random(),
    //         hookPayload: vec![17, 18, 19, 20].into(),
    //         from: Address::random(),
    //         signature: vec![1,2,3,4].into(),
    //     };
    //     let signed_order = SignedOrder {
    //         order: top_of_block_order.clone(),
    //         signature: vec![1,2,3,4].into(),
    //     };
    //
    //     let res = api.send_searcher_order(signed_order).await;
    //     assert!(res.is_ok());
    //     let received_order = handle.from_api.recv().await.expect("Should receive
    // an order");     assert_matches!(received_order,
    // OrderCommand::NewOrder(OrderOrigin::External, AllOrders::TOB(_)));     if
    // let OrderCommand::NewOrder(OrderOrigin::External, AllOrders::TOB(order)) =
    // received_order {         assert_eq!(order, top_of_block_order);
    //     };
    // }

    fn setup_order_api() -> (OrderApiTestHandle, OrderApi<MockOrderPoolHandle, TokioTaskExecutor>) {
        let (to_pool, pool_rx) = unbounded_channel();
        let pool_handle = MockOrderPoolHandle { sender: to_pool };
        let task_executor = TokioTaskExecutor::default();
        let api = OrderApi::new(pool_handle.clone(), task_executor);
        let handle = OrderApiTestHandle { from_api: pool_rx };
        (handle, api)
    }

    struct OrderApiTestHandle {
        from_api: UnboundedReceiver<OrderCommand>
    }

    #[derive(Clone)]
    struct MockOrderPoolHandle {
        sender: UnboundedSender<OrderCommand>
    }

    impl OrderPoolHandle for MockOrderPoolHandle {
        fn new_order(&self, origin: OrderOrigin, order: AllOrders) -> bool {
            self.sender
                .send(OrderCommand::NewOrder(origin, order))
                .is_ok()
        }

        fn subscribe_orders(&self) -> Receiver<PoolManagerUpdate> {
            unimplemented!("Not needed for this test")
        }
    }
}
