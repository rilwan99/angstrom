use std::collections::HashMap;

use alloy_primitives::{Address, Bytes, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_wallet::LocalWallet;
use alloy_sol_types::{eip712_domain, SolStruct};
use angstrom_hook_bindings::{
    sol::SolStandingOrder,
    testnet::MockERC20,
    user_types::{AssetForm, OrderMode, StandingOrder}
};
use clap::Parser;
use rand::{seq::SliceRandom, thread_rng};
use serde::Deserialize;
use validation::order::state::upkeepers::nonces::Nonces;

mod addresses;
use addresses::*;

#[derive(Debug, Deserialize, Clone)]
struct Order {
    is_bid:    bool,
    quantity:  u128,
    price_ray: U256
}

impl Order {
    fn get_amount_in(&self) -> U256 {
        let qty = U256::from(self.quantity);
        if self.is_bid {
            qty * RAY / self.price_ray
        } else {
            qty
        }
    }

    fn get_min_price(&self) -> U256 {
        if self.is_bid {
            self.price_ray
        } else {
            RAY * RAY / self.price_ray
        }
    }

    fn get_asset_in(&self, asset0: Address, asset1: Address) -> Address {
        if self.is_bid {
            asset0
        } else {
            asset1
        }
    }

    fn get_asset_out(&self, asset0: Address, asset1: Address) -> Address {
        if self.is_bid {
            asset1
        } else {
            asset0
        }
    }
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

const RAY: U256 = U256::from_limbs([11515845246265065472, 54210108, 0, 0]);

async fn ensure_tokens<T: alloy_transport::Transport + Clone, P: Provider<T>>(
    account: Address,
    angstrom: Address,
    erc20: &MockERC20::MockERC20Instance<T, P>,
    amount: U256
) -> eyre::Result<()> {
    let MockERC20::allowanceReturn { allowed } = erc20.allowance(account, angstrom).call().await?;
    if allowed < amount {
        let _tx_hash = erc20
            .approve(angstrom, U256::MAX)
            .from(account)
            .send()
            .await?
            .watch()
            .await?;
    }

    let MockERC20::balanceOfReturn { result: balance } = erc20.balanceOf(account).call().await?;

    if balance < amount {
        let _tx_hash = erc20
            .mint(account, balance)
            .from(account)
            .send()
            .await?
            .watch()
            .await?;
    }

    Ok(())
}

#[derive(Debug)]
struct NonceGetter<'a, T: alloy_transport::Transport + Clone, P: Provider<T>> {
    account:    Address,
    angstrom:   Address,
    provider:   &'a P,
    fetched:    Option<(U256, u64)>,
    _transport: core::marker::PhantomData<T>
}

impl<'a, T: alloy_transport::Transport + Clone, P: Provider<T>> NonceGetter<'a, T, P> {
    fn new(account: Address, angstrom: Address, provider: &'a P) -> NonceGetter<'a, T, P> {
        Self { account, angstrom, provider, fetched: None, _transport: Default::default() }
    }

    async fn next(&mut self) -> eyre::Result<u64> {
        let (mut bitmap, mut nonce, mut bit) = match self.fetched {
            Some((bitmap, last_nonce)) => (bitmap, last_nonce & !0xff, (last_nonce & 0xff) + 1),
            None => {
                let slot = Nonces.get_nonce_word_slot(self.account, 0);
                let bitmap = self
                    .provider
                    .get_storage_at(self.angstrom, slot.into(), None)
                    .await?;
                (bitmap, 0, 0)
            }
        };

        while (bitmap >> bit) == (U256::MAX >> bit) {
            nonce += 256;
            bit = 0;
            let slot = Nonces.get_nonce_word_slot(self.account, nonce);
            bitmap = self
                .provider
                .get_storage_at(self.angstrom, slot.into(), None)
                .await?;
        }
        nonce |= bit;

        nonce += (bitmap >> bit).trailing_ones() as u64;
        self.fetched = Some((bitmap, nonce));
        return Ok(nonce);
    }
}

fn address_to_wallet(addr: &Address) -> eyre::Result<LocalWallet> {
    let (_, priv_key) = TEST_WALLETS
        .iter()
        .find(|(test_addr, _)| test_addr == addr)
        .unwrap_or_else(|| panic!("Unknown test wallet {}", addr));
    Ok(priv_key.to_string().parse()?)
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let args = AddLiquidityArgs::parse();

    let data = std::fs::read_to_string(&args.file_path)
        .unwrap_or_else(|_| panic!("No file \"{}\" found", args.file_path));

    let orders: Vec<Order> = serde_json::from_str(&data)?;

    let rpc_url = args.eth_rpc_url.parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    let assets = {
        let mut assets = HashMap::new();
        assets.insert(args.asset0, MockERC20::new(args.asset0, &provider));
        assets.insert(args.asset1, MockERC20::new(args.asset1, &provider));
        assets
    };

    let mut rng = thread_rng();
    let accounts = provider.get_accounts().await?;
    let mut user_orders: HashMap<&Address, Vec<Order>> = HashMap::new();

    for order in orders {
        let account = accounts.choose(&mut rng).unwrap();
        user_orders.entry(account).or_default().push(order);
    }

    println!("hello");

    let chain_id = provider.get_chain_id().await?;
    let domain = eip712_domain! {
        name: "Angstrom",
        version: "v1",
        chain_id: chain_id,
        verifying_contract: args.angstrom,
    };

    println!("getting orders");

    for (account, orders) in user_orders {
        let (total_out0, total_out1) = orders
            .iter()
            .map(|order| {
                if order.is_bid {
                    (order.get_amount_in(), U256::ZERO)
                } else {
                    (U256::ZERO, order.get_amount_in())
                }
            })
            .fold((U256::ZERO, U256::ZERO), |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));
        println!("total_out0: {}", total_out0);
        println!("total_out1: {}", total_out1);

        ensure_tokens(*account, args.angstrom, &assets[&args.asset0], total_out0).await?;
        ensure_tokens(*account, args.angstrom, &assets[&args.asset1], total_out1).await?;

        let mut nonces = NonceGetter::new(*account, args.angstrom, &provider);

        let wallet = address_to_wallet(account)?;

        println!("wallet: {:?}", wallet);

        for order in orders {
            println!("order: {:?}", order);
            let base_order = StandingOrder {
                mode:                 OrderMode::Partial,
                max_amount_in_or_out: order.get_amount_in(),
                min_price:            order.get_min_price(),
                asset_in:             order.get_asset_in(args.asset0, args.asset1),
                asset_in_form:        AssetForm::Liquid,
                asset_out:            order.get_asset_out(args.asset0, args.asset1),
                asset_out_form:       AssetForm::Liquid,
                recipient:            *account,
                hook_data:            None,
                nonce:                nonces.next().await?,
                deadline:             U256::MAX
            };
            println!("base_order: {:?}", base_order);

            let sol_order: SolStandingOrder = base_order.into();
            println!("sol_order: {:?}", sol_order);
            let sign_hash = sol_order.eip712_signing_hash(&domain);
            let (signature, _) = wallet.signer().sign_recoverable(sign_hash.as_slice())?;
            let signature: Bytes = signature.to_vec().into();
        }
    }

    // use angstrom_rpc::api::OrderApiClient;
    // use jsonrpsee::http_client::HttpClientBuilder;
    // let angstrom_client =
    // HttpClientBuilder::default().build(args.testnet_rpc_url)?;
    // angstrom_client
    //     .submit_limit_order(Default::default())
    //     .await?;

    Ok(())
}
