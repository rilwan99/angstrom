use std::{collections::HashMap, future::Future, task::Poll};

use futures::FutureExt;
use rand::Rng;
use reth_primitives::Address;
use reth_provider::{test_utils::NoopProvider, BlockReader, HeaderProvider};

use super::manager::{StromPeerManager, StromPeerManagerBuilder};
use crate::{contract_setup::deploy_contract_and_create_pool, eth::RpcStateProviderFactory};

pub struct StromController<C = NoopProvider> {
    peers: HashMap<u64, StromPeerManager<C>>
}

impl<C> StromController<C>
where
    C: BlockReader + HeaderProvider + Unpin + Clone + Default + 'static
{
    pub fn new() -> Self {
        Self { peers: Default::default() }
    }

    pub async fn spawn_node(
        &mut self,
        id: u64,
        starting_port: u16,
        testnet_block_time_secs: u64
    ) -> eyre::Result<()> {
        tracing::info!(id, "deploying contracts to anvil");

        let rpc_wrapper = RpcStateProviderFactory::spawn_new(testnet_block_time_secs, id).await?;

        let addresses = deploy_contract_and_create_pool(rpc_wrapper.provider.clone()).await?;
        let angstrom_addr = addresses.contract;

        let peer =
            StromPeerManagerBuilder::new(id, starting_port as u64, C::default(), rpc_wrapper).await;

        self.spawn_testnet_node(peer, angstrom_addr).await?;

        Ok(())
    }

    pub async fn connect_all_peers(&mut self) {
        let mut peer_set = self
            .peers
            .iter_mut()
            .map(|(_, peer)| peer)
            .collect::<Vec<_>>();

        for peer in &peer_set {
            for other_peer in &peer_set {
                if *peer.public_key == *other_peer.public_key {
                    continue
                }
                peer.peer.add_validator(other_peer.public_key);
            }
        }
        // add all peers to each other
        for (idx, peer) in peer_set.iter().enumerate().take(peer_set.len() - 1) {
            for other_peer in peer_set.iter().skip(idx + 1) {
                peer.peer
                    .connect_to_peer(other_peer.public_key, other_peer.peer.socket_addr());
            }
        }

        // wait on each peer to add all other peers
        let needed_peers = peer_set.len() - 1;
        let mut peers = peer_set.iter_mut().map(|p| &mut p.peer).collect::<Vec<_>>();

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

    pub async fn run_event<'a, F, O, R>(&'a self, id: Option<u64>, f: F) -> eyre::Result<R>
    where
        F: FnOnce(&'a StromPeerManager<C>) -> O,
        O: Future<Output = R>
    {
        let id = if let Some(i) = id {
            if i > self.peers.iter().map(|(id, _)| *id).max().unwrap() {
                self.get_random_id()
            } else {
                i
            }
        } else {
            self.get_random_id()
        };

        Ok(f(&self.peers.get(&id).unwrap()).await)
    }

    fn add_peer(&mut self, peer: StromPeerManager<C>) {
        self.peers.insert(peer.id, peer);
    }

    async fn spawn_testnet_node(
        &mut self,
        peer_builder: StromPeerManagerBuilder<C>,
        contract_address: Address
    ) -> eyre::Result<()> {
        let peer = peer_builder.build_and_spawn(contract_address).await?;

        self.add_peer(peer);

        Ok(())
    }

    fn get_random_id(&self) -> u64 {
        let max_id = self.peers.iter().map(|(id, _)| *id).max().unwrap();
        rand::thread_rng().gen_range(0..max_id)
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
