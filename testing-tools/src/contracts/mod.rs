use alloy::{
    contract::CallBuilder,
    providers::ext::DebugApi,
    rpc::types::trace::geth::{
        GethDebugBuiltInTracerType, GethDebugTracerType, GethDebugTracingOptions,
        GethDefaultTracingOptions
    }
};
use eyre::eyre;

pub mod anvil;
pub mod deploy;
pub mod environment;
//mod reward;
//pub use reward::RewardTestEnv;

/// This trait is used to provide safe run and potentially debug capabilities
/// for our local contract runs.
pub trait DebugTransaction {
    #[allow(async_fn_in_trait)] // OK because this is not for public consumption
    async fn run_safe(self) -> eyre::Result<()>;
}

impl<T, P, D> DebugTransaction for CallBuilder<T, P, D>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    P: alloy::providers::Provider<T> + Clone,
    D: alloy::contract::CallDecoder
{
    async fn run_safe(self) -> eyre::Result<()> {
        let provider = self.provider.clone();
        let receipt = self
            .gas(30_000_000_u128)
            .send()
            .await?
            .get_receipt()
            .await?;
        if receipt.inner.status() {
            Ok(())
        } else {
            let default_options = GethDebugTracingOptions::default();
            let call_options = GethDebugTracingOptions {
                config: GethDefaultTracingOptions {
                    disable_storage: Some(true),
                    enable_memory: Some(false),
                    ..Default::default()
                },
                tracer: Some(GethDebugTracerType::BuiltInTracer(
                    GethDebugBuiltInTracerType::CallTracer
                )),
                ..Default::default()
            };
            let result = provider
                .debug_trace_transaction(receipt.transaction_hash, call_options)
                .await?;
            println!("TRACE: {result:?}");
            // We can make this do a cool backtrace later
            Err(eyre!("Transaction with hash {} failed", receipt.transaction_hash))
        }
    }
}
