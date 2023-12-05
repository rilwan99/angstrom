/// Tracks info about a single peer.
#[derive(Debug, Clone)]
pub struct Peer {
    /// Where to reach the peer
    addr:                    SocketAddr,
    /// Reputation of the peer.
    reputation:              i32,
    /// The state of the connection, if any.
    state:                   PeerConnectionState,
    /// The [`ForkId`] that the peer announced via discovery.
    fork_id:                 Option<ForkId>,
    /// Whether the entry should be removed after an existing session was
    /// terminated.
    remove_after_disconnect: bool,
    /// The kind of peer
    kind:                    PeerKind,
    /// Whether the peer is currently backed off.
    backed_off:              bool,
    /// Counts number of times the peer was backed off due to a severe
    /// [BackoffKind].
    severe_backoff_counter:  u32
}
