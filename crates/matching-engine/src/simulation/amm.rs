use angstrom_types::matching::SqrtPriceX96;
use uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick;

use crate::cfmm::uniswap::{MarketSnapshot, PoolRange};

/// Create a simple AMM with a single position with fixed liquidity across the
/// range of ticks from `middle_tick - width` to `middle_tick + width`
pub fn single_position_amm(
    middle_tick: i32,
    width: i32,
    liquidity: u128
) -> Option<MarketSnapshot> {
    let amm_price = SqrtPriceX96::from(get_sqrt_ratio_at_tick(middle_tick + 1).unwrap());
    let lower_tick = middle_tick - width;
    let upper_tick = middle_tick + width;
    let ranges = vec![PoolRange::new(lower_tick, upper_tick, liquidity).unwrap()];
    MarketSnapshot::new(ranges, amm_price).ok()
}
