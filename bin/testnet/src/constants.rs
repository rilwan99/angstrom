use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller
        },
        Identity, RootProvider
    },
    pubsub::PubSubFrontend
};
use angstrom_types::sol_bindings::testnet::TestnetHub::TestnetHubInstance;

pub const CACHE_VALIDATION_SIZE: usize = 100_000_000;

pub type StromContractInstance = TestnetHubInstance<
    PubSubFrontend,
    FillProvider<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>
            >,
            WalletFiller<EthereumWallet>
        >,
        RootProvider<PubSubFrontend>,
        PubSubFrontend,
        Ethereum
    >
>;
