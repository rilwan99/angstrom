use matching_engine::cfmm::uniswap::{math::sqrt_price_at_tick, MarketSnapshot, PoolRange};

pub fn generate_single_position_amm_at_tick(
    mid: i32,
    width: i32,
    liquidity: u128
) -> MarketSnapshot {
    let amm_price = sqrt_price_at_tick(mid + 1).unwrap();
    let lower_tick = mid - width;
    let upper_tick = mid + width;
    let ranges = vec![PoolRange::new(lower_tick, upper_tick, liquidity).unwrap()];
    MarketSnapshot::new(ranges, amm_price).unwrap()
}

pub fn generate_amm_market(target_tick: i32) -> MarketSnapshot {
    let range = PoolRange::new(target_tick - 100, target_tick + 100, 100_000_000).unwrap();
    let ranges = vec![range];
    let sqrt_price_x96 = sqrt_price_at_tick(target_tick).unwrap();
    MarketSnapshot::new(ranges, sqrt_price_x96).unwrap()
}
