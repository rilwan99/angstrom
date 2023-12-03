use std::{collections::VecDeque, net::SocketAddr, pin::Pin, sync::Arc};

use fnv::FnvHashMap;
use futures::{
    future::Fuse,
    task::{Context, Poll},
    Stream
};
use reth_eth_wire::{capability::Capabilities, multiplex::ProtocolConnection, Status};
use reth_metrics::common::mpsc::MeteredPollSender;
use reth_network::{
    protocol::{ConnectionHandler, ProtocolHandler},
    SessionId
};
use reth_network_api::Direction;
use reth_primitives::{BytesMut, PeerId};
use tokio::{sync::mpsc, time::Instant};
use tokio_stream::wrappers::ReceiverStream;

use super::handle::SessionCommand;
use crate::StromSessionMessage;

/// An established session with a remote peer.
#[derive(Debug)]
pub struct ProtocolSessionHandle {
    /// The direction of the session
    pub(crate) direction:           Direction,
    /// The assigned id for this session
    pub(crate) session_id:          SessionId,
    /// The identifier of the remote peer
    pub(crate) remote_id:           PeerId,
    /// The timestamp when the session has been established.
    pub(crate) established:         Instant,
    /// Announced capabilities of the peer.
    pub(crate) capabilities:        Arc<Capabilities>,
    /// Sender half of the command channel used send commands _to_ the spawned
    /// session
    pub(crate) commands_to_session: mpsc::Sender<SessionCommand>,
    /// The client's name and version
    pub(crate) client_version:      Arc<str>,
    /// The address we're connected to
    pub(crate) remote_addr:         SocketAddr,
    /// The Status message the peer sent for the `eth` handshake
    pub(crate) status:              Arc<Status>
}


pub struct StromSession {
    pub(crate) conn:               ProtocolConnection,
    pub(crate) peer_id:            PeerId,
    pub(crate) commands_rx:        ReceiverStream<SessionCommand>,
    pub(crate) to_session_manager: MeteredPollSender<StromSessionMessage>,
    /// Incoming internal requests which are delegated to the remote peer.
    //pub(crate) internal_request_tx: Fuse<ReceiverStream<PeerRequest>>,
    /// All requests sent to the remote peer we're waiting on a response
    //pub(crate) inflight_requests:   FnvHashMap<u64, InflightRequest>,
    /// All requests that were sent by the remote peer and we're waiting on an
    /// internal response
    //pub(crate) received_requests_from_remote: Vec<ReceivedRequest>,
    /// Buffered messages that should be handled and sent to the peer.
   //pub(crate) queued_outgoing:    VecDeque<OutgoingMessage>
}




impl StromSession {
    pub fn new(
        conn: ProtocolConnection,
        to_session_manager: MeteredPollSender<StromSessionMessage>,
        peer_id: PeerId,
        commands_rx: ReceiverStream<SessionCommand>
    ) -> Self {
        Self {
            conn,
            peer_id,
            commands_rx,
            to_session_manager,
            //internal_request_tx: internal_request_rx.fuse(),
            //inflight_requests: FnvHashMap::default(),
            //received_requests_from_remote: Vec::new(),
            //queued_outgoing: VecDeque::new()
        }
    }

    pub fn on_command(command: SessionCommand) -> Poll<Option<SessionCommand>> {
        Poll::Ready(Some(command))
    }
}

impl Stream for StromSession {
    type Item = BytesMut;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

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
                    },
                    Poll::Pending => break, // No more commands, break the loop
                } 
                }
    
        }
    }
}
