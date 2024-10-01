use std::task::Poll;

use angstrom::cli::StromHandles;
use futures::FutureExt;
use reth_network_peers::PeerId;

use super::peers::StromPeer;

pub async fn connect_all_peers(self_peers: &mut [(PeerId, StromPeer, StromHandles)]) {
    let peer_set = self_peers.iter().collect::<Vec<_>>();
    for (pk, peer, _) in &*self_peers {
        for (other, ..) in &peer_set {
            if *pk == *other {
                continue
            }
            peer.add_validator(*other);
        }
    }
    // add all peers to each other
    let peers = self_peers.iter().collect::<Vec<_>>();
    for (idx, (_, handle, _)) in peers.iter().enumerate().take(peers.len() - 1) {
        for peer in peers.iter().skip(idx + 1) {
            let (id, neighbour, _) = peer;
            handle.connect_to_peer(*id, neighbour.socket_addr());
        }
    }

    // wait on each peer to add all other peers
    let needed_peers = self_peers.len() - 1;
    let mut peers = self_peers.iter_mut().map(|(_, p, _)| p).collect::<Vec<_>>();

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
