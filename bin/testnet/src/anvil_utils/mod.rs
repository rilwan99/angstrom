pub mod anvil_manager;
use alloy::contract::CallBuilder;
pub use anvil_manager::*;
pub mod anvil_eth_data_cleanser;
pub use anvil_eth_data_cleanser::*;

pub async fn run_transaction<T, P, D>(call: CallBuilder<T, P, D>) -> eyre::Result<()>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    P: alloy::providers::Provider<T>,
    D: alloy::contract::CallDecoder
{
    call.gas(30_000_000_u128)
        .send()
        .await?
        .get_receipt()
        .await?
        .inner
        .status();
    Ok(())
}
