//! Connection tests

use std::collections::HashSet;

use ethers_core::rand;
use ethers_providers::Ws;
use futures::StreamExt;
use guard_discv4::Discv4Config;
use guard_eth_wire::{errors::EthStreamError, DEFAULT_HELLO_VERIFICATION_MESSAGE};
use guard_network::{NetworkConfigBuilder, Swarm, SwarmEvent};
use reth_primitives::mainnet_nodes;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tokio::sync::mpsc::unbounded_channel;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::net::testnet::{next_session_closed, next_session_established, PeerConfig, Testnet};

#[tokio::test(flavor = "multi_thread")]
async fn test_testnet_establish_connections() {
    reth_tracing::init_test_tracing();

    let (tx, mut rx) = unbounded_channel();
    let net = Testnet::create_with(3, tx).await;

    for peer in net.peers_iter() {
        assert_eq!(0, peer.num_peers());
    }

    let mut handles = net.handles();
    let handle0 = handles.next().unwrap();
    let handle1 = handles.next().unwrap();
    let handle2 = handles.next().unwrap();

    assert!(handles.next().is_none());

    drop(handles);

    let mut addresses = net.addresses();
    let addr0 = addresses.next().unwrap();
    let addr1 = addresses.next().unwrap();
    let addr2 = addresses.next().unwrap();

    assert!(addresses.next().is_none());
    drop(addresses);

    let mut peer_ids = net.peer_ids();
    let id0 = peer_ids.next().unwrap();
    let id1 = peer_ids.next().unwrap();
    let id2 = peer_ids.next().unwrap();

    assert!(peer_ids.next().is_none());
    drop(peer_ids);

    let handle = net.spawn();

    handle0.add_peer(id1, addr1);
    handle0.add_peer(id2, addr2);

    let mut expected_connections = HashSet::from([id1, id2]);
    let mut expected_peers = expected_connections.clone();

    //println!("{:#?}\n\n", expected_connections);

    assert!(!expected_connections.is_empty());
    assert!(!expected_peers.is_empty());

    // wait for all initiator connections
    while let Some(event) = rx.recv().await {
        if let (Some(ev), id) = event {
            if id != id0 {
                continue
            }
            match ev {
                SwarmEvent::ValidMessage { .. } => panic!("unexpected event"),
                SwarmEvent::InvalidCapabilityMessage { .. } => {
                    panic!("unexpected event")
                }
                SwarmEvent::BadMessage { .. } => panic!("unexpected event"),
                SwarmEvent::ProtocolBreach { .. } => panic!("unexpected event"),
                SwarmEvent::TcpListenerClosed { .. } => panic!("unexpected event"),
                SwarmEvent::TcpListenerError(_) => panic!("unexpected event"),
                SwarmEvent::IncomingTcpConnection { .. } => {
                    //println!("INCOMING CONNECTION: {:?}\n", remote_addr);
                    panic!("unexpected event")
                }
                SwarmEvent::OutgoingTcpConnection { peer_id, .. } => {
                    //println!("OUTGOING CONNECTION: {:?}\nTO: {:?}\n", remote_addr, peer_id);
                    assert!(expected_connections.contains(&peer_id));
                }
                SwarmEvent::SessionEstablished { peer_id, .. } => {
                    //println!("SESSION ESTABLISHED: {:#x}\n", peer_id);
                    assert!(expected_connections.remove(&peer_id));
                    if expected_connections.is_empty() {
                        break
                    }
                }
                SwarmEvent::SessionClosed { .. } => {
                    panic!("unexpected event")
                }
                SwarmEvent::PeerAdded(peer_id) => assert!(expected_peers.remove(&peer_id)),
                SwarmEvent::PeerRemoved(_) => panic!("unexpected event"),
                SwarmEvent::IncomingPendingSessionClosed { error, .. } => {
                    panic!("unexpected event: {:?}", error)
                }
                SwarmEvent::OutgoingPendingSessionClosed { error, .. } => {
                    panic!("unexpected event: {:?}", error)
                }
                SwarmEvent::OutgoingConnectionError { .. } => {
                    panic!("unexpected event")
                }
                SwarmEvent::AlreadyConnected { .. } => {
                    panic!("unexpected event")
                }
            }
        } else {
            break
        }
    }
    //println!("FINISHED MESSAGES");

    assert!(expected_connections.is_empty());
    assert!(expected_peers.is_empty());

    let net = handle.terminate().await;

    assert_eq!(net.peers()[0].num_peers(), 2);
    assert_eq!(net.peers()[1].num_peers(), 1);
    assert_eq!(net.peers()[2].num_peers(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_testnet_already_connected() {
    reth_tracing::init_test_tracing();

    let (tx, mut rx) = unbounded_channel();
    let mut net = Testnet::create_with(1, tx).await;

    let secret_key = SecretKey::new(&mut rand::thread_rng());

    // initialize two peers with the same identifier
    let p2 = PeerConfig::with_secret_key(secret_key).await;
    let p3 = PeerConfig::with_secret_key(secret_key).await;

    net.extend_peer_with_config(vec![p2, p3]).await.unwrap();

    let mut handles = net.handles();
    let handle0 = handles.next().unwrap();
    let handle1 = handles.next().unwrap();
    let handle2 = handles.next().unwrap();
    drop(handles);

    let mut addresses = net.addresses();
    let addr0 = addresses.next().unwrap();
    let addr1 = addresses.next().unwrap();
    drop(addresses);

    let mut peer_ids = net.peer_ids();
    let id0 = peer_ids.next().unwrap();
    let id1 = peer_ids.next().unwrap();
    let id2 = peer_ids.next().unwrap();
    drop(peer_ids);

    let handle = net.spawn();

    handle0.add_peer(id1, addr1);
    let peer_id = next_session_established(&mut rx, id0).await;
    assert!(peer_id.is_some());
    assert_eq!(peer_id.unwrap(), id1);

    handle2.add_peer(id0, addr0);
    let peer_id = next_session_closed(&mut rx, id0).await;
    assert!(peer_id.is_some());
    assert_eq!(peer_id.unwrap(), id2);

    let net = handle.terminate().await;

    assert_eq!(net.peers()[0].num_peers(), 1);
    assert_eq!(net.peers()[1].num_peers(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_testnet_get_peer() {
    reth_tracing::init_test_tracing();

    let (tx, mut rx) = unbounded_channel();
    let net = Testnet::create_with(3, tx).await;

    let mut handles = net.handles();
    let handle0 = handles.next().unwrap();
    drop(handles);

    let mut addresses = net.addresses();
    let addr0 = addresses.next().unwrap();
    let addr1 = addresses.next().unwrap();
    let addr2 = addresses.next().unwrap();

    assert!(addresses.next().is_none());
    drop(addresses);

    let mut peer_ids = net.peer_ids();
    let id0 = peer_ids.next().unwrap();
    let id1 = peer_ids.next().unwrap();
    let id2 = peer_ids.next().unwrap();

    assert!(peer_ids.next().is_none());
    drop(peer_ids);

    let handle = net.spawn();

    handle0.add_peer(id1, addr1);
    let _ = next_session_established(&mut rx, id0).await.unwrap();

    handle0.add_peer(id2, addr2);
    let _ = next_session_established(&mut rx, id0).await.unwrap();

    let peers = handle0.all_peers().await;

    let net = handle.terminate().await;
    assert_eq!(net.peers()[0].num_peers(), peers.len());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_testnet_get_peer_by_id() {
    reth_tracing::init_test_tracing();

    let (tx, mut rx) = unbounded_channel();
    let net = Testnet::create_with(3, tx).await;

    let mut handles = net.handles();
    let handle0 = handles.next().unwrap();
    drop(handles);

    let mut addresses = net.addresses();
    let addr0 = addresses.next().unwrap();
    let addr1 = addresses.next().unwrap();
    let addr2 = addresses.next().unwrap();

    assert!(addresses.next().is_none());
    drop(addresses);

    let mut peer_ids = net.peer_ids();
    let id0 = peer_ids.next().unwrap();
    let id1 = peer_ids.next().unwrap();
    let id2 = peer_ids.next().unwrap();

    assert!(peer_ids.next().is_none());
    drop(peer_ids);

    let _handle = net.spawn();

    handle0.add_peer(id1, addr1);
    let _ = next_session_established(&mut rx, id0).await.unwrap();

    let peer = handle0.peer_by_id(id1).await;
    assert!(peer.is_some());

    let peer = handle0.peer_by_id(id2).await;
    assert!(peer.is_none());
}

/*
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_connect_with_boot_nodes() {
    reth_tracing::init_test_tracing();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let secp = Secp256k1::new();
    let pub_key: [u8; 64] = PublicKey::from_secret_key(&secp, &secret_key).serialize_uncompressed()
        [1..]
        .try_into()
        .unwrap();
    let mut discv4 = Discv4Config::builder();
    discv4.add_boot_nodes(mainnet_nodes());

    let config =
        NetworkConfigBuilder::new(secret_key, DEFAULT_HELLO_VERIFICATION_MESSAGE, pub_key.into())
            .discovery(discv4)
            .build();

    let (_, rx) = unbounded_channel();
    let network = Swarm::new(config, rx).await.unwrap();

    let handle = network.handle().clone();
    let mut events = handle.event_listener();
    tokio::task::spawn(network);

    while let Some(ev) = events.next().await {
        dbg!(ev);
    }
}
*/

/*
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_connect_with_builder() {
    reth_tracing::init_test_tracing();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let mut discv4 = Discv4Config::builder();
    discv4.add_boot_nodes(mainnet_nodes());

    let client = NoopProvider::default();
    let config = NetworkConfigBuilder::new(secret_key)
        .discovery(discv4)
        .build(client);
    let (handle, network, _, requests) = NetworkManager::new(config)
        .await
        .unwrap()
        .into_builder()
        .request_handler(client)
        .split_with_handle();

    let mut events = handle.event_listener();

    tokio::task::spawn(async move {
        tokio::join!(network, requests);
    });

    let h = handle.clone();
    task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            dbg!(h.num_connected_peers());
        }
    });

    while let Some(ev) = events.next().await {
        dbg!(ev);
    }
}

// expects a `ENODE="enode://"` env var that holds the record
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_connect_to_trusted_peer() {
    reth_tracing::init_test_tracing();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let discv4 = Discv4Config::builder();

    let client = NoopProvider::default();
    let config = NetworkConfigBuilder::new(secret_key)
        .discovery(discv4)
        .build(client);
    let (handle, network, transactions, requests) = NetworkManager::new(config)
        .await
        .unwrap()
        .into_builder()
        .request_handler(client)
        .transactions(testing_pool())
        .split_with_handle();

    let mut events = handle.event_listener();

    tokio::task::spawn(async move {
        tokio::join!(network, requests, transactions);
    });

    let node: NodeRecord = std::env::var("ENODE").unwrap().parse().unwrap();

    handle.add_trusted_peer(node.id, node.tcp_addr());

    let h = handle.clone();
    h.update_sync_state(SyncState::Syncing);

    task::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            dbg!(h.num_connected_peers());
        }
    });

    let fetcher = handle.fetch_client().await.unwrap();

    let headers = fetcher
        .get_headers(HeadersRequest {
            start: 73174u64.into(),
            limit: 10,
            direction: HeadersDirection::Falling,
        })
        .await;

    dbg!(&headers);

    while let Some(ev) = events.next().await {
        dbg!(ev);
    }
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::serial]
#[ignore] // TODO: Re-enable once we figure out why this test is flakey
async fn test_incoming_node_id_blacklist() {
    reth_tracing::init_test_tracing();
    tokio::time::timeout(GETH_TIMEOUT, async move {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        // instantiate geth and add ourselves as a peer
        let temp_dir = tempfile::tempdir().unwrap().into_path();
        let geth = Geth::new()
            .data_dir(temp_dir)
            .disable_discovery()
            .authrpc_port(0)
            .spawn();
        let geth_endpoint = SocketAddr::new([127, 0, 0, 1].into(), geth.port());
        let provider = Provider::<Http>::try_from(format!("http://{geth_endpoint}")).unwrap();

        // get the peer id we should be expecting
        let geth_peer_id = enr_to_peer_id(provider.node_info().await.unwrap().enr);

        let ban_list = BanList::new(vec![geth_peer_id], HashSet::new());
        let peer_config = PeersConfig::default().with_ban_list(ban_list);

        let (reth_p2p, reth_disc) = unused_tcp_udp();
        let config = NetworkConfigBuilder::new(secret_key)
            .listener_addr(reth_p2p)
            .discovery_addr(reth_disc)
            .peer_config(peer_config)
            .build(NoopProvider::default());

        let network = NetworkManager::new(config).await.unwrap();

        let handle = network.handle().clone();
        let events = handle.event_listener();

        tokio::task::spawn(network);

        // make geth connect to us
        let our_enode = NodeRecord::new(reth_p2p, *handle.peer_id());

        provider.add_peer(our_enode.to_string()).await.unwrap();

        let mut event_stream = NetworkEventStream::new(events);

        // check for session to be opened
        let incoming_peer_id = event_stream.next_session_established().await.unwrap();
        assert_eq!(incoming_peer_id, geth_peer_id);

        // check to see that the session was closed
        let incoming_peer_id = event_stream.next_session_closed().await.unwrap().0;
        assert_eq!(incoming_peer_id, geth_peer_id);
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::serial]
// #[cfg_attr(not(feature = "geth-tests"), ignore)]
#[ignore] // TODO: Re-enable once we figure out why this test is flakey
async fn test_incoming_connect_with_single_geth() {
    reth_tracing::init_test_tracing();
    tokio::time::timeout(GETH_TIMEOUT, async move {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        // instantiate geth and add ourselves as a peer
        let temp_dir = tempfile::tempdir().unwrap().into_path();
        let geth = Geth::new()
            .data_dir(temp_dir)
            .disable_discovery()
            .authrpc_port(0)
            .spawn();
        let geth_endpoint = SocketAddr::new([127, 0, 0, 1].into(), geth.port());
        let provider = Provider::<Http>::try_from(format!("http://{geth_endpoint}")).unwrap();

        // get the peer id we should be expecting
        let geth_peer_id = enr_to_peer_id(provider.node_info().await.unwrap().enr);

        let (reth_p2p, reth_disc) = unused_tcp_udp();
        let config = NetworkConfigBuilder::new(secret_key)
            .listener_addr(reth_p2p)
            .discovery_addr(reth_disc)
            .build(NoopProvider::default());

        let network = NetworkManager::new(config).await.unwrap();

        let handle = network.handle().clone();
        tokio::task::spawn(network);

        let events = handle.event_listener();
        let mut event_stream = NetworkEventStream::new(events);

        // make geth connect to us
        let our_enode = NodeRecord::new(reth_p2p, *handle.peer_id());

        provider.add_peer(our_enode.to_string()).await.unwrap();

        // check for a sessionestablished event
        let incoming_peer_id = event_stream.next_session_established().await.unwrap();
        assert_eq!(incoming_peer_id, geth_peer_id);
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::serial]
#[cfg_attr(not(feature = "geth-tests"), ignore)]
#[ignore] // TODO: Re-enable once we figure out why this test is flakey
async fn test_outgoing_connect_with_single_geth() {
    reth_tracing::init_test_tracing();
    tokio::time::timeout(GETH_TIMEOUT, async move {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        let (reth_p2p, reth_disc) = unused_tcp_udp();
        let config = NetworkConfigBuilder::new(secret_key)
            .listener_addr(reth_p2p)
            .discovery_addr(reth_disc)
            .build(NoopProvider::default());
        let network = NetworkManager::new(config).await.unwrap();

        let handle = network.handle().clone();
        tokio::task::spawn(network);

        // create networkeventstream to get the next session established event easily
        let events = handle.event_listener();
        let mut event_stream = NetworkEventStream::new(events);

        // instantiate geth and add ourselves as a peer
        let temp_dir = tempfile::tempdir().unwrap().into_path();
        let geth = Geth::new()
            .disable_discovery()
            .data_dir(temp_dir)
            .authrpc_port(0)
            .spawn();

        let geth_p2p_port = geth.p2p_port().unwrap();
        let geth_socket = SocketAddr::new([127, 0, 0, 1].into(), geth_p2p_port);
        let geth_endpoint = SocketAddr::new([127, 0, 0, 1].into(), geth.port()).to_string();

        let provider = Provider::<Http>::try_from(format!("http://{geth_endpoint}")).unwrap();

        // get the peer id we should be expecting
        let geth_peer_id: PeerId = enr_to_peer_id(provider.node_info().await.unwrap().enr);

        // add geth as a peer then wait for a `SessionEstablished` event
        handle.add_peer(geth_peer_id, geth_socket);

        // check for a sessionestablished event
        let incoming_peer_id = event_stream.next_session_established().await.unwrap();
        assert_eq!(incoming_peer_id, geth_peer_id);
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::serial]
#[cfg_attr(not(feature = "geth-tests"), ignore)]
#[ignore] // TODO: Re-enable once we figure out why this test is flakey
async fn test_geth_disconnect() {
    reth_tracing::init_test_tracing();
    tokio::time::timeout(GETH_TIMEOUT, async move {
        let secret_key = SecretKey::new(&mut rand::thread_rng());

        let (reth_p2p, reth_disc) = unused_tcp_udp();
        let config = NetworkConfigBuilder::new(secret_key)
            .listener_addr(reth_p2p)
            .discovery_addr(reth_disc)
            .build(NoopProvider::default());
        let network = NetworkManager::new(config).await.unwrap();

        let handle = network.handle().clone();
        tokio::task::spawn(network);

        // create networkeventstream to get the next session established event easily
        let mut events = handle.event_listener();

        // instantiate geth and add ourselves as a peer
        let temp_dir = tempfile::tempdir().unwrap().into_path();
        let geth = Geth::new()
            .disable_discovery()
            .data_dir(temp_dir)
            .authrpc_port(0)
            .spawn();

        let geth_p2p_port = geth.p2p_port().unwrap();
        let geth_socket = SocketAddr::new([127, 0, 0, 1].into(), geth_p2p_port);
        let geth_endpoint = SocketAddr::new([127, 0, 0, 1].into(), geth.port()).to_string();

        let provider = Provider::<Http>::try_from(format!("http://{geth_endpoint}")).unwrap();

        // get the peer id we should be expecting
        let geth_peer_id: PeerId = enr_to_peer_id(provider.node_info().await.unwrap().enr);

        // add geth as a peer then wait for `PeerAdded` and `SessionEstablished` events.
        handle.add_peer(geth_peer_id, geth_socket);

        match events.next().await {
            Some(NetworkEvent::PeerAdded(peer_id)) => assert_eq!(peer_id, geth_peer_id),
            _ => panic!("Expected a peer added event"),
        }

        if let Some(NetworkEvent::SessionEstablished { peer_id, .. }) = events.next().await {
            assert_eq!(peer_id, geth_peer_id);
        } else {
            panic!("Expected a session established event");
        }

        // remove geth as a peer deliberately
        handle.disconnect_peer(geth_peer_id);

        // wait for a disconnect from geth
        if let Some(NetworkEvent::SessionClosed { peer_id, .. }) = events.next().await {
            assert_eq!(peer_id, geth_peer_id);
        } else {
            panic!("Expected a session closed event");
        }
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_shutdown() {
    reth_tracing::init_test_tracing();
    let net = Testnet::create(3).await;

    let mut handles = net.handles();
    let handle0 = handles.next().unwrap();
    let handle1 = handles.next().unwrap();
    let handle2 = handles.next().unwrap();

    drop(handles);
    let _handle = net.spawn();

    let mut listener0 = NetworkEventStream::new(handle0.event_listener());
    let mut listener1 = NetworkEventStream::new(handle1.event_listener());

    handle0.add_peer(*handle1.peer_id(), handle1.local_addr());
    handle0.add_peer(*handle2.peer_id(), handle2.local_addr());
    handle1.add_peer(*handle2.peer_id(), handle2.local_addr());

    let mut expected_connections = HashSet::from([*handle1.peer_id(), *handle2.peer_id()]);

    // Before shutting down, we have two connected peers
    let peer1 = listener0.next_session_established().await.unwrap();
    let peer2 = listener0.next_session_established().await.unwrap();
    assert_eq!(handle0.num_connected_peers(), 2);
    assert!(expected_connections.contains(&peer1));
    assert!(expected_connections.contains(&peer2));

    handle0.shutdown().await.unwrap();

    // All sessions get disconnected
    let (peer1, _reason) = listener0.next_session_closed().await.unwrap();
    let (peer2, _reason) = listener0.next_session_closed().await.unwrap();
    assert_eq!(handle0.num_connected_peers(), 0);
    assert!(expected_connections.remove(&peer1));
    assert!(expected_connections.remove(&peer2));

    // Connected peers receive a shutdown signal
    let (_peer, reason) = listener1.next_session_closed().await.unwrap();
    assert_eq!(reason, Some(DisconnectReason::ClientQuitting));

    // New connections ignored
    handle0.add_peer(*handle1.peer_id(), handle1.local_addr());
    assert_eq!(handle0.num_connected_peers(), 0);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_disconnect_incoming_when_exceeded_incoming_connections() {
    let net = Testnet::create(1).await;
    let (reth_p2p, reth_disc) = unused_tcp_udp();
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let peers_config = PeersConfig::default().with_max_inbound(0);

    let config = NetworkConfigBuilder::new(secret_key)
        .listener_addr(reth_p2p)
        .discovery_addr(reth_disc)
        .peer_config(peers_config)
        .build(NoopProvider::default());
    let network = NetworkManager::new(config).await.unwrap();

    let other_peer_handle = net.handles().next().unwrap();

    let handle = network.handle().clone();

    other_peer_handle.add_peer(*handle.peer_id(), handle.local_addr());

    tokio::task::spawn(network);
    let _handle = net.spawn();

    tokio::time::sleep(Duration::from_secs(1)).await;

    assert_eq!(handle.num_connected_peers(), 0);
}
*/
