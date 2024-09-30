use alloy::contract::CallBuilder;
use eyre::eyre;

pub mod anvil;
pub mod deploy;
mod reward;
pub use reward::RewardTestEnv;

pub trait DebugTransaction {
    async fn run_safe(self) -> eyre::Result<()>;
}

impl<T, P, D> DebugTransaction for CallBuilder<T, P, D>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    P: alloy::providers::Provider<T>,
    D: alloy::contract::CallDecoder
{
    async fn run_safe(self) -> eyre::Result<()> {
        let receipt = self
            .gas(30_000_000_u128)
            .send()
            .await?
            .get_receipt()
            .await?;
        if receipt.inner.status() {
            Ok(())
        } else {
            Err(eyre!("Transaction with hash {} failed", receipt.transaction_hash))
        }
    }
}
