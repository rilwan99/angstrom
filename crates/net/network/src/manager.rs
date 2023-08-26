//! High level network management.
//!
//! The [`NetworkManager`] contains the state of the network as a whole. It controls how connections
//! are handled and keeps track of connections to peers.
//!
//! ## Capabilities
//!
//! The network manages peers depending on their announced capabilities via their RLPx sessions. Most importantly the [Ethereum Wire Protocol](https://github.com/ethereum/devp2p/blob/master/caps/eth.md)(`eth`).
//!
//! ## Overview
//!
//! The [`NetworkManager`] is responsible for advancing the state of the `network`. The `network` is
//! made up of peer-to-peer connections between nodes that are available on the same network.
//! Responsible for peer discovery is ethereum's discovery protocol (discv4, discv5). If the address
//! (IP+port) of our node is published via discovery, remote peers can initiate inbound connections
//! to the local node. Once a (tcp) connection is established, both peers start to authenticate a [RLPx session](https://github.com/ethereum/devp2p/blob/master/rlpx.md) via a handshake. If the handshake was successful, both peers announce their capabilities and are now ready to exchange sub-protocol messages via the RLPx session.

use crate::{
    config::NetworkConfig,
    discovery::Discovery,
    error::{NetworkError, ServiceKind},
    listener::ConnectionListener,
    message::{NewBlockMessage, PeerMessage, PeerRequest, PeerRequestSender},
    metrics::{DisconnectMetrics, NetworkMetrics, NETWORK_POOL_TRANSACTIONS_SCOPE},
    network::{NetworkHandle, NetworkHandleMessage},
    peers::{PeersHandle, PeersManager},
    session::SessionManager,
    state::NetworkState,
    swarm::{NetworkConnectionState, Swarm, SwarmEvent},
    transactions::NetworkTransactionEvent,
};
use futures::{Future, StreamExt};
use parking_lot::Mutex;
use reth_eth_wire::{
    capability::{Capabilities, CapabilityMessage},
    DisconnectReason, EthVersion, Status,
};
use reth_metrics::common::mpsc::UnboundedMeteredSender;
use reth_net_common::bandwidth_meter::BandwidthMeter;
use reth_network_api::ReputationChangeKind;
use reth_primitives::{listener::EventListeners, ForkId, NodeRecord, PeerId, H256};
use reth_provider::{BlockNumReader, BlockReader};
use reth_rpc_types::{EthProtocolInfo, NetworkStatus};
use std::{
    net::SocketAddr,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
};
use tokio::sync::mpsc::{self, error::TrySendError};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error, info, trace, warn};

/// Manages the _entire_ state of the network.
///
/// This is an endless [`Future`] that consistently drives the state of the entire network forward.
///
/// The [`NetworkManager`] is the container type for all parts involved with advancing the network.
#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
///  graph TB
///    handle(NetworkHandle)
///    events(NetworkEvents)
///    transactions(Transactions Task)
///    ethrequest(ETH Request Task)
///    discovery(Discovery Task)
///    subgraph NetworkManager
///      direction LR
///      subgraph Swarm
///          direction TB
///          B1[(Session Manager)]
///          B2[(Connection Lister)]
///          B3[(Network State)]
///      end
///   end
///   handle <--> |request response channel| NetworkManager
///   NetworkManager --> |Network events| events
///   transactions <--> |transactions| NetworkManager
///   ethrequest <--> |ETH request handing| NetworkManager
///   discovery --> |Discovered peers| NetworkManager
/// ```
#[must_use = "The NetworkManager does nothing unless polled"]
pub struct NetworkManager<C> {}




impl<C> Future for NetworkManager<C>
where
    C: BlockReader + Unpin,
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // process incoming messages from a handle
        loop {
            match this.from_handle_rx.poll_next_unpin(cx) {
                Poll::Pending => break,
                Poll::Ready(None) => {
                    // This is only possible if the channel was deliberately closed since we always
                    // have an instance of `NetworkHandle`
                    error!("Network message channel closed.");
                    return Poll::Ready(())
                }
                Poll::Ready(Some(msg)) => this.on_handle_message(msg),
            };
        }

        // This loop drives the entire state of network and does a lot of work.
        // Under heavy load (many messages/events), data may arrive faster than it can be processed
        // (incoming messages/requests -> events), and it is possible that more data has already
        // arrived by the time an internal event is processed. Which could turn this loop into a
        // busy loop.  Without yielding back to the executor, it can starve other tasks waiting on
        // that executor to execute them, or drive underlying resources To prevent this, we
        // preemptively return control when the `budget` is exhausted. The value itself is
        // chosen somewhat arbitrarily, it is high enough so the swarm can make meaningful progress
        // but low enough that this loop does not starve other tasks for too long.
        // If the budget is exhausted we manually yield back control to the (coop) scheduler. This
        // manual yield point should prevent situations where polling appears to be frozen. See also <https://tokio.rs/blog/2020-04-preemption>
        // And tokio's docs on cooperative scheduling <https://docs.rs/tokio/latest/tokio/task/#cooperative-scheduling>
        let mut budget = 1024;

        loop {
            // advance the swarm
            match this.swarm.poll_next_unpin(cx) {
                Poll::Pending | Poll::Ready(None) => break,
                Poll::Ready(Some(event)) => {
                    // handle event
                    match event {
                        SwarmEvent::ValidMessage { peer_id, message } => {
                            this.on_peer_message(peer_id, message)
                        }
                        SwarmEvent::InvalidCapabilityMessage { peer_id, capabilities, message } => {
                            this.on_invalid_message(peer_id, capabilities, message);
                            this.metrics.invalid_messages_received.increment(1);
                        }
                        SwarmEvent::TcpListenerClosed { remote_addr } => {
                            trace!(target : "net", ?remote_addr, "TCP listener closed.");
                        }
                        SwarmEvent::TcpListenerError(err) => {
                            trace!(target : "net", ?err, "TCP connection error.");
                        }
                        SwarmEvent::IncomingTcpConnection { remote_addr, session_id } => {
                            trace!(target : "net", ?session_id, ?remote_addr, "Incoming connection");
                            this.metrics.total_incoming_connections.increment(1);
                            this.metrics
                                .incoming_connections
                                .set(this.swarm.state().peers().num_inbound_connections() as f64);
                        }
                        SwarmEvent::OutgoingTcpConnection { remote_addr, peer_id } => {
                            trace!(target : "net", ?remote_addr, ?peer_id, "Starting outbound connection.");
                            this.metrics.total_outgoing_connections.increment(1);
                            this.metrics
                                .outgoing_connections
                                .set(this.swarm.state().peers().num_outbound_connections() as f64);
                        }
                        SwarmEvent::SessionEstablished {
                            peer_id,
                            remote_addr,
                            client_version,
                            capabilities,
                            version,
                            messages,
                            status,
                            direction,
                        } => {
                            let total_active =
                                this.num_active_peers.fetch_add(1, Ordering::Relaxed) + 1;
                            this.metrics.connected_peers.set(total_active as f64);
                            info!(
                                target : "net",
                                ?remote_addr,
                                %client_version,
                                ?peer_id,
                                ?total_active,
                                "Session established"
                            );
                            debug!(target: "net", kind=%direction, peer_enode=%NodeRecord::new(remote_addr, peer_id), "Established peer enode");

                            if direction.is_incoming() {
                                this.swarm
                                    .state_mut()
                                    .peers_mut()
                                    .on_incoming_session_established(peer_id, remote_addr);
                            }
                            this.event_listeners.notify(NetworkEvent::SessionEstablished {
                                peer_id,
                                remote_addr,
                                client_version,
                                capabilities,
                                version,
                                status,
                                messages,
                            });
                        }
                        SwarmEvent::PeerAdded(peer_id) => {
                            trace!(target: "net", ?peer_id, "Peer added");
                            this.event_listeners.notify(NetworkEvent::PeerAdded(peer_id));
                            this.metrics
                                .tracked_peers
                                .set(this.swarm.state().peers().num_known_peers() as f64);
                        }
                        SwarmEvent::PeerRemoved(peer_id) => {
                            trace!(target: "net", ?peer_id, "Peer dropped");
                            this.event_listeners.notify(NetworkEvent::PeerRemoved(peer_id));
                            this.metrics
                                .tracked_peers
                                .set(this.swarm.state().peers().num_known_peers() as f64);
                        }
                        SwarmEvent::SessionClosed { peer_id, remote_addr, error } => {
                            let total_active =
                                this.num_active_peers.fetch_sub(1, Ordering::Relaxed) - 1;
                            this.metrics.connected_peers.set(total_active as f64);
                            trace!(
                                target : "net",
                                ?remote_addr,
                                ?peer_id,
                                ?total_active,
                                ?error,
                                "Session disconnected"
                            );

                            let mut reason = None;
                            if let Some(ref err) = error {
                                // If the connection was closed due to an error, we report the peer
                                this.swarm.state_mut().peers_mut().on_active_session_dropped(
                                    &remote_addr,
                                    &peer_id,
                                    err,
                                );
                                reason = err.as_disconnected();
                            } else {
                                // Gracefully disconnected
                                this.swarm
                                    .state_mut()
                                    .peers_mut()
                                    .on_active_session_gracefully_closed(peer_id);
                            }
                            this.metrics.closed_sessions.increment(1);
                            // This can either be an incoming or outgoing connection which was
                            // closed. So we update both metrics
                            this.metrics
                                .incoming_connections
                                .set(this.swarm.state().peers().num_inbound_connections() as f64);
                            this.metrics
                                .outgoing_connections
                                .set(this.swarm.state().peers().num_outbound_connections() as f64);
                            if let Some(reason) = reason {
                                this.disconnect_metrics.increment(reason);
                            }
                            this.metrics.backed_off_peers.set(
                                this.swarm.state().peers().num_backed_off_peers().saturating_sub(1)
                                    as f64,
                            );
                            this.event_listeners
                                .notify(NetworkEvent::SessionClosed { peer_id, reason });
                        }
                        SwarmEvent::IncomingPendingSessionClosed { remote_addr, error } => {
                            debug!(
                                target : "net",
                                ?remote_addr,
                                ?error,
                                "Incoming pending session failed"
                            );

                            if let Some(ref err) = error {
                                this.swarm
                                    .state_mut()
                                    .peers_mut()
                                    .on_incoming_pending_session_dropped(remote_addr, err);
                                this.metrics.pending_session_failures.increment(1);
                                if let Some(reason) = err.as_disconnected() {
                                    this.disconnect_metrics.increment(reason);
                                }
                            } else {
                                this.swarm
                                    .state_mut()
                                    .peers_mut()
                                    .on_incoming_pending_session_gracefully_closed();
                            }
                            this.metrics.closed_sessions.increment(1);
                            this.metrics
                                .incoming_connections
                                .set(this.swarm.state().peers().num_inbound_connections() as f64);
                            this.metrics.backed_off_peers.set(
                                this.swarm.state().peers().num_backed_off_peers().saturating_sub(1)
                                    as f64,
                            );
                        }
                        SwarmEvent::OutgoingPendingSessionClosed {
                            remote_addr,
                            peer_id,
                            error,
                        } => {
                            trace!(
                                target : "net",
                                ?remote_addr,
                                ?peer_id,
                                ?error,
                                "Outgoing pending session failed"
                            );

                            if let Some(ref err) = error {
                                this.swarm.state_mut().peers_mut().on_pending_session_dropped(
                                    &remote_addr,
                                    &peer_id,
                                    err,
                                );
                                this.metrics.pending_session_failures.increment(1);
                                if let Some(reason) = err.as_disconnected() {
                                    this.disconnect_metrics.increment(reason);
                                }
                            } else {
                                this.swarm
                                    .state_mut()
                                    .peers_mut()
                                    .on_pending_session_gracefully_closed(&peer_id);
                            }
                            this.metrics.closed_sessions.increment(1);
                            this.metrics
                                .outgoing_connections
                                .set(this.swarm.state().peers().num_outbound_connections() as f64);
                            this.metrics.backed_off_peers.set(
                                this.swarm.state().peers().num_backed_off_peers().saturating_sub(1)
                                    as f64,
                            );
                        }
                        SwarmEvent::OutgoingConnectionError { remote_addr, peer_id, error } => {
                            trace!(
                                target : "net",
                                ?remote_addr,
                                ?peer_id,
                                ?error,
                                "Outgoing connection error"
                            );

                            this.swarm.state_mut().peers_mut().on_outgoing_connection_failure(
                                &remote_addr,
                                &peer_id,
                                &error,
                            );

                            this.metrics
                                .outgoing_connections
                                .set(this.swarm.state().peers().num_outbound_connections() as f64);
                            this.metrics.backed_off_peers.set(
                                this.swarm.state().peers().num_backed_off_peers().saturating_sub(1)
                                    as f64,
                            );
                        }
                        SwarmEvent::BadMessage { peer_id } => {
                            this.swarm.state_mut().peers_mut().apply_reputation_change(
                                &peer_id,
                                ReputationChangeKind::BadMessage,
                            );
                            this.metrics.invalid_messages_received.increment(1);
                        }
                        SwarmEvent::ProtocolBreach { peer_id } => {
                            this.swarm.state_mut().peers_mut().apply_reputation_change(
                                &peer_id,
                                ReputationChangeKind::BadProtocol,
                            );
                        }
                    }
                }
            }

            // ensure we still have enough budget for another iteration
            budget -= 1;
            if budget == 0 {
                // make sure we're woken up again
                cx.waker().wake_by_ref();
                break
            }
        }

        Poll::Pending
    }
}


