use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use ethers_core::rand::rngs::ThreadRng;
use ethers_providers::{Http, Provider};
use ethers_reth::RethMiddleware;
use ethers_signers::LocalWallet;
use guard_network::{config::SecretKey, NetworkConfig, PeersConfig};
use leader::leader_manager::LeaderConfig;
use reth_primitives::{mainnet_nodes, NodeRecord, H512};
use sim::spawn_revm_sim;
use stale_guard::{Guard, SubmissionServerConfig};
use tokio::runtime::Handle;
use url::Url;

#[derive(Debug, Parser)]
pub struct Args {
    // #[arg(long, value_name = "FILE")]
    // pub staking_secret_key:   PathBuf,
    // #[arg(long, value_name = "FILE")]
    // pub bundle_key:           PathBuf,
    // #[arg(long, value_name = "FILE")]
    // pub submission_key:       PathBuf,
    // #[arg(long, value_delimiter = ',')]
    // pub bootnodes:            Option<Vec<NodeRecord>>,
    // #[arg(long, value_delimiter = ',')]
    // pub builder_submissions:  Option<Vec<Url>>,
    // #[arg(long)]
    // pub bundle_sim_url:       Option<Url>,
    #[arg(long, default_value = "false")]
    pub enable_subscriptions: bool,
    #[arg(long)]
    pub full_node: PathBuf,
    #[arg(long)]
    pub full_node_ws: Url,
}

impl Args {
    pub async fn run(self) -> anyhow::Result<()> {
        //let fake_key = SecretKey::new(&mut rand::thread_rng());
        let fake_key =
            SecretKey::from_str("046cfcdbef4955744de5f87e739883e7ffa5daa05945bda2b7f5d4b3123935de")
                .unwrap();
        let fake_pub_key = "04a3905ec9415c386d249b9bc9e430ce47c2f0e9dff67f749042dd2e58b24c3dda4e77f6c6c93d9b5d6377d63dd76c7e51e75057b7c3ff2b39f70027dcd50e80eb";

        let fake_edsca = LocalWallet::new(&mut rand::thread_rng());
        let fake_bundle = LocalWallet::new(&mut rand::thread_rng());

        let inner = Provider::new(Http::new(self.full_node_ws));

        let middleware = Box::leak(Box::new(
            RethMiddleware::new(inner, self.full_node, tokio::runtime::Handle::current(), 1)
                .unwrap(),
        ));
        let (sim, handle) = spawn_revm_sim(middleware, 6942069);

        let network_config = NetworkConfig::new(fake_key, fake_pub_key.parse().unwrap());
        let leader_config = LeaderConfig {
            simulator: sim,
            edsca_key: fake_edsca,
            bundle_key: fake_bundle,
            middleware,
        };

        let fake_addr = "ws://127.0.0.1:6969".parse()?;
        let server_config = SubmissionServerConfig {
            addr: fake_addr,
            cors_domains: "balls".into(),
            allow_subscriptions: self.enable_subscriptions,
        };

        let guard = Guard::new(network_config, leader_config, server_config, handle).await?;
        tokio::spawn(guard).await?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    Handle::current().spawn_blocking(|| Args::parse().run());
    Ok(())
}
