use ethers_middleware::Middleware;
use ethers_providers::PubsubClient;
use guard_network::{
    test_harness::NetworkTestClient, GaurdStakingEvent, NetworkConfig, SwarmEvent
};
use leader::leader_manager::LeaderConfig;
use sim::Simulator;
use stale_guard::{test_harness::GuardHandle, SubmissionServerConfig};
use tokio::sync::mpsc::{channel, unbounded_channel, Receiver, UnboundedSender};

/// basic test harness that gives us control into what the guard sends
/// as well as a network client that allows us to send and receive messages
/// from the guard network itself
pub struct BasicHarness {
    pub guard_handle:   GuardHandle,
    pub network_client: NetworkTestClient,
    pub receive:        Receiver<SwarmEvent>,
    pub sender:         UnboundedSender<GaurdStakingEvent>
}

impl BasicHarness {
    pub async fn new<M, S>(
        guard_network_config: NetworkConfig,
        client_network_config: NetworkConfig,
        leader_config: LeaderConfig<M, S>,
        server_config: SubmissionServerConfig
    ) -> anyhow::Result<Self>
    where
        M: Middleware + Unpin + 'static,
        S: Simulator + Unpin + 'static,
        <M as Middleware>::Provider: PubsubClient
    {
        let (tx, rx) = channel(10);
        let (stake_tx, stake_rx) = unbounded_channel();

        let network_client = NetworkTestClient::new(client_network_config, stake_rx, tx).await?;
        let guard_network =
            GuardHandle::new(guard_network_config, leader_config, server_config).await?;

        Ok(Self { guard_handle: guard_network, network_client, receive: rx, sender: stake_tx })
    }
}
