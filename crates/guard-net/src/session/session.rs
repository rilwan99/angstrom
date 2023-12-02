use reth_eth_wire::{multiplex::ProtocolConnection, ConnectionHandler, ProtocolHandler};

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

pub struct ProtocolSession {
    pub(crate) conn: ProtocolStream,
    pub(crate) peer_id: PeerId,
    pub(crate) commands_rx: ReceiverStream<SessionCommand>,
    pub(crate) to_session_manager: MeteredPollSender<ActiveSessionMessage>,
    /// Incoming internal requests which are delegated to the remote peer.
    pub(crate) internal_request_tx: Fuse<ReceiverStream<PeerRequest>>,
    /// All requests sent to the remote peer we're waiting on a response
    pub(crate) inflight_requests: FnvHashMap<u64, InflightRequest>,
    /// All requests that were sent by the remote peer and we're waiting on an
    /// internal response
    pub(crate) received_requests_from_remote: Vec<ReceivedRequest>,
    /// Buffered messages that should be handled and sent to the peer.
    pub(crate) queued_outgoing: VecDeque<OutgoingMessage>
}

impl ProtocolSession {
    pub fn new(
        conn: ProtocolStream,
        to_session_manager: MeteredPollSender<ActiveSessionMessage>
    ) -> Self {
        Self {
            conn,
            peer_id,
            commands_rx,
            to_session_manager,
            internal_request_tx: internal_request_rx.fuse(),
            inflight_requests: FnvHashMap::default(),
            received_requests_from_remote: Vec::new(),
            queued_outgoing: VecDeque::new()
        }
    }
}

impl Stream for ProtocolSession {
    type Output = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        'main: loop {
            let mut progress = false;

            // we prioritize incoming commands sent from the session manager
            loop {
                match this.commands_rx.poll_next_unpin(cx) {}
            }
        }
    }
}
