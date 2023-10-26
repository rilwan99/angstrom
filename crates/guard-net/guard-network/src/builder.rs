//! Builder support for configuring the entire setup.

use reth_transaction_pool::TransactionPool;
use tokio::sync::mpsc;

use crate::{transactions::TransactionsManager, NetworkHandle, NetworkManager};

/// We set the max channel capacity of the EthRequestHandler to 256
/// 256 requests with malicious 10MB body requests is 2.6GB which can be
/// absorbed by the node.
pub(crate) const ETH_REQUEST_CHANNEL_CAPACITY: usize = 256;

/// A builder that can configure all components of the network.
#[allow(missing_debug_implementations)]
pub struct NetworkBuilder<Tx> {
    pub(crate) network:      NetworkManager,
    pub(crate) transactions: Tx
}

// === impl NetworkBuilder ===

impl<Tx> NetworkBuilder<Tx> {
    /// Consumes the type and returns all fields.
    pub fn split(self) -> (NetworkManager, Tx) {
        let NetworkBuilder { network, transactions } = self;
        (network, transactions)
    }

    /// Consumes the type and returns all fields and also return a
    /// [`NetworkHandle`].
    pub fn split_with_handle(self) -> (NetworkHandle, NetworkManager, Tx) {
        let NetworkBuilder { network, transactions } = self;
        let handle = network.handle().clone();
        (handle, network, transactions)
    }

    /// Creates a new [`TransactionsManager`] and wires it to the network.
    pub fn transactions<Pool: TransactionPool>(
        self,
        pool: Pool
    ) -> NetworkBuilder<TransactionsManager<Pool>> {
        let NetworkBuilder { mut network, .. } = self;
        let (tx, rx) = mpsc::unbounded_channel();
        network.set_transactions(tx);
        let handle = network.handle().clone();
        let transactions = TransactionsManager::new(handle, pool, rx);
        NetworkBuilder { network, transactions }
    }
}
