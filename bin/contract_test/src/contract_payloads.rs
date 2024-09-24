use alloy::sol;
use pade::derive::PadeEncode;

sol! {
    type PoolId is bytes32;

    #[derive(PadeEncode, Debug)]
    struct Asset {
        address addr;
        uint128 borrow;
        uint128 save;
        uint128 settle;
    }

    #[derive(PadeEncode, Debug)]
    struct Pair {
        uint16 index0;
        uint16 index1;
        uint16 store_index;
        uint256 price_1over0;
    }

    #[derive(Debug, Default, PadeEncode)]
    struct RewardsUpdate {
        bool onlyCurrent;
        uint128 onlyCurrentQuantity;
        int24 startTick;
        uint128 startLiquidity;
        uint128[] quantities;
    }

    #[derive(PadeEncode, Debug)]
    struct PoolUpdate {
        bool zero_for_one;
        uint16 pair_index;
        uint128 swap_in_quantity;
        RewardsUpdate rewards_update;
    }

    #[derive(PadeEncode, Debug)]
    struct MockContractMessage {
        Asset[] assetList;
        Pair[] pairList;
        PoolUpdate update;
    }
}
