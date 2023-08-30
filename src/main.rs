use std::{path::PathBuf, str::FromStr, sync::Arc};

use clap::Parser;
use ethers_core::rand::rngs::ThreadRng;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Provider};
use ethers_reth::RethMiddleware;
use ethers_signers::LocalWallet;
use guard_network::{config::SecretKey, NetworkConfig, PeersConfig};
use hex_literal::hex;
use leader::leader_manager::LeaderConfig;
use reth_primitives::{mainnet_nodes, NodeRecord, PeerId, H512};
use sim::spawn_revm_sim;
use stale_guard::{Guard, SubmissionServerConfig};
use tokio::runtime::{Handle, Runtime};
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
    pub full_node:            PathBuf,
    #[arg(long)]
    pub full_node_ws:         String
}

impl Args {
    pub fn run(self, rt: Runtime) -> anyhow::Result<()> {
        reth_tracing::init(vec![]);
        //let fake_key = SecretKey::new(&mut rand::thread_rng());
        let fake_key =
            SecretKey::from_str("046cfcdbef4955744de5f87e739883e7ffa5daa05945bda2b7f5d4b3123935de")
                .unwrap();
        let fake_pub_key: Vec<u8>= hex!("04a3905ec9415c386d249b9bc9e430ce47c2f0e9dff67f749042dd2e58b24c3dda4e77f6c6c93d9b5d6377d63dd76c7e51e75057b7c3ff2b39f70027dcd50e80eb").to_vec();
        let fake_pub_key = fake_pub_key.as_slice();

        let fake_pub_key: &[u8; 64] =
            unsafe { &*(fake_pub_key as *const _ as *mut [u8]).cast() as &[u8; 64] };

        let fake_edsca = LocalWallet::new(&mut rand::thread_rng());
        let fake_bundle = LocalWallet::new(&mut rand::thread_rng());

        let inner = Provider::new(Http::new(self.full_node_ws.parse::<Url>()?));

        let middleware: &mut SignerMiddleware<Provider<Http>, LocalWallet> =
            Box::leak(Box::new(SignerMiddleware::new(
                inner,
                "046cfcdbef4955744de5f87e739883e7ffa5daa05945bda2b7f5d4b3123935de"
                    .parse()
                    .unwrap()
            )));

        let db_path = self.full_node.as_ref();
        let db = Arc::new(reth_db::mdbx::Env::<reth_db::mdbx::WriteMap>::open(
            db_path,
            reth_db::mdbx::EnvKind::RO,
            None
        )?);

        let sim = spawn_revm_sim(db, 6942069);

        //let fake_pub_key: PeerId = fake_pub_key.into();
        let network_config = NetworkConfig::new(fake_key, fake_pub_key.into());
        let leader_config = LeaderConfig {
            simulator: sim,
            edsca_key: fake_edsca,
            bundle_key: fake_bundle,
            middleware
        };

        let fake_addr = "127.0.0.1:6969".parse()?;

        let server_config = SubmissionServerConfig {
            addr:                fake_addr,
            // cors_domains:        "balls".into(),
            allow_subscriptions: self.enable_subscriptions
        };
        println!("spawning guard");

        let guard = rt.block_on(Guard::new(network_config, leader_config, server_config));
        rt.block_on(guard?);

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(3)
        .build()
        .unwrap();

    Args::parse().run(rt)?;

    Ok(())
}
