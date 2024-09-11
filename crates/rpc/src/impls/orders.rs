use alloy_primitives::Bytes;
use alloy_sol_types::SolType;
use angstrom_types::{
    orders::OrderOrigin,
    sol_bindings::{
        grouped_orders::AllOrders,
        sol::{FlashOrder, StandingOrder, TopOfBlockOrder}
    }
};
use jsonrpsee::{core::RpcResult, PendingSubscriptionSink, SubscriptionMessage};
use order_pool::OrderPoolHandle;
use reth_tasks::TaskSpawner;

use crate::{api::OrderApiServer, types::OrderSubscriptionKind};

pub struct OrderApi<OrderPool, Spawner> {
    pool:         OrderPool,
    task_spawner: Spawner,
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
    async fn send_limit_order(&self, order: Bytes) -> RpcResult<bool> {
        StandingOrder::abi_decode(order.as_ref(), true)
            .map(|order| self.pool.new_order(OrderOrigin::External, AllOrders::Partial(order)))
            .map_or(Ok(false), Ok)
    }

    async fn send_searcher_order(&self, order: Bytes) -> RpcResult<bool> {
        TopOfBlockOrder::abi_decode(order.as_ref(), true)
            .map(|order| self.pool.new_order(OrderOrigin::External, AllOrders::TOB(order)))
            .map_or(Ok(false), Ok)
    }

    async fn send_flash_order(&self, order: Bytes) -> RpcResult<bool> {
        FlashOrder::abi_decode(order.as_ref(), true)
            .map(|order| self.pool.new_order(OrderOrigin::External, AllOrders::KillOrFill(order)))
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
                    (OrderSubscriptionKind::NewOrders, order_pool::PoolManagerUpdate::NewOrder(order_update)) => {
                        Some(SubscriptionMessage::from_json(&order_update).unwrap())
                    },
                    (OrderSubscriptionKind::FilledOrders, order_pool::PoolManagerUpdate::FilledOrder(filled_order)) => {
                        Some(SubscriptionMessage::from_json(&filled_order).unwrap())
                    },
                    _ => None,
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
    use super::*;
    use alloy_primitives::{Address, U256};
    use alloy_sol_types::SolValue;
    use angstrom_network::pool_manager::OrderCommand;
    use angstrom_types::sol_bindings::sol::{FlashOrder, SolAssetForm, StandingOrder, TopOfBlockOrder};
    use order_pool::PoolManagerUpdate;
    use reth_tasks::TokioTaskExecutor;
    use std::assert_matches::assert_matches;
    use tokio::sync::broadcast::Receiver;
    use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

    #[tokio::test]
    async fn test_send_standing_order() {
        let (mut handle, api) = setup_order_api();

        let standing_order = StandingOrder {
            mode: "limit".to_string(),
            max_amount_in_or_out: U256::from(1000000),
            min_price: U256::from(100),
            asset_in: 1u16,
            asset_in_form: SolAssetForm::Liquid,
            asset_out: 2u16,
            asset_out_form: SolAssetForm::Liquid,
            recipient: Address::random(),
            hook_data: vec![1, 2, 3, 4].into(),
            nonce: 12345,
            deadline: U256::from(1000000000),
            signature: vec![5, 6, 7, 8].into(),
        };
        let encoded = standing_order.abi_encode();

        let res = api.send_limit_order(encoded.into()).await;
        assert!(res.is_ok());
        let received_order = handle.from_api.recv().await.expect("Should receive an order");
        assert_matches!(received_order, OrderCommand::NewOrder(OrderOrigin::External, AllOrders::Partial(_)));
        if let OrderCommand::NewOrder(OrderOrigin::External, AllOrders::Partial(order)) = received_order {
            assert_eq!(order, standing_order);
        };
    }

    #[tokio::test]
    async fn test_send_flash_order() {
        let (mut handle, api) = setup_order_api();

        let flash_order = FlashOrder {
            mode: "market".to_string(),
            max_amount_in_or_out: U256::from(500000),
            min_price: U256::from(50),
            asset_in: 3u16,
            asset_in_form: SolAssetForm::Liquid,
            asset_out: 4u16,
            asset_out_form: SolAssetForm::Liquid,
            recipient: Address::random(),
            hook_data: vec![9, 10, 11, 12].into(),
            valid_for_block: 1000,
            signature: vec![13, 14, 15, 16].into(),
        };

        let encoded = flash_order.abi_encode();
        let res = api.send_flash_order(encoded.into()).await;
        assert!(res.is_ok());
        let received_order = handle.from_api.recv().await.expect("Should receive an order");
        assert_matches!(received_order, OrderCommand::NewOrder(OrderOrigin::External, AllOrders::KillOrFill(_)));
        if let OrderCommand::NewOrder(OrderOrigin::External, AllOrders::KillOrFill(order)) = received_order {
            assert_eq!(order, flash_order);
        };
    }

    #[tokio::test]
    async fn test_send_top_of_block_order() {
        let (mut handle, api) = setup_order_api();

        let top_of_block_order = TopOfBlockOrder {
            amountIn: U256::from(100000),
            amountOut: U256::from(90000),
            assetInIndex: 5,
            assetInForm: SolAssetForm::Liquid,
            assetOutIndex: 6,
            assetOutForm: SolAssetForm::Liquid,
            recipient: Address::random(),
            hook: Address::random(),
            hookPayload: vec![17, 18, 19, 20].into(),
            from: Address::random(),
            signature: vec![1,2,3,4].into(),
        };
        let encoded = top_of_block_order.abi_encode();

        let res = api.send_searcher_order(encoded.into()).await;
        assert!(res.is_ok());
        let received_order = handle.from_api.recv().await.expect("Should receive an order");
        assert_matches!(received_order, OrderCommand::NewOrder(OrderOrigin::External, AllOrders::TOB(_)));
        if let OrderCommand::NewOrder(OrderOrigin::External, AllOrders::TOB(order)) = received_order{
            assert_eq!(order, top_of_block_order);
        };
    }


    fn setup_order_api() -> (OrderApiTestHandle, OrderApi<MockOrderPoolHandle, TokioTaskExecutor>) {
        let (to_pool, pool_rx) = unbounded_channel();
        let pool_handle = MockOrderPoolHandle { sender: to_pool };
        let task_executor = TokioTaskExecutor::default();
        let api = OrderApi::new(pool_handle.clone(), task_executor);
        let handle = OrderApiTestHandle { from_api: pool_rx };
        (handle, api)
    }

    struct OrderApiTestHandle {
        from_api: UnboundedReceiver<OrderCommand>,
    }

    #[derive(Clone)]
    struct MockOrderPoolHandle {
        sender: UnboundedSender<OrderCommand>,
    }

    impl OrderPoolHandle for MockOrderPoolHandle {
        fn new_order(&self, origin: OrderOrigin, order: AllOrders) -> bool {
            self.sender.send(OrderCommand::NewOrder(origin, order)).is_ok()
        }

        fn subscribe_orders(&self) -> Receiver<PoolManagerUpdate> {
            unimplemented!("Not needed for this test")
        }
    }
}

