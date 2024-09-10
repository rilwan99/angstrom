use alloy::{
    primitives::{Address, Bytes},
    sol
};
use pade_macro::PadeEncode;
use serde::{Deserialize, Serialize};

use crate::{
    contract_payloads::tob::{Asset, RewardsUpdate},
    sol_bindings::sol::TopOfBlockOrder as InternalTopOfBlockOrder
};

// This currently exists in types::sol_bindings as well, but that one is
// outdated so I'm building a new one here for now and then migrating
#[derive(PadeEncode, Clone, Default, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopOfBlockOrder {
    pub use_internal:    bool,
    pub quantity_in:     u128,
    pub quantity_out:    u128,
    pub asset_in_index:  u16,
    pub asset_out_index: u16,
    pub recipient:       Option<Address>,
    pub hook_data:       Option<Bytes>,
    pub signature:       Bytes
}

impl TopOfBlockOrder {
    pub fn of(internal: &InternalTopOfBlockOrder) -> Self {
        let quantity_in = u128::from_le_bytes(internal.amountIn.to_le_bytes());
        let quantity_out = u128::from_le_bytes(internal.amountOut.to_le_bytes());
        let asset_in_index = internal.assetInIndex;
        let asset_out_index = internal.assetOutIndex;
        let recipient = Some(internal.recipient);
        let hook_data = Some(internal.hookPayload.clone());
        let signature = internal.signature.clone();
        Self {
            use_internal: false,
            quantity_in,
            quantity_out,
            asset_in_index,
            asset_out_index,
            recipient,
            hook_data,
            signature
        }
    }
}

#[derive(PadeEncode)]
pub struct StandingValidation {
    nonce:    u64,
    // 40 bits wide in reality
    #[pade_width(5)]
    deadline: u64
}

#[derive(PadeEncode)]
pub enum OrderQuantities {
    Exact { quantity: u128 },
    Partial { min_quantity_in: u128, max_quantity_in: u128, filled_quantity: u128 }
}

#[derive(PadeEncode)]
pub struct UserOrder {
    pub use_internal:        bool,
    pub pair_index:          u16,
    pub min_price:           alloy::primitives::U256,
    pub recipient:           Option<Address>,
    pub hook_data:           Option<Bytes>,
    pub a_to_b:              bool,
    pub standing_validation: Option<StandingValidation>,
    pub order_quantities:    OrderQuantities,
    pub exact_in:            bool,
    pub signature:           Bytes
}

sol! {
    #[derive(PadeEncode)]
    struct Pair {
        uint16 t1_idx;
        uint16 t0_idx;
        uint256 uniswap_price;
    }

    #[derive(PadeEncode)]
    struct PoolUpdate {
        uint16 asset_in_index;
        uint16 asset_out_index;
        uint128 swap_in_quantity;
        RewardsUpdate rewards_update;
    }
}

#[derive(PadeEncode)]
pub struct AngstromBundle {
    assets:              Vec<Asset>,
    pairs:               Vec<Pair>,
    pool_updates:        Vec<PoolUpdate>,
    top_of_block_orders: Vec<TopOfBlockOrder>,
    user_orders:         Vec<UserOrder>
}

impl AngstromBundle {
    pub fn new(
        assets: Vec<Asset>,
        pairs: Vec<Pair>,
        pool_updates: Vec<PoolUpdate>,
        top_of_block_orders: Vec<TopOfBlockOrder>,
        user_orders: Vec<UserOrder>
    ) -> Self {
        Self { assets, pairs, pool_updates, top_of_block_orders, user_orders }
    }
}
