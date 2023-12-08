use std::{ops::Deref, pin::Pin};

use alloy_rlp::Encodable;
use futures::{
    task::{Context, Poll},
    Stream, StreamExt
};
use reth_eth_wire::multiplex::ProtocolConnection;
use reth_metrics::common::mpsc::MeteredPollSender;
use reth_network_api::Direction;
use reth_primitives::{BytesMut, PeerId};
use secp256k1::SecretKey;
use tokio::time::Duration;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::PollSender;

use super::handle::SessionCommand;
use crate::{types::message::StromProtocolMessage, StromSessionMessage};

#[allow(dead_code)]
pub struct StromSession {
    /// The underlying connection.
    pub(crate) conn:               ProtocolConnection,
    /// Identifier of the node we're connected to.
    pub(crate) remote_peer_id:     PeerId,
    /// Incoming commands from the manager
    pub(crate) commands_rx:        ReceiverStream<SessionCommand>,
    /// Sink to send messages to the [`SessionManager`](super::SessionManager).
    pub(crate) to_session_manager: MeteredPollSender<StromSessionMessage>,

    /// If an [ActiveSession] does not receive a response at all within this
    /// duration then it is considered a protocol violation and the session
    /// will initiate a drop.
    pub(crate) protocol_breach_request_timeout: Duration,
    /// Used to reserve a slot to guarantee that the termination message is
    /// delivered
    pub(crate) terminate_message: Option<(PollSender<StromSessionMessage>, StromSessionMessage)>,
    pub is_verified: bool,
    pub signing_key: SecretKey
}

impl StromSession {
    pub fn new(
        conn: ProtocolConnection,
        peer_id: PeerId,
        commands_rx: ReceiverStream<SessionCommand>,
        to_session_manager: MeteredPollSender<StromSessionMessage>,
        protocol_breach_request_timeout: Duration,
        signing_key: SecretKey
    ) -> Self {
        Self {
            signing_key,
            is_verified: false,
            conn,
            remote_peer_id: peer_id,
            commands_rx,
            to_session_manager,
            protocol_breach_request_timeout,
            terminate_message: None
        }
    }

    /// Report back that this session has been closed.
    fn emit_disconnect(&mut self, cx: &mut Context<'_>) -> Poll<Option<BytesMut>> {
        let msg = StromSessionMessage::Disconnected { peer_id: self.remote_peer_id };

        self.terminate_message = Some((self.to_session_manager.inner().clone(), msg));
        self.poll_terminate_message(cx).expect("message is set")
    }

    /// If a termination message is queued, this will try to send it to the
    /// manager
    fn poll_terminate_message(&mut self, cx: &mut Context<'_>) -> Option<Poll<Option<BytesMut>>> {
        let (mut tx, msg) = self.terminate_message.take()?;
        match tx.poll_reserve(cx) {
            Poll::Pending => {
                self.terminate_message = Some((tx, msg));
                return Some(Poll::Pending)
            }
            Poll::Ready(Ok(())) => {
                let _ = tx.send_item(msg);
            }
            Poll::Ready(Err(_)) => {
                // channel closed
            }
        }
        // terminate the task
        Some(Poll::Ready(None))
    }

    fn poll_verification(&mut self, cx: &mut Context<'_>) {
        todo!()
    }

    fn poll_commands(&mut self, cx: &mut Context<'_>) -> Option<Poll<Option<BytesMut>>> {
        if let Poll::Ready(inner) = self.commands_rx.poll_next_unpin(cx).map(|inner| {
            inner.map_or_else(
                || Poll::Ready(None),
                |msg| match msg {
                    SessionCommand::Disconnect { .. } => self.emit_disconnect(cx),
                    SessionCommand::Message(msg) => {
                        let msg = StromProtocolMessage {
                            message_type: msg.message_id(),
                            message:      msg
                        };
                        let mut bytes = BytesMut::with_capacity(msg.length());
                        msg.encode(&mut bytes);
                        Poll::Ready(Some(bytes))
                    }
                }
            )
        }) {
            Some(inner)
        } else {
            None
        }
    }

    fn poll_incoming(&mut self, cx: &mut Context<'_>) -> Option<Poll<Option<BytesMut>>> {
        // processes incoming messages until there are none left or the stream closes
        while let Poll::Ready(msg) = self.conn.poll_next_unpin(cx).map(|data| {
            data.map(|bytes| {
                let msg = StromProtocolMessage::decode_message(&mut bytes.deref());
                let _ = self.to_session_manager.send_item(
                    msg.map(|m| StromSessionMessage::ValidMessage {
                        peer_id: self.remote_peer_id,
                        message: m
                    })
                    .unwrap_or(StromSessionMessage::BadMessage { peer_id: self.remote_peer_id })
                );

                Poll::<()>::Pending
            })
            .ok_or_else(|| self.emit_disconnect(cx))
        }) {
            if let Err(e) = msg {
                return Some(e)
            }
        }
        None
    }
}

//TODO: Implement poll functionality with: on_command, on_timeout, on_message
// from wire..

impl Stream for StromSession {
    type Item = BytesMut;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if !this.is_verified {
            this.poll_verification(cx);
        }

        // if the session is terminate we have to send the termination message before we
        // can close
        if let Some(terminate) = this.poll_terminate_message(cx) {
            return terminate
        }

        // progress manager commands
        if let Some(msg) = this.poll_commands(cx) {
            return msg
        }

        // processes messages from the wire
        if let Some(msg) = this.poll_incoming(cx) {
            return msg
        }

        Poll::Pending
    }
}
