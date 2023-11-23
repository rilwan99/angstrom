//! CLI definition and entrypoint to executable

use clap::{value_parser, ArgAction, Args, Parser, Subcommand, ValueEnum};
use eyre::Result;
use guard_rpc::{
    api::{ConsensusApiServer, OrderApiServer, QuotingApiServer},
    ConsensusApi, OrderApi, QuotesApi
};
use order_pool::OrderPool;
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
    Cli::<StaleGuardCliExt>::parse().run()
}

struct StaleGuardCliExt;

impl RethCliExt for StaleGuardCliExt {
    type Node = StaleGuardConfig;
}

#[derive(Debug, Clone, Copy, Default, clap::Args)]
struct StaleGuardConfig {
    #[clap(long)]
    pub mev_guard: bool
}

impl RethNodeCommandConfig for StaleGuardConfig {
    fn extend_rpc_modules<Conf, Reth>(
        &mut self,
        _config: &Conf,
        _components: &Reth,
        mut rpc_components: RethRpcComponents<'_, Reth>
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

        Ok(())
    }

    fn configure_network<Conf, Reth>(
        &mut self,
        config: &mut Conf,
        components: &Reth
    ) -> eyre::Result<()>
    where
        Conf: RethNetworkConfig,
        Reth: RethNodeComponents
    {
        //config.add_rlpx_sub_protocol();
        todo!()
    }
}
