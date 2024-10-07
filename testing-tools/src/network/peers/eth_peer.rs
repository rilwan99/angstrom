use reth_network::NetworkHandle;

pub struct EthPeer {
    eth_peer_handle:    PeerHandle<PeerPool>,
    eth_network_handle: NetworkHandle
}

impl EthPeer {
    //pub fn new(eth_network: Network)
}
