use reth_primitives::PeerId;

pub enum StromNetworkEvent {
    /// event emitted when a new strom peer is added
    PeerAdded(PeerId),
    /// event emitted when a new strom peer is removed
    PeerRemoved(PeerId)
}
