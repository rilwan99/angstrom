//! Represents an established session.

use core::sync::atomic::Ordering;
use std::{
    collections::VecDeque,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc},
    task::{ready, Context, Poll},
    time::{Duration, Instant}
};

use fnv::FnvHashMap;
use futures::{SinkExt, StreamExt};
use reth_ecies::stream::ECIESStream;
use reth_eth_wire::{
    capability::Capabilities,
    errors::{EthStreamError, P2PStreamError},
    message::EthBroadcastMessage,
    DisconnectReason, EthMessage, EthStream, P2PStream
};
use reth_metrics::common::mpsc::MeteredSender;
use reth_net_common::bandwidth_meter::MeteredStream;
use reth_primitives::PeerId;
use tokio::{net::TcpStream, sync::mpsc::error::TrySendError, time::Interval};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, info, trace};

use crate::session::{
    config::INITIAL_REQUEST_TIMEOUT,
    handle::{ActiveSessionMessage, SessionCommand},
    SessionId
};

/// Constants for timeout updating

/// Minimum timeout value
const MINIMUM_TIMEOUT: Duration = Duration::from_secs(2);
/// Maximum timeout value
const MAXIMUM_TIMEOUT: Duration = INITIAL_REQUEST_TIMEOUT;
/// How much the new measurements affect the current timeout (X percent)
const SAMPLE_IMPACT: f64 = 0.1;
/// Amount of RTTs before timeout
const TIMEOUT_SCALING: u32 = 3;

/// The type that advances an established session by listening for incoming
/// messages (from local node or read from connection) and emitting events back
/// to the [`SessionManager`](super::SessionManager).
///
/// It listens for
///    - incoming commands from the [`SessionManager`](super::SessionManager)
///    - incoming _internal_ requests/broadcasts via the request/command channel
///    - incoming requests/broadcasts _from remote_ via the connection
///    - responses for handled ETH requests received from the remote peer.
#[allow(unused)]
pub(crate) struct ActiveSession {
    /// Keeps track of request ids.
    pub(crate) next_id: u64,
    /// The underlying connection.
    pub(crate) conn: EthStream<P2PStream<ECIESStream<MeteredStream<TcpStream>>>>,
    /// Identifier of the node we're connected to.
    pub(crate) remote_peer_id: PeerId,
    /// The address we're connected to.
    pub(crate) remote_addr: SocketAddr,
    /// All capabilities the peer announced
    pub(crate) remote_capabilities: Arc<Capabilities>,
    /// Internal identifier of this session
    pub(crate) session_id: SessionId,
    /// Incoming commands from the manager
    pub(crate) commands_rx: ReceiverStream<SessionCommand>,
    /// Sink to send messages to the [`SessionManager`](super::SessionManager).
    pub(crate) to_session_manager: MeteredSender<ActiveSessionMessage>,
    /// A message that needs to be delivered to the session manager
    pub(crate) pending_message_to_session: Option<ActiveSessionMessage>,
    /// All requests sent to the remote peer we're waiting on a response
    pub(crate) inflight_requests: FnvHashMap<u64, InflightRequest>,
    /// All requests that were sent by the remote peer.
    pub(crate) received_requests_from_remote: Vec<ReceivedRequest>,
    /// Buffered messages that should be handled and sent to the peer.
    pub(crate) queued_outgoing: VecDeque<OutgoingMessage>,
    /// The maximum time we wait for a response from a peer.
    pub(crate) internal_request_timeout: Arc<AtomicU64>,
    /// Interval when to check for timed out requests.
    pub(crate) internal_request_timeout_interval: Interval,
    /// If an [ActiveSession] does not receive a response at all within this
    /// duration then it is considered a protocol violation and the session
    /// will initiate a drop.
    pub(crate) protocol_breach_request_timeout: Duration
}

impl ActiveSession {
    /// Returns `true` if the session is currently in the process of
    /// disconnecting
    fn is_disconnecting(&self) -> bool {
        self.conn.inner().is_disconnecting()
    }

    /// Returns the next request id
    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Shrinks the capacity of the internal buffers.
    pub fn shrink_to_fit(&mut self) {
        self.received_requests_from_remote.shrink_to_fit();
        self.queued_outgoing.shrink_to_fit();
    }

    /// Notify the manager that the peer sent a bad message
    fn on_bad_message(&self) {
        let _ = self
            .to_session_manager
            .try_send(ActiveSessionMessage::BadMessage { peer_id: self.remote_peer_id });
    }

    /// Report back that this session has been closed.
    fn emit_disconnect(&self) {
        trace!(target: "net::session", remote_peer_id=?self.remote_peer_id, "emitting disconnect");
        // NOTE: we clone here so there's enough capacity to deliver this message
        let _ = self
            .to_session_manager
            .clone()
            .try_send(ActiveSessionMessage::Disconnected {
                peer_id:     self.remote_peer_id,
                remote_addr: self.remote_addr
            });
    }

    /// Report back that this session has been closed due to an error
    fn close_on_error(&self, error: EthStreamError) {
        // NOTE: we clone here so there's enough capacity to deliver this message
        let _ = self.to_session_manager.clone().try_send(
            ActiveSessionMessage::ClosedOnConnectionError {
                peer_id: self.remote_peer_id,
                remote_addr: self.remote_addr,
                error
            }
        );
    }

    /// Starts the disconnect process
    fn start_disconnect(&mut self, reason: DisconnectReason) -> Result<(), EthStreamError> {
        self.conn
            .inner_mut()
            .start_disconnect(reason)
            .map_err(P2PStreamError::from)
            .map_err(Into::into)
    }

    /// Flushes the disconnect message and emits the corresponding message
    fn poll_disconnect(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        debug_assert!(self.is_disconnecting(), "not disconnecting");

        // try to close the flush out the remaining Disconnect message
        let _ = ready!(self.conn.poll_close_unpin(cx));
        self.emit_disconnect();
        Poll::Ready(())
    }

    /// Attempts to disconnect by sending the given disconnect reason
    fn try_disconnect(&mut self, reason: DisconnectReason, cx: &mut Context<'_>) -> Poll<()> {
        match self.start_disconnect(reason) {
            Ok(()) => {
                // we're done
                self.poll_disconnect(cx)
            }
            Err(err) => {
                debug!(target: "net::session", ?err, remote_peer_id=?self.remote_peer_id, "could not send disconnect");
                self.close_on_error(err);
                Poll::Ready(())
            }
        }
    }

    /// Updates the request timeout with a request's timestamps
    fn update_request_timeout(&mut self, sent: Instant, received: Instant) {
        let elapsed = received.saturating_duration_since(sent);

        let current = Duration::from_millis(self.internal_request_timeout.load(Ordering::Relaxed));
        let request_timeout = calculate_new_timeout(current, elapsed);
        self.internal_request_timeout
            .store(request_timeout.as_millis() as u64, Ordering::Relaxed);
        self.internal_request_timeout_interval = tokio::time::interval(request_timeout);
    }
}

impl Future for ActiveSession {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.is_disconnecting() {
            return this.poll_disconnect(cx)
        }

        // The receive loop can be CPU intensive since it involves message decoding
        // which could take up a lot of resources and increase latencies for
        // other sessions if not yielded manually. If the budget is exhausted we
        // manually yield back control to the (coop) scheduler. This manual yield point should prevent situations where polling appears to be frozen. See also <https://tokio.rs/blog/2020-04-preemption>
        // And tokio's docs on cooperative scheduling <https://docs.rs/tokio/latest/tokio/task/#cooperative-scheduling>
        let mut budget = 4;

        // The main poll loop that drives the session
        'main: loop {
            let mut progress = false;

            // we prioritize incoming commands sent from the session manager
            loop {
                match this.commands_rx.poll_next_unpin(cx) {
                    Poll::Pending => break,
                    Poll::Ready(None) => {
                        // this is only possible when the manager was dropped, in which case we also
                        // terminate this session
                        return Poll::Ready(())
                    }
                    Poll::Ready(Some(cmd)) => {
                        progress = true;
                        match cmd {
                            SessionCommand::Disconnect { reason } => {
                                info!(target: "net::session", ?reason, remote_peer_id=?this.remote_peer_id, "Received disconnect command for session");
                                let reason =
                                    reason.unwrap_or(DisconnectReason::DisconnectRequested);

                                return this.try_disconnect(reason, cx)
                            }
                        }
                    }
                }
            }

            // Send messages by advancing the sink and queuing in buffered messages
            while this.conn.poll_ready_unpin(cx).is_ready() {
                if let Some(msg) = this.queued_outgoing.pop_front() {
                    progress = true;
                    let res = match msg {
                        OutgoingMessage::Eth(msg) => this.conn.start_send_unpin(msg),
                        OutgoingMessage::Broadcast(msg) => this.conn.start_send_broadcast(msg)
                    };
                    if let Err(err) = res {
                        debug!(target: "net::session", ?err,  remote_peer_id=?this.remote_peer_id, "failed to send message");
                        // notify the manager
                        this.close_on_error(err);
                        return Poll::Ready(())
                    }
                } else {
                    // no more messages to send over the wire
                    break
                }
            }

            // read incoming messages from the wire
            'receive: loop {
                // ensure we still have enough budget for another iteration
                budget -= 1;
                if budget == 0 {
                    // make sure we're woken up again
                    cx.waker().wake_by_ref();
                    break 'main
                }

                // try to resend the pending message that we could not send because the channel
                // was full.
                if let Some(msg) = this.pending_message_to_session.take() {
                    match this.to_session_manager.try_send(msg) {
                        Ok(_) => {}
                        Err(err) => {
                            match err {
                                TrySendError::Full(msg) => {
                                    this.pending_message_to_session = Some(msg);
                                    // ensure we're woken up again
                                    cx.waker().wake_by_ref();
                                    break 'receive
                                }
                                TrySendError::Closed(_) => {}
                            }
                        }
                    }
                }
            }

            if !progress {
                break 'main
            }
        }

        this.shrink_to_fit();

        Poll::Pending
    }
}

/// Tracks a request received from the peer
pub(crate) struct ReceivedRequest {
    /// Protocol Identifier
    request_id: u64,
    /// Timestamp when we read this msg from the wire.
    #[allow(unused)]
    received:   Instant
}

/// A request that waits for a response from the peer
pub(crate) struct InflightRequest {
    /// Request we sent to peer and the internal response channel
    request:   RequestState,
    /// Instant when the request was sent
    timestamp: Instant,
    /// Time limit for the response
    deadline:  Instant
}

// === impl InflightRequest ===

impl InflightRequest {
    /// Returns true if the request is timedout
    #[inline]
    fn is_timed_out(&self, now: Instant) -> bool {
        now > self.deadline
    }
}

/// All outcome variants when handling an incoming message
enum OnIncomingMessageOutcome {
    /// Message successfully handled.
    Ok,
    /// Message is considered to be in violation fo the protocol
    BadMessage { error: EthStreamError, message: EthMessage },
    /// Currently no capacity to handle the message
    NoCapacity(ActiveSessionMessage)
}

impl From<Result<(), ActiveSessionMessage>> for OnIncomingMessageOutcome {
    fn from(res: Result<(), ActiveSessionMessage>) -> Self {
        match res {
            Ok(_) => OnIncomingMessageOutcome::Ok,
            Err(msg) => OnIncomingMessageOutcome::NoCapacity(msg)
        }
    }
}

enum RequestState {
    /// Request already timed out
    TimedOut
}

/// Outgoing messages that can be sent over the wire.
pub(crate) enum OutgoingMessage {
    /// A message that is owned.
    Eth(EthMessage),
    /// A message that may be shared by multiple sessions.
    Broadcast(EthBroadcastMessage)
}

impl From<EthMessage> for OutgoingMessage {
    fn from(value: EthMessage) -> Self {
        OutgoingMessage::Eth(value)
    }
}

impl From<EthBroadcastMessage> for OutgoingMessage {
    fn from(value: EthBroadcastMessage) -> Self {
        OutgoingMessage::Broadcast(value)
    }
}

/// Calculates a new timeout using an updated estimation of the RTT
#[inline]
fn calculate_new_timeout(current_timeout: Duration, estimated_rtt: Duration) -> Duration {
    let new_timeout = estimated_rtt.mul_f64(SAMPLE_IMPACT) * TIMEOUT_SCALING;

    // this dampens sudden changes by taking a weighted mean of the old and new
    // values
    let smoothened_timeout = current_timeout.mul_f64(1.0 - SAMPLE_IMPACT) + new_timeout;

    smoothened_timeout.clamp(MINIMUM_TIMEOUT, MAXIMUM_TIMEOUT)
}
