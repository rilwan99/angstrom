//! CLI definition and entrypoint to executable

use clap::Parser;
use guard_rpc::{
    api::{ConsensusApiServer, OrderApiServer, QuotingApiServer},
    ConsensusApi, OrderApi, QuotesApi
};
use reth::cli::{
    components::{RethNodeComponents, RethRpcComponents},
    config::{RethNetworkConfig, RethRpcConfig},
    ext::{RethCliExt, RethNodeCommandConfig},
    Cli
};
/// Convenience function for parsing CLI options, set up logging and run the
/// chosen command.
#[inline]
pub fn run() -> eyre::Result<()> {
    Cli::<StromRethExt>::parse()
        .with_node_extension(StaleGuardConfig::default())
        .run()
}

struct StromRethExt;

impl RethCliExt for StromRethExt {
    type Node = StaleGuardConfig;
}

#[derive(Debug, Clone, Copy, Default, clap::Args)]
struct StaleGuardConfig {
    #[clap(long)]
    pub mev_guard: bool
}

impl RethNodeCommandConfig for StaleGuardConfig {
    fn configure_network<Conf, Reth>(
        &mut self,
        _config: &mut Conf,
        _components: &Reth
    ) -> eyre::Result<()>
    where
        Conf: RethNetworkConfig,
        Reth: RethNodeComponents
    {
        //config.add_rlpx_sub_protocol();
        todo!()
    }

    #[allow(dead_code)]
    fn on_components_initialized<Reth: RethNodeComponents>(
        &mut self,
        _components: &Reth
    ) -> eyre::Result<()> {
        // Initialize the eth interacting modules, aka pool upkeep + consensus
        todo!()
    }

    fn extend_rpc_modules<Conf, Reth>(
        &mut self,
        _config: &Conf,
        _components: &Reth,
        rpc_components: RethRpcComponents<'_, Reth>
    ) -> eyre::Result<()>
    where
        Conf: RethRpcConfig,
        Reth: RethNodeComponents
    {
        //TODO: Add the handle to the order pool & consensus module
        let pool = rpc_components.registry.pool();
        let consensus = _components.network();

        let order_api = OrderApi { pool: pool.clone() };
        let quotes_api = QuotesApi { pool: pool.clone() };
        let consensus_api = ConsensusApi { consensus };
        rpc_components
            .modules
            .merge_configured(order_api.into_rpc())?;
        rpc_components
            .modules
            .merge_configured(quotes_api.into_rpc())?;
        rpc_components
            .modules
            .merge_configured(consensus_api.into_rpc())?;

        //_components.task_executor().spawn_critical();

        Ok(())
    }
}
