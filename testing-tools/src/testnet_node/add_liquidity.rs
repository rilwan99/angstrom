use alloy_primitives::{Address, I256, U160, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_sol_types::SolCall;
use angstrom_hook_bindings::testnet::TestnetHub;
use clap::Parser;
use futures::future::try_join_all;
use serde::Deserialize;

type Tick = i32;
type SqrtPriceX96 = U160;

#[derive(Debug, Deserialize)]
struct PoolRange {
    lower_tick: Tick,
    upper_tick: Tick,
    liquidity:  u128
}

#[derive(Debug, Deserialize)]
struct LiquidityRanges {
    ranges:       Vec<PoolRange>,
    #[allow(unused)]
    tick_spacing: Tick,
    sqrt_price:   SqrtPriceX96
}

const TEST_ACCOUNT0: &'static str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

#[derive(Parser)]
struct AddLiquidityArgs {
    #[arg(help_heading = "Path of liquidity data .json", index = 1)]
    file_path: String,
    #[arg(long, default_value = "0x1696C7203769A71c97Ca725d42b13270ee493526")]
    asset0:    Address,
    #[arg(long, default_value = "0x332Fb35767182F8ac9F9C1405db626105F6694E0")]
    asset1:    Address,
    #[arg(
        short,
        long,
        default_value =TEST_ACCOUNT0,
        help = "Address of test account (must be RPC unlocked)"
    )]
    signer:    Address,
    #[arg(long, default_value = "0x76ca03a67C049477FfB09694dFeF00416dB69746")]
    hub_addr:  Address,
    #[arg(short, long, default_value = "http://127.0.0.1:8545")]
    rpc_url:   String
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = AddLiquidityArgs::parse();

    let data = std::fs::read_to_string(&args.file_path)
        .unwrap_or_else(|_| panic!("No file \"{}\" found", args.file_path));

    let data: LiquidityRanges = serde_json::from_str(&data)?;

    let rpc_url = args.rpc_url.parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    let hub = TestnetHub::new(args.hub_addr, &provider);

    let TestnetHub::isInitializedReturn { _0: is_initialized } =
        hub.isInitialized(args.asset0, args.asset1).call().await?;

    if is_initialized {
        println!("WARNING: Pool already initialized");
        return Ok(());
    }

    // TODO: Allow non-RPC unlocked signer.
    let init_tx = hub
        .initializePool(args.asset0, args.asset1, U256::from(data.sqrt_price))
        .from(args.signer)
        .send()
        .await?
        .watch()
        .await?;
    println!("Initialized pool (tx: {})", init_tx);

    println!("Adding liquidity:");
    // TODO: Join together
    let mut txs = Vec::with_capacity(data.ranges.len());
    for range in data.ranges.iter() {
        let tx = *hub
            .execute(
                TestnetHub::modifyLiquidityCall {
                    asset0:         args.asset0,
                    asset1:         args.asset1,
                    tickLower:      range.lower_tick,
                    tickUpper:      range.upper_tick,
                    liquidityDelta: I256::try_from(range.liquidity).unwrap()
                }
                .abi_encode()
                .into()
            )
            .from(args.signer)
            .send()
            .await?
            .tx_hash();
        println!("    tx: {}", tx);
        txs.push(tx);
    }

    let receipts = try_join_all(
        txs.iter()
            .map(|tx| provider.get_transaction_receipt(tx.clone()))
    )
    .await?;

    let missing_receipts: Vec<_> = receipts
        .iter()
        .zip(txs.iter())
        .filter_map(|(r, tx)| match r {
            None => Some(tx),
            Some(_) => None
        })
        .collect();
    if !missing_receipts.is_empty() {
        println!("missing_receipts: {:?}", missing_receipts);
    }
    let failed_txs: Vec<_> = receipts
        .iter()
        .zip(txs.iter())
        .filter_map(|(r, tx)| {
            r.as_ref()
                .and_then(|receipt| if receipt.inner.is_success() { None } else { Some(tx) })
        })
        .collect();

    if !failed_txs.is_empty() {
        println!("failed txs:");
        failed_txs.iter().for_each(|tx| println!("    - {}", tx));
    } else {
        println!("Successfully added");
    }

    Ok(())
}
