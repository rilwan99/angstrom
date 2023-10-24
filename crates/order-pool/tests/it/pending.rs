use assert_matches::assert_matches;
use order_pool::{
    test_utils::{testing_pool, MockTransactionFactory},
    OrderOrigin, TransactionPool
};

#[tokio::test(flavor = "multi_thread")]
async fn txpool_new_pending_txs() {
    let txpool = testing_pool();
    let mut mock_tx_factory = MockTransactionFactory::default();
    let transaction = mock_tx_factory.create_eip1559();

    let added_result = txpool
        .add_transaction(OrderOrigin::External, transaction.transaction.clone())
        .await;
    assert_matches!(added_result, Ok(hash) if hash == transaction.transaction.get_hash());

    let mut best_txns = txpool.best_transactions();
    assert_matches!(best_txns.next(), Some(tx) if tx.transaction.get_hash() == transaction.transaction.get_hash());
    assert_matches!(best_txns.next(), None);
    let transaction = mock_tx_factory.create_eip1559();
    let added_result = txpool
        .add_transaction(OrderOrigin::External, transaction.transaction.clone())
        .await;
    assert_matches!(added_result, Ok(hash) if hash == transaction.transaction.get_hash());
    assert_matches!(best_txns.next(), Some(tx) if tx.transaction.get_hash() == transaction.transaction.get_hash());
}