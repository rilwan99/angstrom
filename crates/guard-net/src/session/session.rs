use std::pin::Pin;

use futures::{
    task::{Context, Poll},
    Stream
};
use reth_eth_wire::multiplex::ProtocolConnection;
use reth_primitives::{BytesMut, PeerId};
use tokio::{sync::mpsc, time::Duration};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::PollSender;

use super::handle::SessionCommand;
use crate::StromSessionMessage;

#[allow(dead_code)]
pub struct StromSession {
    /// Keeps track of request ids.
    pub(crate) next_id: u64,
    /// The underlying connection.
    pub(crate) conn: ProtocolConnection,
    /// Identifier of the node we're connected to.
    pub(crate) remote_peer_id: PeerId,
    /// Incoming commands from the manager
    pub(crate) commands_rx: ReceiverStream<SessionCommand>,
    /// Sink to send messages to the [`SessionManager`](super::SessionManager).
    pub(crate) to_session_manager: mpsc::Sender<StromSessionMessage>,
    /// A message that needs to be delivered to the session manager
    pub(crate) pending_message_to_session: Option<StromSessionMessage>,

    /// All requests sent to the remote peer we're waiting on a response
    //pub(crate) inflight_requests: FnvHashMap<u64, InflightRequest>,
    /// All requests that were sent by the remote peer and we're waiting on an
    /// internal response
    //pub(crate) received_requests_from_remote: Vec<ReceivedRequest>,
    /// Buffered messages that should be handled and sent to the peer.
    //pub(crate) queued_outgoing: VecDeque<OutgoingMessage>,
    /// The maximum time we wait for a response from a peer.

    /// If an [ActiveSession] does not receive a response at all within this
    /// duration then it is considered a protocol violation and the session
    /// will initiate a drop.
    pub(crate) protocol_breach_request_timeout: Duration,
    /// Used to reserve a slot to guarantee that the termination message is
    /// delivered
    pub(crate) terminate_message: Option<(PollSender<StromSessionMessage>, StromSessionMessage)>
}

impl StromSession {
    pub fn new(
        conn: ProtocolConnection,
        peer_id: PeerId,
        commands_rx: ReceiverStream<SessionCommand>,
        to_session_manager: mpsc::Sender<StromSessionMessage>,
        protocol_breach_request_timeout: Duration
    ) -> Self {
        Self {
            next_id: 0,
            conn,
            remote_peer_id: peer_id,
            commands_rx,
            to_session_manager,
            pending_message_to_session: None,
            protocol_breach_request_timeout,
            terminate_message: None
        }
    }

    pub fn on_command(command: SessionCommand) -> Poll<Option<SessionCommand>> {
        Poll::Ready(Some(command))
    }
}

//TODO: Implement poll functionality with: on_command, on_timeout, on_message
// from wire..

impl Stream for StromSession {
    type Item = BytesMut;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()

        /*let this = self.get_mut();

        'main: loop {
            let mut progress = false;

            // we prioritize incoming commands sent from the session manager
            loop {
                match this.commands_rx.poll_next_unpin(cx) {
                    Poll::Ready(Some(command)) => match command {
                        SessionCommand::Send(data) => {
                            // Implement logic to handle the send command
                            this.conn.send(data);
                        }
                        SessionCommand::Disconnect => {
                            // Implement logic to handle the disconnect command
                            this.conn.close();
                        }
                    },
                    Poll::Ready(None) => {
                        // Implement logic for when the stream of commands is finished
                        return Poll::Ready(None);
                    }
                    Poll::Pending => break // No more commands, break the loop
                }
            }
        }*/
    }
}
