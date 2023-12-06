use std::{
    pin::Pin,
    task::{Context, Poll}
};

use futures::Stream;
use reth_primitives::PeerId;

use crate::{peers::PeersManager, session::StromSessionManager, PeerKind};
#[derive(Debug)]
#[must_use = "Swarm does nothing unless polled"]
pub(crate) struct Swarm {
    /// All sessions.
    sessions:      StromSessionManager,
    peers_manager: PeersManager
}

impl Swarm {
    /// Creates a new `Swarm`.
    pub fn new(sessions: StromSessionManager, peers_manager: PeersManager) -> Self {
        Swarm { sessions, peers_manager }
    }

    /// Access to the [`SessionManager`].
    pub(crate) fn sessions(&self) -> &StromSessionManager {
        &self.sessions
    }

    /// Mutable access to the [`SessionManager`].
    pub(crate) fn sessions_mut(&mut self) -> &mut StromSessionManager {
        &mut self.sessions
    }

    pub(crate) fn state(&self) -> &PeersManager {
        &self.peers_manager
    }

    pub(crate) fn state_mut(&mut self) -> &mut PeersManager {
        &mut self.peers_manager
    }

    pub(crate) fn remove_peer(&mut self, peer_id: PeerId, kind: PeerKind) {
        match kind {
            PeerKind::Basic => self.peers_manager.remove_peer(peer_id),
            PeerKind::MevGuard => self.peers_manager.remove_peer_from_trusted_set(peer_id),
            _ => todo!()
        }
    }
}

impl Stream for Swarm {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Poll the session manager

        Poll::Pending
    }
}
