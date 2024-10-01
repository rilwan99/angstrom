use std::sync::Arc;

use alloy::{primitives::FixedBytes, providers::Provider};
use angstrom::cli::{initialize_strom_handles, StromHandles};
use angstrom_eth::handle::Eth;
use angstrom_network::{pool_manager::OrderCommand, NetworkOrderEvent, PoolManagerBuilder};
use angstrom_rpc::{api::OrderApiServer, OrderApi};
use futures::StreamExt;
use jsonrpsee::server::ServerBuilder;
use order_pool::{order_storage::OrderStorage, PoolConfig};
use reth_metrics::common::mpsc::UnboundedMeteredReceiver;
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader};
use reth_tasks::TokioTaskExecutor;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{span, Instrument, Level};
use validation::init_validation;

use super::handles::{ReceivingStromHandles, SendingStromHandles};
use crate::{
    anvil_utils::AnvilEthDataCleanser, eth::RpcStateProviderFactory,
    strom_network::peers::StromPeer, CACHE_VALIDATION_SIZE
};

pub struct StromPeerManager<C = NoopProvider> {
    id:               u64,
    port:             u64,
    public_key:       FixedBytes<64>,
    peer:             StromPeer<C>,
    rpc_wrapper:      RpcStateProviderFactory,
    tx_strom_handles: SendingStromHandles,
    init_rx_handles:  Option<ReceivingStromHandles>
}

impl<C> StromPeerManager<C>
where
    C: BlockReader + Clone + Unpin + 'static
{
    pub async fn new(
        id: u64,
        port: u64,
        provider: C,
        rpc_wrapper: RpcStateProviderFactory
    ) -> Self {
        let span = span!(Level::TRACE, "testnet node", id = id);
        let handles = initialize_strom_handles();
        let peer = StromPeer::new_fully_configed(
            provider,
            Some(handles.pool_tx.clone()),
            Some(handles.consensus_tx_op.clone())
        )
        .instrument(span)
        .await;
        let pk = peer.get_node_public_key();

        let tx_handles = (&handles).into();
        let rx_handles = handles.into();

        Self {
            id,
            peer,
            public_key: pk,
            rpc_wrapper,
            tx_strom_handles: tx_handles,
            init_rx_handles: Some(rx_handles),
            port: port + id - 1
        }
    }

    pub fn peer_mut(&mut self) -> &mut StromPeer<C> {
        &mut self.peer
    }

    pub fn peer(&self) -> &StromPeer<C> {
        &self.peer
    }

    pub fn public_key(&self) -> FixedBytes<64> {
        self.public_key
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn take_rx_handles(&mut self) -> ReceivingStromHandles {
        self.init_rx_handles
            .take()
            .expect("rx handles already taken")
    }

    pub fn tx_handles(&mut self) -> SendingStromHandles {
        self.tx_strom_handles.clone()
    }

    pub async fn spawn_testnet_node(&mut self, contract_address: Address) -> eyre::Result<()> {
        let span = span!(Level::ERROR, "testnet node", id = self.id);
        let pool = self.tx_strom_handles.get_pool_handle();
        let executor: TokioTaskExecutor = Default::default();

        let (tx_handles, rx_handles) = (self.tx_handles(), self.take_rx_handles());

        let rpc_w = self.rpc_wrapper.clone();
        let state_stream = self
            .rpc_wrapper
            .provider
            .clone()
            .subscribe_blocks()
            .await?
            .into_stream()
            .map(move |block| {
                let cloned_block = block.clone();
                let rpc = rpc_w.clone();
                async move {
                    let number = cloned_block.header.number;
                    let mut res = vec![];
                    for hash in cloned_block.transactions.hashes() {
                        let Ok(Some(tx)) = rpc.provider.get_transaction_by_hash(hash).await else {
                            continue
                        };
                        res.push(tx);
                    }
                    (number, res)
                }
            })
            .buffer_unordered(10);

        let order_api = OrderApi::new(pool.clone(), executor.clone());

        let eth_handle = AnvilEthDataCleanser::spawn(
            executor.clone(),
            contract_address,
            tx_handles.eth_tx,
            rx_handles.eth_rx,
            state_stream,
            7,
            span
        )
        .await?;

        let validator = init_validation(self.rpc_wrapper.clone(), CACHE_VALIDATION_SIZE);

        let network_handle = self.peer.handle.clone();

        let pool_config = PoolConfig::default();
        let order_storage = Arc::new(OrderStorage::new(&pool_config));

        let _pool_handle = PoolManagerBuilder::new(
            validator.clone(),
            Some(order_storage.clone()),
            network_handle.clone(),
            eth_handle.subscribe_network(),
            rx_handles.pool_rx
        )
        .with_config(pool_config)
        .build_with_channels(
            executor,
            tx_handles.orderpool_tx,
            rx_handles.orderpool_rx,
            tx_handles.pool_manager_tx
        );
        let port = self.port;
        let server = ServerBuilder::default()
            .build(format!("127.0.0.1:{}", port))
            .await?;

        let addr = server.local_addr().unwrap();
        tracing::info!("rpc server started on: {}", addr);

        tokio::spawn(async move {
            let server_handle = server.start(order_api.into_rpc());
            let _ = server_handle.stopped().await;
        });

        Ok(())
    }
}

// pub trait PeerManager<C> {
//     fn peer_mut(&mut self) -> &mut StromPeer<C>;

//     fn peer(&self) -> &StromPeer<C>;

//     fn public_key(&self) -> FixedBytes<64>;

//     fn id(&self) -> u64;

//     fn take_rx_handles(&mut self) -> ReceivingStromHandles;

//     fn spawn_testnet_node(
//         &self,
//         strom_handles: SendingStromHandles,
//         rpc_wrapper: RpcStateProviderFactory,
//         contract_address: Address,
//         rx_handles: ReceivingStromHandles
//     ) -> impl futures::Future<Output = eyre::Result<()>>;
// }
