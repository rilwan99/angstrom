/// Where the transaction originates from.
///
/// Depending on where the transaction was picked up, it affects how the
/// transaction is handled internally, e.g. limits for simultaneous transaction
/// of one sender.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrderOrigin {
    /// Transaction is coming from a local source.
    Local,
    /// Transaction has been received externally.
    ///
    /// This is usually considered an "untrusted" source, for example received
    /// from another in the network.
    External,
    /// Transaction is originated locally and is intended to remain private.
    ///
    /// This type of transaction should not be propagated to the network. It's
    /// meant for private usage within the local node only.
    Private
}
