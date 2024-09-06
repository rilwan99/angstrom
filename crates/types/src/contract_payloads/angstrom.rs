use alloy::sol;
use alloy::primitives::{Address, Bytes, Signature};
use pade_macro::PadeEncode;
use bincode::{Decode, Encode};

use crate::contract_payloads::tob::{Asset, PoolRewardsUpdate};

// This currently exists in types::sol_bindings as well, but that one is outdated so I'm building a new one here for now and then migrating
#[derive(PadeEncode, Clone, Default, Debug, Hash, PartialEq, Eq, Encode, Decode)]
pub struct TopOfBlockOrder {
    pub use_internal: bool,
    pub quantity_in: u128,
    pub quantity_out: u128,
    pub asset_in_index: u16,
    pub asset_out_index: u16,
    #[bincode(with_serde)]
    pub recipient: Option<Address>,
    #[bincode(with_serde)]
    pub hook_data: Option<Bytes>,
    #[bincode(with_serde)]
    pub signature: Bytes
}

#[derive(PadeEncode)]
pub struct StandingValidation {
    nonce: u64,
    // 40 bits wide in reality
    #[pade_width(5)]
    deadline: u64
}

#[derive(PadeEncode)]
pub enum OrderQuantities {
    Exact { quantity: u128 },
    Partial {
        min_quantity_in: u128,
        max_quantity_in: u128,
        filled_quatity: u128
    }
}

#[derive(PadeEncode)]
pub struct UserOrder {
    use_internal: bool,
    pair_index: u16,
    min_price: alloy::primitives::U256,
    recipient: Option<Address>,
    hook_data: Option<Bytes>,
    a_to_b: bool,
    standing_validation: Option<StandingValidation>,
    order_quantities: OrderQuantities,
    exact_in: bool,
    signature: Signature
}

sol! {
    #[derive(PadeEncode)]
    struct Pair {
        uint16 index_a;
        uint16 index_b;
        uint256 price_AOverB;
    }

    #[derive(PadeEncode)]
    struct PoolSwap {
        uint16 asset_in_index;
        uint16 asset_out_index;
        uint128 quantity_in;
    }
}

#[derive(PadeEncode)]
pub struct AngstromBundle {
    assets: Vec<Asset>,
    pairs: Vec<Pair>,
    swaps: Vec<PoolSwap>,
    tob_orders: Vec<TopOfBlockOrder>,
    user_orders: Vec<UserOrder>,
    pool_rewards: Vec<PoolRewardsUpdate>
}

impl AngstromBundle {
    pub fn new(assets: Vec<Asset>, pairs: Vec<Pair>, swaps: Vec<PoolSwap>, tob_orders: Vec<TopOfBlockOrder>, user_orders: Vec<UserOrder>, pool_rewards: Vec<PoolRewardsUpdate>) -> Self {
        Self {
            assets, pairs, swaps, tob_orders, user_orders, pool_rewards
        }
    }
}