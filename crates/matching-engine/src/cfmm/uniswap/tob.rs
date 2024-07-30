use std::collections::HashMap;

use alloy_primitives::U256;
use angstrom_types::sol_bindings::{grouped_orders::OrderWithStorageData, sol::TopOfBlockOrder};

use super::{math::sqrt_price_at_tick, MarketSnapshot, Tick};

pub fn calculate_reward(
    tob: OrderWithStorageData<TopOfBlockOrder>,
    amm: MarketSnapshot
) -> HashMap<Tick, U256> {
    // Determine whether we're buying from or selling to the AMM
    let tick_motion = if tob.is_bid { 1 } else { -1 };
    let current_tick = amm.current_tick;
    let next_tick = current_tick + tick_motion;

    let current_price = amm.sqrt_price_x96;
    let next_price = sqrt_price_at_tick(next_tick).unwrap();

    let mut output = tob.order.amountOut;
    // Determine how many ticks we pass across
    // Distribute our bribe to the ticks
    HashMap::new()
}
