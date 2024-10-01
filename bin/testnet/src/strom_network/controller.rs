use std::{collections::HashMap, marker::PhantomData, task::Poll};

use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};

use super::manager::StromPeerManager;
use crate::eth::RpcStateProviderFactory;

pub struct StromController<C = NoopProvider> {
    peers: HashMap<u64, StromPeerManager<C>>
}

impl<C> StromController<C>
where
    // P: PeerManager<C>,
    C: BlockReader + HeaderProvider + Unpin + Clone + 'static
{
    pub fn new() -> Self {
        Self { peers: Default::default() }
    }

    pub fn add_peer(&mut self, peer: StromPeerManager<C>) {
        self.peers.insert(peer.id(), peer);
    }

    pub async fn connect_all_peers(&mut self) {
        let mut peer_set = self
            .peers
            .iter_mut()
            .map(|(_, peer)| peer)
            .collect::<Vec<_>>();

        for peer in &peer_set {
            for other_peer in &peer_set {
                if *peer.public_key() == *other_peer.public_key() {
                    continue
                }
                peer.peer().add_validator(other_peer.public_key());
            }
        }
        // add all peers to each other
        for (idx, peer) in peer_set.iter().enumerate().take(peer_set.len() - 1) {
            for other_peer in peer_set.iter().skip(idx + 1) {
                peer.peer()
                    .connect_to_peer(other_peer.public_key(), other_peer.peer().socket_addr());
            }
        }

        // wait on each peer to add all other peers
        let needed_peers = peer_set.len() - 1;
        let mut peers = peer_set
            .iter_mut()
            .map(|p| p.peer_mut())
            .collect::<Vec<_>>();

        std::future::poll_fn(|cx| {
            let mut all_connected = true;
            for peer in &mut peers {
                if peer.poll_unpin(cx).is_ready() {
                    tracing::error!("peer failed");
                }
                all_connected &= peer.get_peer_count() == needed_peers
            }

            if all_connected {
                return Poll::Ready(())
            }

            Poll::Pending
        })
        .await
    }

    pub async fn spawn_all_testnet_nodes(&mut self, contract_address: Address) -> eyre::Result<()> {
        futures::future::join_all(
            self.peers
                .iter_mut()
                .map(|(_, peer)| peer.spawn_testnet_node(contract_address))
        )
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}

/*


    pub async fn spawn_all_testnet_nodes(
        &mut self,
        rpc_wrapper: RpcStateProviderFactory,
        contract_address: Address
    ) -> eyre::Result<()> {
        let handles = self
            .peers
            .iter_mut()
            .map(|(_, peer)| peer)
            .collect::<Vec<_>>();

        futures::future::join_all(handles.into_iter().map(|(&peer, tx_handles, rx_handles)| {
            peer.spawn_testnet_node(tx_handles, rpc_wrapper.clone(), contract_address, rx_handles)
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        Ok(())
    }
}



*/
