use alloy::sol;
use pade_macro::PadeEncode;

sol! {
    #[derive(PadeEncode)]
    struct Asset {
        address addr;
        uint128 borrow;
        uint128 save;
        uint128 settle;
    }

    #[derive(PadeEncode)]
    struct RewardsUpdate {
        #[pade_width(3)]
        int24 startTick;
        uint128 startLiquidity;
        uint128[] quantities;
    }

    #[derive(PadeEncode)]
    struct PoolRewardsUpdate {
        uint16 asset0;
        uint16 asset1;
        RewardsUpdate update;
    }

    #[derive(PadeEncode)]
    struct MockContractMessage {
        Asset[] addressList;
        PoolRewardsUpdate update;
    }
}
