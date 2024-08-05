use std::collections::HashMap;

use alloy_primitives::U256;
use angstrom_types::{
    matching::SqrtPriceX96,
    sol_bindings::{grouped_orders::OrderWithStorageData, sol::TopOfBlockOrder}
};
use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

use super::{MarketSnapshot, Tick};

pub fn calculate_reward(
    tob: OrderWithStorageData<TopOfBlockOrder>,
    amm: MarketSnapshot
) -> HashMap<Tick, U256> {
    // Determine whether we're buying from or selling to the AMM
    let tick_motion = if tob.is_bid { 1 } else { -1 };
    let current_tick = amm.current_tick;
    let next_tick = current_tick + tick_motion;

    let current_price = amm.sqrt_price_x96;
    let next_price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(next_tick).unwrap());

    let mut output = tob.order.amountOut;

    while output > U256::ZERO {
        output -= U256::from(1);
    }
    // Determine how many ticks we pass across
    // Distribute our bribe to the ticks
    HashMap::new()
}
