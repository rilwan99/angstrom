use alloy::{primitives::Signed, sol};
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
}

#[derive(Debug, PadeEncode)]
enum RewardsUpdate {
    MultiTick { start_tick: Signed<24, 1>, start_liquidity: u128, quantities: Vec<u128> },
    CurrentOnly { amount: u128 }
}

#[derive(Debug, PadeEncode)]
struct PoolUpdate {
    zero_for_one:     bool,
    pair_index:       u16,
    swap_in_quantity: u128,
    rewards_update:   RewardsUpdate
}

#[derive(PadeEncode, Debug)]
struct MockContractMessage {
    assets: Vec<Asset>,
    pairs:  Vec<Pair>,
    update: PoolUpdate
}
