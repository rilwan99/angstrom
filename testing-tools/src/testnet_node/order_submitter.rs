use alloy_primitives::{Address, I256, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_sol_types::SolCall;
use angstrom_hook_bindings::testnet::TestnetHub;
use angstrom_rpc::api::OrderApiClient;
use clap::Parser;
use jsonrpsee::http_client::HttpClientBuilder;
use serde::Deserialize;

mod addresses;
use addresses::*;

#[derive(Debug, Deserialize)]
struct Order {
    is_bid:    bool,
    quantity:  u128,
    price_ray: U256
}

#[derive(Parser)]
struct AddLiquidityArgs {
    #[arg(help_heading = "Path of order .json", index = 1)]
    file_path:       String,
    #[arg(long, default_value = DEFAULT_ASSET0)]
    asset0:          Address,
    #[arg(long, default_value = DEFAULT_ASSET1)]
    asset1:          Address,
    #[arg(long, default_value = DEFAULT_ANGSTROM)]
    angstrom:        Address,
    #[arg(short, long, default_value = "http://127.0.0.1:4200")]
    testnet_rpc_url: String,
    #[arg(short, long, default_value = "http://127.0.0.1:8545")]
    eth_rpc_url:     String
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = AddLiquidityArgs::parse();

    let data = std::fs::read_to_string(&args.file_path)
        .unwrap_or_else(|_| panic!("No file \"{}\" found", args.file_path));

    let orders: Vec<Order> = serde_json::from_str(&data)?;

    let rpc_url = args.eth_rpc_url.parse()?;

    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    let angstrom_client = HttpClientBuilder::default().build(args.testnet_rpc_url)?;
    angstrom_client
        .submit_limit_order(Default::default())
        .await?;

    Ok(())
}
