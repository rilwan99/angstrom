/// The type that tracks the reputation score.
pub type Reputation = i32;

/// Various kinds of stale guard specific reputation changes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReputationChangeKind {
    /// Received an unknown message from the peer
    BadMessage,
    /// Peer sent a bad order, i.e. an order who's signature isn't recoverable
    BadOrder,
    /// Peer failed to respond in time.
    Timeout,
    /// Failed to establish a connection to the peer.
    FailedToConnect,
    /// Connection dropped by peer.
    Dropped,
    /// Reset the reputation to the default value.
    Reset,
    /// Peer sent an invalid composable order, invalidity know at state n - 1
    InvalidComposableOrder
}

impl ReputationChangeKind {
    /// Returns true if the reputation change is a reset.
    pub fn is_reset(&self) -> bool {
        matches!(self, Self::Reset)
    }
}
