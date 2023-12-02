use reth_eth_wire::DisconnectReason;

use crate::types::StromMessage;
/// Commands that can be sent to the spawned session.
//TODO: Create a subvariant of messages only for bidirectional messages received during an active
// session
#[derive(Debug)]
pub enum SessionCommand {
    /// Disconnect the connection
    Disconnect {
        /// Why the disconnect was initiated
        reason: Option<DisconnectReason>
    },
    /// Sends a message to the peer
    Message(StromMessage)
}
